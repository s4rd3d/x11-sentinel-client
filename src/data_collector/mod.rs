/**
 * This module implements the main data collection logic.
 */
use std::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use serde_json::json;

use x11rb::connection::Connection;
use x11rb::protocol::Event;

use uuid::Uuid;

use crate::status::Status;

mod metadata;
mod utils;

//==============================================================================
// Structs
//==============================================================================

struct State {
    buffer: Vec<EventType>,
    buffer_size_limit: usize,
    epoch: u64,
    session_id: String,
    stream_id: String,
    sequence_number: u64,
}

impl State {
    /// Constructor for the State object.
    fn new() -> State {
        // Initialize empty buffer
        let buffer = vec![];

        // Get buffer size limit from the environment.
        let buffer_size_limit: usize = utils::get_env_var("BUFFER_SIZE_LIMIT");

        // Milliseconds since 00:00:00 UTC 1 January 1970
        let epoch: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap();

        // Unique identifier of the current user session.
        let session_id = utils::get_session_id();

        // Generate unique stream identifier.
        let stream_id = Uuid::new_v4().to_string();

        // Sequence number for chunk submissions starting at 0.
        let sequence_number = 0;

        State {
            buffer,
            buffer_size_limit,
            epoch,
            session_id,
            stream_id,
            sequence_number
        }
    }

    /// Event handler for `MotionEvent`.
    fn handle_raw_motion_event(
        &mut self,
        event: x11rb::protocol::xinput::RawMotionEvent,
        pointer: x11rb::protocol::xproto::QueryPointerReply,
    ) -> () {
        self.push(EventType::MotionEvent(
            0,
            event.axisvalues_raw[0].integral,
            event.axisvalues_raw[0].frac,
            event.axisvalues_raw[1].integral,
            event.axisvalues_raw[1].frac,
            pointer.root_x,
            pointer.root_y,
            event.time,
        ));
    }

    /// Event handler for `ScrollEvent`.
    fn handle_scroll_event(
        &mut self,
        event: x11rb::protocol::xinput::RawMotionEvent,
        pointer: x11rb::protocol::xproto::QueryPointerReply,
    ) -> () {
        self.push(EventType::ScrollEvent(
            1,
            event.axisvalues_raw[0].integral,
            event.axisvalues_raw[0].frac,
            pointer.root_x,
            pointer.root_y,
            event.time,
        ));
    }

    /// Event handler for `TouchBeginEvent`.
    fn handle_touch_begin_event(
        &mut self,
        event: x11rb::protocol::xinput::RawTouchBeginEvent,
        pointer: x11rb::protocol::xproto::QueryPointerReply,
    ) -> () {
        self.push(EventType::TouchBeginEvent(
            2,
            event.axisvalues_raw[0].integral,
            event.axisvalues_raw[0].frac,
            event.axisvalues_raw[1].integral,
            event.axisvalues_raw[1].frac,
            pointer.root_x,
            pointer.root_y,
            event.time,
        ));
    }

    /// Event handler for `TouchUpdateEvent`.
    fn handle_touch_update_event(
        &mut self,
        event: x11rb::protocol::xinput::RawTouchUpdateEvent,
        pointer: x11rb::protocol::xproto::QueryPointerReply,
    ) -> () {
        self.push(EventType::TouchUpdateEvent(
            3,
            event.axisvalues_raw[0].integral,
            event.axisvalues_raw[0].frac,
            event.axisvalues_raw[1].integral,
            event.axisvalues_raw[1].frac,
            pointer.root_x,
            pointer.root_y,
            event.time,
        ));
    }

    /// Event handler for `TouchUpdateEvent`.
    fn handle_touch_end_event(
        &mut self,
        event: x11rb::protocol::xinput::RawTouchEndEvent,
        pointer: x11rb::protocol::xproto::QueryPointerReply,
    ) -> () {
        self.push(EventType::TouchEndEvent(
            4,
            event.axisvalues_raw[0].integral,
            event.axisvalues_raw[0].frac,
            event.axisvalues_raw[1].integral,
            event.axisvalues_raw[1].frac,
            pointer.root_x,
            pointer.root_y,
            event.time,
        ));
    }

    /// Event handler for `ButtonPressEvent`.
    fn handle_button_press_event(
        &mut self,
        event: x11rb::protocol::xinput::RawButtonPressEvent,
        pointer: x11rb::protocol::xproto::QueryPointerReply,
    ) -> () {
        self.push(EventType::ButtonPressEvent(
            5,
            pointer.root_x,
            pointer.root_y,
            event.detail,
            event.time,
        ));
    }

