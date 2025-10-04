use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::Buffer;

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
    assert!(buffer.update(&Event::Key(KeyEvent::from(KeyCode::Esc))));
    for ch in "someotherchars".chars() {
        assert!(!buffer.update(&char_event(ch)));
    }
    assert!(buffer.update(&char_event('i')));
    for ch in " World".chars() {
        assert!(buffer.update(&char_event(ch)));
    }
    assert!(buffer.as_content() == "Hello World");
}
