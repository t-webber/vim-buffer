use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::Buffer;

#[test]
fn only_one_history_entry() {
    let mut buf = Buffer::from("abcabc");
    buf.update(&Event::Key(KeyEvent::from(KeyCode::Char('f'))));
    assert_eq!(buf.history.as_vec(), &[Box::from("abcabc")]);
    buf.update(&Event::Key(KeyEvent::from(KeyCode::Char('c'))));
    assert_eq!(buf.history.as_vec(), &[Box::from("abcabc")]);
}
