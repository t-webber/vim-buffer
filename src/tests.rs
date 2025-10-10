use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::{Buffer, Mode};

macro_rules! evt {
    ($buffer:ident, $name:ident) => {
        evt!($buffer, KeyCode::$name)
    };
    ($buffer:ident, $name:literal) => {
        evt!($buffer, KeyCode::Char($name))
    };
    ($buffer:ident, $name:expr) => {
        $buffer.update(&Event::Key(KeyEvent::from($name)))
    };
}

#[test]
fn do_nothing() {
    let mut buffer = Buffer::default();
    assert!(!evt!(buffer, Enter));
    assert!(evt!(buffer, 'i'));
    assert!(!evt!(buffer, Enter));
}

#[test]
fn backspace() {
    let mut buffer = Buffer::default();
    assert!(evt!(buffer, 'i'));
    assert!(evt!(buffer, 'a'));
    assert!(evt!(buffer, Backspace));
    assert!(buffer.as_content().is_empty());
}

#[test]
fn mode_switch() {
    let mut buffer = Buffer::default();
    assert_eq!(buffer.as_mode(), Mode::Normal);
    assert!(evt!(buffer, 'i'));
    assert_eq!(buffer.as_mode(), Mode::Insert);
    assert!(evt!(buffer, Esc));
    assert_eq!(buffer.as_mode(), Mode::Normal);
}

#[test]
fn hello_world() {
    let mut buffer = Buffer::default();
    assert!(evt!(buffer, 'i'));
    for ch in "Hello World".chars() {
        assert!(evt!(buffer, KeyCode::Char(ch)));
    }
    assert!(buffer.as_content() == "Hello World");
}
