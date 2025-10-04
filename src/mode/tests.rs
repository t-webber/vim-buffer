use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::action::Action;
use crate::mode::{HandleEvent as _, Mode};


fn expect_action(mode: Mode, event: &Event, action: Action) {
    assert_eq!(Some(action), mode.handle_event(event));
}

fn expect_none(mode: Mode, event: &Event) {
    assert!(mode.handle_event(event).is_none());
}


fn code_to_event(code: KeyCode) -> Event {
    Event::Key(KeyEvent::from(code))
}

fn test_insert_char(ch: char) {
    let event = code_to_event(KeyCode::Char(ch));
    expect_action(Mode::Insert, &event, Action::InsertChar(ch));
}

#[test]
fn insert_char() {
    for code in 0..=0x0010_ffff {
        if let Some(ch) = char::from_u32(code) {
            test_insert_char(ch);
        }
    }
}

#[test]
fn escape() {
    let event = code_to_event(KeyCode::Esc);
    expect_action(Mode::Insert, &event, Action::SelectMode(Mode::Normal));
}

#[test]
fn insert() {
    let event = code_to_event(KeyCode::Char('i'));
    expect_action(Mode::Normal, &event, Action::SelectMode(Mode::Insert));
}

#[test]
fn none() {
    let event = code_to_event(KeyCode::Down);
    expect_none(Mode::Insert, &event);
    expect_none(Mode::Normal, &event);
}
