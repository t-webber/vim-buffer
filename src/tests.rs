use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::{Buffer, Mode};

macro_rules! evt {
    ($name:ident) => {
        evt!(KeyCode::$name)
    };
    ($name:literal) => {
        evt!(KeyCode::Char($name))
    };
    ($name:expr) => {
        Event::Key(KeyEvent::from($name))
    };
}


macro_rules! do_evt {
    ($buffer:ident, $name:ident) => {
        do_evt!($buffer, KeyCode::$name)
    };
    ($buffer:ident, $name:literal) => {
        do_evt!($buffer, KeyCode::Char($name))
    };
    ($buffer:ident, $name:expr) => {
        $buffer.update(&Event::Key(KeyEvent::from($name)))
    };
}

#[test]
fn do_nothing() {
    let mut buffer = Buffer::default();
    assert!(!do_evt!(buffer, Enter));
    assert!(do_evt!(buffer, 'i'));
    assert!(!do_evt!(buffer, Enter));
}

#[test]
fn backspace() {
    let mut buffer = Buffer::default();
    assert!(do_evt!(buffer, 'i'));
    assert!(do_evt!(buffer, 'a'));
    assert!(do_evt!(buffer, Backspace));
    assert_eq!(buffer.as_content(), "");
}

#[test]
fn chars_normal_mode() {
    let mut buffer = Buffer::default();
    for ch in "someotherchrs".chars() {
        assert!(!do_evt!(buffer, KeyCode::Char(ch)));
    }
    assert_eq!(buffer.as_content(), "");
}

#[test]
fn mode_switch() {
    let mut buffer = Buffer::default();
    assert_eq!(buffer.as_mode(), Mode::Normal);
    assert!(do_evt!(buffer, 'i'));
    assert_eq!(buffer.as_mode(), Mode::Insert);
    assert!(do_evt!(buffer, Esc));
    assert_eq!(buffer.as_mode(), Mode::Normal);
}

#[test]
fn hello_world() {
    let mut buffer = Buffer::default();
    assert!(do_evt!(buffer, 'i'));
    for ch in "Hello World".chars() {
        assert!(do_evt!(buffer, KeyCode::Char(ch)));
    }
    assert_eq!(buffer.as_content(), "Hello World");
}

fn test_events(events: &[Event], expected: &str) {
    let mut buffer = Buffer::default();
    for event in events {
        buffer.update(event);
    }
    assert_eq!(buffer.as_content(), expected);
}

#[test]
fn insert_a() {
    test_events(&[evt!('i'), evt!('a'), evt!(Esc), evt!('a'), evt!('b')], "ab");
}

#[test]
fn insert_i() {
    test_events(&[evt!('i'), evt!('a'), evt!(Esc), evt!('i'), evt!('b')], "ba");
}
