use std::io::prelude::*;
use std::process::Command;
use std::process::Stdio;
use x11rb::protocol::randr::get_monitors;

use std::fmt;

pub struct MonitorMetadata {
    name: u32,
    primary: bool,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    width_in_millimeters: u32,
    height_in_millimeters: u32,
    dpi: f64,
}

/// Implementation of the default formatter for `MonitorMetadata` for debug
/// purposes.
impl fmt::Display for MonitorMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{
                name: {}
                primary: {}
                x: {}
                y: {}
                width: {}
                height: {}
                width_in_millimeters: {}
                height_in_millimeters: {}
                dpi: {}
            }}",
            self.name,
            self.primary,
            self.x,
            self.y,
            self.width,
            self.height,
            self.width_in_millimeters,
            self.height_in_millimeters,
            self.dpi
        )
    }
}

/// Calculate the DPI (Dot Per Inch) value of the screen.
fn calculate_dpi(length_in_pixel: f64, length_in_mm: f64) -> f64 {
    return length_in_pixel / mm_to_inch(length_in_mm);
}

/// Convert millimeter to inch
fn mm_to_inch(x: f64) -> f64 {
    return x / 25.4;
}

/// Query information about the operating system.
pub fn get_os_metadata() -> os_info::Info{
  os_info::get()
}
/// Query information about the monitors which are being used.
/// The `MonitorInfo` struct contains various information about the monitors
/// including the pixel dimensions, physical dimensions, layout and more.
pub fn get_monitor_metadata(
    conn: &x11rb::rust_connection::RustConnection,
    screen: &x11rb::protocol::xproto::Screen,
) -> Vec<MonitorMetadata> {
    match get_monitors(conn, screen.root, true) {
        Ok(cookie) => match cookie.reply() {
            Ok(reply) => {
                let mut result = Vec::new();
                for monitor in reply.monitors {
                    let monitor_metadata = MonitorMetadata {
                        name: monitor.name,
                        primary: monitor.primary,
                        x: monitor.x,
                        y: monitor.y,
                        width: monitor.width,
                        height: monitor.height,
                        width_in_millimeters: monitor.width_in_millimeters,
                        height_in_millimeters: monitor.height_in_millimeters,
                        dpi: calculate_dpi(
                            monitor.width as f64,
                            monitor.width_in_millimeters as f64,
                        ),
                    };
                    result.push(monitor_metadata);
                }
                return result;
            }
            Err(error) => {
                println!("Could not get reply from the server: {}", error);
                return vec![];
            }
        },
        Err(error) => {
            println!("Could not get monitor info: {}", error);
            return vec![];
        }
    }
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
pub fn get_input_device_metadata() -> String {
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
