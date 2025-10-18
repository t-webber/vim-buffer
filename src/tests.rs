use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

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
        $buffer.update(&evt!($name))
    };
    ($buffer:ident, $name:literal) => {
        $buffer.update(&evt!($name))
    };
    ($buffer:ident, $name:expr) => {
        $buffer.update(&evt!($name))
    };
}

fn cap(cap: char) -> Event {
    Event::Key(KeyEvent::new_with_kind(
        KeyCode::Char(cap),
        KeyModifiers::SHIFT,
        KeyEventKind::Press,
    ))
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
    assert!(do_evt!(buffer, Backspace));
    assert!(do_evt!(buffer, 'a'));
    assert!(do_evt!(buffer, 'b'));
    assert!(do_evt!(buffer, Esc));
    assert!(do_evt!(buffer, 'i'));
    assert!(do_evt!(buffer, Backspace));
    assert_eq!(buffer.as_content(), "b");
    assert_eq!(buffer.as_cursor(), 0);
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
        assert!(buffer.update(event), "{event:?}");
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

#[test]
fn insert_cap_i() {
    test_events(
        &[
            evt!('i'),
            evt!(' '),
            evt!('a'),
            evt!('b'),
            evt!(Esc),
            cap('I'),
            evt!('c'),
        ],
        " cab",
    );
}

#[test]
fn insert_cap_a() {
    test_events(
        &[
            evt!('i'),
            evt!(' '),
            evt!('a'),
            evt!('b'),
            evt!(Esc),
            evt!('i'),
            evt!(Esc),
            evt!('i'),
            evt!('c'),
            evt!(Esc),
            cap('A'),
            evt!('d'),
        ],
        " cabd",
    );
}

#[test]
fn insert_cap_i_empty_line() {
    test_events(&[evt!('i'), evt!(' '), evt!(Esc), cap('I'), evt!('c')], " c");
}
