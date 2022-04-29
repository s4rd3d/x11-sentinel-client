use std::io::prelude::*;
use std::process::Command;
use std::process::Stdio;

use x11rb::connection::Connection;
use x11rb::protocol::xinput::list_input_devices;
use x11rb::protocol::xinput::xi_query_version;
use x11rb::protocol::xinput::xi_select_events;
use x11rb::protocol::xinput::DeviceUse;
use x11rb::protocol::xinput::EventMask;
use x11rb::protocol::xinput::XIEventMask;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;

/// Print various metadata of the input screen.
fn print_screen_metadata(screen: &x11rb::protocol::xproto::Screen) -> () {
    println!("Screen width in pixels: {}", screen.width_in_pixels);
    println!("Screen height in pixels: {}", screen.height_in_pixels);
    println!(
        "Screen width in milimeters: {}",
        screen.width_in_millimeters
    );
    println!(
        "Screen height in milimeters: {}",
        screen.height_in_millimeters
    );
}

/// Query information about input devices with mouse capabilities.
/// The function reads the `/proc/bus/input/devices` file and uses the `grep`
/// command to filter four mouse devices.
///
/// The result of the function can be interpreted as follows:
///
/// The B in front stands for bitmap, N, P, S, U, H are simply first letter in
/// corresponding name value and I is for ID. In ordered fashion:
///
/// I => @id: id of the device
/// Bus     => id.bustype
/// Vendor  => id.vendor
/// Product => id.product
/// Version => id.version
/// N => name of the device.
/// P => physical path to the device in the system hierarchy.
/// S => sysfs path.
/// U => unique identification code for the device (if device has it).
/// H => list of input handles associated with the device.
/// B => bitmaps
/// PROP => device properties and quirks.
/// EV   => types of events supported by the device.
/// KEY  => keys/buttons this device has.
/// MSC  => miscellaneous events supported by the device.
/// LED  => leds present on the device.
fn get_input_device_metadata() -> String {
    let process1 = match Command::new("cat")
        .arg("/proc/bus/input/devices")
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(process) => process,
        Err(error) => {
            println!("Could not spawn cat: {}", error);
            return String::new();
        }
    };

    let process2 = match Command::new("grep")
        .args(["-B", "5", "-A", "5", "mouse"])
        .stdin(process1.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(process) => process,
        Err(error) => {
            println!("Could not spawn grep: {}", error);
            return String::new();
        }
    };

    let mut s = String::new();
    match process2.stdout.unwrap().read_to_string(&mut s) {
        Ok(_) => return s,
        Err(error) => {
            println!("couldn't read grep stdout: {}", error);
            return String::new();
        }
    }
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

fn main() {
    // Create connection with the X server.
    let (conn, screen_number) = match x11rb::connect(None) {
        Ok(result) => result,
        Err(error) => panic!("Could not establish connection: {:?}", error),
    };

    // Setup connection and print protocol version.
    let setup = &conn.setup();
    println!(
        "X Protocol version: {}.{}",
        setup.protocol_major_version, setup.protocol_minor_version
    );

    // Check if xinput extension version 2.0 is enabled.
    match xi_query_version(&conn, 2, 0) {
        Ok(result) => match result.reply() {
            Ok(version) => println!(
                "XInput version: {}.{}",
                version.major_version, version.minor_version
            ),
            Err(error) => panic!("Could not get reply from server: {:?}", error),
        },
        Err(error) => panic!("Could not query XInput version: {:?}", error),
    };

    // Select screen and print its metadata.
    let screen = &setup.roots[screen_number];
    print_screen_metadata(screen);

    // Print input device metadata.
    let input_device_metadata = get_input_device_metadata();
    println!(
        "Input devices with mouse capabilities: \n{}",
        &input_device_metadata
    );

    // Get input devices.
    let input_devices = match list_input_devices(&conn) {
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
    match xi_select_events(&conn, screen.root, &event_masks) {
        Ok(cookie) => match cookie.check() {
            Ok(result) => drop(result),
            Err(error) => panic!("Could not apply event masks: {:?}", error),
        },
        Err(error) => panic!("Could not connect to server: {:?}", error),
    };

    // Send pending requests to the X server.
    match conn.flush() {
        Ok(result) => drop(result),
        Err(error) => panic!("Error, flush did not succeed: {:?}", error),
    }

    // Main event loop.
    loop {
        // The program should not panic on connection error.
        let event = match conn.wait_for_event() {
            Ok(result) => result,
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
                let pointer = get_pointer(&conn, screen.root);
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
