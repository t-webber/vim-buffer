use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::Buffer;
use crate::mode::Mode;

fn char_event(ch: char) -> Event {
    Event::Key(KeyEvent::from(KeyCode::Char(ch)))
}

#[test]
fn do_nothing() {
    let mut buffer = Buffer::default();
    assert!(!buffer.update(&Event::Key(KeyEvent::from(KeyCode::Enter))));
}

#[test]
fn hello_world() {
    let mut buffer = Buffer::default();
    assert!(buffer.update(&char_event('i')));
    for ch in "Hello".chars() {
        assert!(buffer.update(&char_event(ch)));
    }
    assert!(matches!(buffer.as_mode(), Mode::Insert));
    assert!(buffer.update(&Event::Key(KeyEvent::from(KeyCode::Esc))));
    assert!(matches!(buffer.as_mode(), Mode::Normal));
    for ch in "someotherchars".chars() {
        assert!(!buffer.update(&char_event(ch)));
    }
    assert!(matches!(buffer.as_mode(), Mode::Normal));
    assert!(buffer.update(&char_event('i')));
    assert!(matches!(buffer.as_mode(), Mode::Insert));
    for ch in " World".chars() {
        assert!(buffer.update(&char_event(ch)));
    }
    assert!(buffer.as_content() == "Hello World");
}
