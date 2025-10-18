#![allow(clippy::restriction)]

use crossterm::event::{Event, KeyCode, KeyEvent};

fn char_event(ch: char) -> Event {
    Event::Key(KeyEvent::from(KeyCode::Char(ch)))
}

/// Transforms a string input of keymaps into a list of events.
///
/// Only chars are supported for now.
pub fn parse_events(keymaps: &str) -> Vec<Event> {
    keymaps.chars().map(char_event).collect()
}
