use vim_buffer::{Buffer, ChevronGroupError, EventParsingError, Mode};

macro_rules! evt {
    ($name:ident) => {
        evt!(crossterm::event::KeyCode::$name)
    };
    ($name:literal) => {
        evt!(crossterm::event::KeyCode::Char($name))
    };
    ($name:expr) => {
        crossterm::event::Event::Key(crossterm::event::KeyEvent::from($name))
    };
}

macro_rules! do_evt {
    ($buffer:ident, $name:ident) => {
        $buffer.update(&evt!($name))
    };
    ($buffer:ident, $name:literal) => {
        $buffer.update(&evt!($name))
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
fn mode_switch() {
    let mut buffer = Buffer::default();
    assert_eq!(buffer.as_mode(), Mode::Normal);
    assert!(do_evt!(buffer, 'i'));
    assert_eq!(buffer.as_mode(), Mode::Insert);
    assert!(do_evt!(buffer, Esc));
    assert_eq!(buffer.as_mode(), Mode::Normal);
}

#[test]
fn empty_group() {
    let mut buffer = Buffer::default();
    assert_eq!(
        buffer.update_from_string("<>"),
        Err(EventParsingError::ChevronGroup(
            ChevronGroupError::ExpectedLetter { got: '>' }
        ))
    );
}

#[test]
fn from_string() {
    assert_eq!(Buffer::from("abc").as_content(), "abc");
    assert_eq!(Buffer::from(String::from("abc")).as_content(), "abc");
}

#[test]
fn default() {
    let buffer = Buffer::default();
    assert!(buffer.is_empty());
    assert_eq!(buffer.as_cursor(), 0);
    assert_eq!(buffer.as_mode(), Mode::Normal);
}

#[test]
fn sizes() {
    let mut buffer = Buffer::default();
    buffer.update_from_string("iabcdef<BS><Left>").unwrap();
    assert_eq!(buffer.len(), 5);
    assert_eq!(buffer.as_cursor(), 4);
}