    /// Event handler for `ButtonReleaseEvent`.
    fn handle_button_release_event(
        &mut self,
        event: x11rb::protocol::xinput::RawButtonReleaseEvent,
        pointer: x11rb::protocol::xproto::QueryPointerReply,
    ) -> () {
        self.push(EventType::ButtonReleaseEvent(
            6,
            pointer.root_x,
            pointer.root_y,
            event.detail,
            event.time,
        ));
    }

    fn push(&mut self, event: EventType) -> () {
        self.buffer.push(event);
        if self.buffer.len() > self.buffer_size_limit {
            self.submit();
        }
    }

    /// Return a deep copy of the buffer then clear its contents.
    fn flush_buffer(&mut self) -> Vec<EventType> {
        let send_buffer = self.buffer.clone();
        self.buffer.clear();
        return send_buffer;
    }

    fn submit(&mut self) -> () {
        // Retrieve data
        let send_buffer = self.flush_buffer();

        // Do not send empty buffer
        if send_buffer.len() == 0 {
            return;
        }

        let body = json!({
            "metadata": {
                "epoch": self.epoch,
                "session_id": self.session_id,
                "stream_id": self.stream_id,
                "sequence_number": self.sequence_number
            },
            "chunk": send_buffer,
        });
        println!("Buffer: {:?}", body.to_string());
    }
}

//==============================================================================
// Enums
//==============================================================================

#[derive(Copy, Clone, Debug, Serialize)]
#[serde(untagged)]
enum EventType {
    MotionEvent(u8, i32, u32, i32, u32, i16, i16, u32),
    ScrollEvent(u8, i32, u32, i16, i16, u32),
    TouchBeginEvent(u8, i32, u32, i32, u32, i16, i16, u32),
    TouchUpdateEvent(u8, i32, u32, i32, u32, i16, i16, u32),
    TouchEndEvent(u8, i32, u32, i32, u32, i16, i16, u32),
    ButtonPressEvent(u8, i16, i16, u32, u32),
    ButtonReleaseEvent(u8, i16, i16, u32, u32),
}

//==============================================================================
// Public functions
//==============================================================================

pub fn run(rx: mpsc::Receiver<Status>) -> () {
    // Setup connection to the X server.
    let (connection, screen_number) = utils::setup_connection();

    let mut state = State::new();

    // Setup connection.
    let setup = &connection.setup();

    // Select screen.
    let screen = &setup.roots[screen_number];

    // Collect platform and device specific metadata.
    metadata::query_metadata(&connection, screen);

    // Apply specific event masks to the connection.
    utils::select_events(&connection, screen);

    // Send pending requests to the X server.
    match connection.flush() {
        Ok(result) => drop(result),
        Err(error) => panic!("Error, flush did not succeed: {:?}", error),
    }

    // Main event loop.
    loop {
        // Process incoming status updates (if any).
        match rx.try_recv() {
            Ok(msg) => println!(
                "phase: {} description: {}, value: {}",
                msg.phase, msg.description, msg.value
            ),
            Err(_) => (),
        }

        // Poll for a new event, the program should not panic on connection
        // error.
        let event = match connection.wait_for_event() {
            Ok(event) => event,
            Err(error) => {
                println!("Connection error: {:?}", error);
                continue;
            }
        };

        // Get the transformed pointer coordinates too for comparison.
        let pointer = utils::get_pointer(&connection, screen.root);

        // Handle motion events.
        match event {
            Event::XinputRawMotion(event) => match event.axisvalues_raw.len() {
                1 => state.handle_scroll_event(event, pointer),
                2 => state.handle_raw_motion_event(event, pointer),
                _ => (),
            },
            Event::XinputRawTouchBegin(event) => {
                state.handle_touch_begin_event(event, pointer);
            }
            Event::XinputRawTouchUpdate(event) => {
                state.handle_touch_update_event(event, pointer);
            }
            Event::XinputRawTouchEnd(event) => {
                state.handle_touch_end_event(event, pointer);
            }
            Event::XinputRawButtonPress(event) => {
                state.handle_button_press_event(event, pointer);
            }
            Event::XinputRawButtonRelease(event) => {
                state.handle_button_release_event(event, pointer);
            }
            _ => continue,
        }
    }
}
