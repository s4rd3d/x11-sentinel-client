use x11rb::connection::Connection;

fn main() {
    let (conn, _number_of_screens) = x11rb::connect(None).unwrap();
    let setup = &conn.setup();
    println!(
        "X Protocol version: {}.{}",
        setup.protocol_major_version, setup.protocol_minor_version
    );
}
