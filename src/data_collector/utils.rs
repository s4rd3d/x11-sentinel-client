/**
 * Utility functions for the data collector module.
 */
use std::env;
use std::process::Command;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use x11rb::protocol::xinput;
use x11rb::protocol::xproto;

//==============================================================================
// Enums
//==============================================================================

#[derive(Clone, Debug)]
pub enum Message {
    MetadataChangedMessage,
    X11EventMessage(
        x11rb::protocol::Event,
        x11rb::protocol::xproto::QueryPointerReply,
    ),
}

//==============================================================================
// Public functions
//==============================================================================

/// Return milliseconds since 00:00:00 UTC 1 January 1970
pub fn now() -> u64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap();
}

/// Generic function to get an environment variable and parse it to the desired
/// type.
pub fn get_env_var<T: FromStr>(name: &str) -> T {
    let var: T = match env::var(name) {
        Ok(string) => match string.parse() {
            Ok(value) => value,
            Err(_error) => panic!("Could not parse {} environment variable.", name),
        },
        Err(error) => panic!("Could not find {} environment variable: {}", name, error),
    };
    return var;
}

/// Get the output of the `who` command as the unique identifier of the current
/// user session
pub fn get_session_id() -> String {
    let output = Command::new("who")
        .output()
        .expect("Failed to execute command: who");

    // Convert result to String.
    let mut result = String::from_utf8(output.stdout).unwrap();

    // Strip whitespace.
    result.retain(|c| !c.is_whitespace());
    return result;
}

/// Get the pointer from the X server.
pub fn get_pointer(
    connection: &x11rb::rust_connection::RustConnection,
    window: u32,
) -> x11rb::protocol::xproto::QueryPointerReply {
    return match xproto::query_pointer(connection, window) {
        Ok(cookie) => match cookie.reply() {
            Ok(result) => result,
            Err(error) => panic!("Could not get reply from server: {:?}", error),
        },
        Err(error) => panic!("Cold not query pointer: {:?}", error),
    };
}

/// Setup connection to the X server and check extension availability.
pub fn setup_connection() -> (x11rb::rust_connection::RustConnection, usize) {
    // Create connection with the X server.
    let (connection, screen_number) = match x11rb::connect(None) {
        Ok(result) => result,
        Err(error) => panic!("Could not establish connection: {:?}", error),
    };

    // Check if the xinput extension is enabled
    check_xinput(&connection);

    (connection, screen_number)
}

/// Create event masks for the desired xinput events and apply them to the root
/// window.
pub fn select_events(
    connection: &x11rb::rust_connection::RustConnection,
    screen: &x11rb::protocol::xproto::Screen,
) -> () {
    // Get input devices.
    let input_devices = match xinput::list_input_devices(connection) {
        Ok(result) => result.reply().unwrap().devices,
        Err(error) => panic!("Could not get input devices: {:?}", error),
    };

    // Create an event mask for master pointer devices.
    let mut event_masks: Vec<xinput::EventMask> = Vec::new();
    for device in input_devices {
        match device.device_use {
            xinput::DeviceUse::IS_X_POINTER => {
                event_masks.push(xinput::EventMask {
                    deviceid: device.device_id.into(),
                    mask: vec![(xinput::XIEventMask::RAW_MOTION
                        | xinput::XIEventMask::RAW_TOUCH_BEGIN
                        | xinput::XIEventMask::RAW_TOUCH_UPDATE
                        | xinput::XIEventMask::RAW_TOUCH_END
                        | xinput::XIEventMask::RAW_BUTTON_PRESS
                        | xinput::XIEventMask::RAW_BUTTON_RELEASE)
                        .into()],
                });
            }
            _ => continue,
        };
    }

    // Apply event masks.
    match xinput::xi_select_events(connection, screen.root, &event_masks) {
        Ok(cookie) => match cookie.check() {
            Ok(result) => drop(result),
            Err(error) => panic!("Could not apply event masks: {:?}", error),
        },
        Err(error) => panic!("Could not connect to server: {:?}", error),
    };
}

//==============================================================================
// Internal functions
//==============================================================================

// Check if xinput extension version 2.4 is enabled.
fn check_xinput(connection: &x11rb::rust_connection::RustConnection) -> () {
    let major = 2;
    let minor = 4;
    match xinput::xi_query_version(connection, major, minor) {
        Ok(result) => match result.reply() {
            Ok(_version) => (),
            Err(error) => panic!("Could not get reply from server: {:?}", error),
        },
        Err(error) => panic!("Could not query XInput version: {:?}", error),
    };
}
