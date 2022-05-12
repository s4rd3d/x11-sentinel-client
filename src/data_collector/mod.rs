/**
 * This module implements the main data collection logic.
 */
use x11rb::connection::Connection;

use x11rb::protocol::xinput::list_input_devices;
use x11rb::protocol::xinput::xi_query_version;
use x11rb::protocol::xinput::xi_select_events;
use x11rb::protocol::xinput::DeviceUse;
use x11rb::protocol::xinput::EventMask;
use x11rb::protocol::xinput::XIEventMask;

use x11rb::protocol::xproto::*;

use std::sync::mpsc;
use x11rb::protocol::Event;

mod metadata;

fn check_xinput(connection: &x11rb::rust_connection::RustConnection) -> () {
    // Check if xinput extension version 2.0 is enabled.
    match xi_query_version(connection, 2, 0) {
        Ok(result) => match result.reply() {
            Ok(version) => println!(
                "XInput version: {}.{}",
                version.major_version, version.minor_version
            ),
            Err(error) => panic!("Could not get reply from server: {:?}", error),
        },
        Err(error) => panic!("Could not query XInput version: {:?}", error),
    };
}

/// Get the pointer from the X server.
fn get_pointer(
    connection: &x11rb::rust_connection::RustConnection,
    window: u32,
) -> x11rb::protocol::xproto::QueryPointerReply {
    return match query_pointer(connection, window) {
        Ok(cookie) => match cookie.reply() {
            Ok(result) => result,
            Err(error) => panic!("Could not get reply from server: {:?}", error),
        },
        Err(error) => panic!("Cold not query pointer: {:?}", error),
    };
}

/// Setup connection to the X server and check extension availability.
fn setup() -> (x11rb::rust_connection::RustConnection, usize) {
    // Create connection with the X server.
    let (connection, screen_number) = match x11rb::connect(None) {
        Ok(result) => result,
        Err(error) => panic!("Could not establish connection: {:?}", error),
    };

    // Check if the xinput extension is enabled
    check_xinput(&connection);

    (connection, screen_number)
}

fn query_metadata(
    connection: &x11rb::rust_connection::RustConnection,
    screen: &x11rb::protocol::xproto::Screen,
) -> () {
    // Print monitor metadata
    let monitor_metadata = metadata::get_monitor_metadata(&connection, &screen);
    for metadata in monitor_metadata {
        println!("Monitor information: \n{}", &metadata);
    }

    // Print input device metadata.
    let input_device_metadata = metadata::get_input_device_metadata();
    println!(
        "Input devices with mouse capabilities: \n{}",
        &input_device_metadata
    );

    // Print operating system metadata.
    let os_metadata = metadata::get_os_metadata();
    println!("Operating system information: {}", &os_metadata);
}

fn select_events(
    connection: &x11rb::rust_connection::RustConnection,
    screen: &x11rb::protocol::xproto::Screen,
) -> () {
    // Get input devices.
    let input_devices = match list_input_devices(connection) {
        Ok(result) => result.reply().unwrap().devices,
        Err(error) => panic!("Could not get input devices: {:?}", error),
    };

    // Create an event mask for master pointer devices.
    let mut event_masks: Vec<EventMask> = Vec::new();
    for device in input_devices {
        match device.device_use {
            DeviceUse::IS_X_POINTER => {
                event_masks.push(EventMask {
                    deviceid: device.device_id.into(),
                    mask: vec![XIEventMask::RAW_MOTION.into()],
                });
            }
            _ => continue,
        };
    }

    // Apply event masks.
    match xi_select_events(connection, screen.root, &event_masks) {
        Ok(cookie) => match cookie.check() {
            Ok(result) => drop(result),
            Err(error) => panic!("Could not apply event masks: {:?}", error),
        },
        Err(error) => panic!("Could not connect to server: {:?}", error),
    };

}

pub fn run(rx: mpsc::Receiver<&str>) -> () {
    // Setup connection to the X server
    let (connection, screen_number) = setup();

    // Setup connection and print protocol version.
    let setup = &connection.setup();
    println!(
        "X Protocol version: {}.{}",
        setup.protocol_major_version, setup.protocol_minor_version
    );

    // Select screen
    let screen = &setup.roots[screen_number];

    // Collect platform and device specific metadata.
    query_metadata(&connection, screen);

    // Apply specific event masks to the connection.
    select_events(&connection, screen);

    // Send pending requests to the X server.
    match connection.flush() {
        Ok(result) => drop(result),
        Err(error) => panic!("Error, flush did not succeed: {:?}", error),
    }

    // Main event loop.
    loop {
        // Process incoming status updates (if any).
        match rx.try_recv() {
            Ok(msg) => println!("Status: {}", msg),
            Err(_) => (),
        }

        // Poll for a new event, the program should not panic on connection
        // error.
        let event = match connection.poll_for_event() {
            Ok(result) => match result {
                Some(event) => event,
                None => continue,
            },
            Err(error) => {
                println!("Connection error: {:?}", error);
                continue;
            }
        };

        // Handle motion events.
        //
        // A RawDevice event provides the information provided by the driver to the
        // client. RawEvent provides both the raw data as supplied by the driver and
        // transformed data as used in the server. Transformations include, but are
        // not limited to, axis clipping and acceleration.
        // Transformed valuator data may be equivalent to raw data. In this case,
        // both raw and transformed valuator data is provided.
        //
        // axisvalues
        // Valuator data in device-native resolution. This is a non-sparse
        // array, value N represents the axis corresponding to the Nth bit set
        // in valuators.
        //
        // axisvalues_raw
        // Untransformed valuator data in device-native resolution. This is a
        // non-sparse array, value N represents the axis corresponding to the
        // Nth bit set in valuators.
        //
        // FP3232
        // Fixed point decimal in 32.32 format as one INT32 and one CARD32.
        // The INT32 contains the integral part, the CARD32 the decimal fraction
        // shifted by 32.
        match event {
            Event::XinputRawMotion(event) => {
                // Get the transformed pointer coordinates too for comparison
                let pointer = get_pointer(&connection, screen.root);
                println!(
                    "root_x: {} root_y: {}, raw_x: {:?}, raw_y: {:?}, sequence: {}, t: {}",
                    pointer.root_x,
                    pointer.root_y,
                    event.axisvalues_raw[0],
                    event.axisvalues_raw[1],
                    event.sequence,
                    event.time,
                );
            }
            _ => continue,
        }
    }
}
