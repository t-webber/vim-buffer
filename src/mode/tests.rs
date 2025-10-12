use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::action::Action;
use crate::mode::{HandleEvent as _, Mode};

fn expect_action(mode: Mode, event: Event, action: Option<Action>) {
    let real_actions = mode.handle_event(&event);

    assert!(real_actions.len() <= 1);
    assert_eq!(real_actions.first(), action.as_ref());
}

fn code_event(code: KeyCode) -> Event {
    event(code, None, None)
}

fn event(
    code: KeyCode,
    modifiers: Option<KeyModifiers>,
    kind: Option<KeyEventKind>,
) -> Event {
    Event::Key(KeyEvent::new_with_kind(
        code,
        modifiers.unwrap_or(KeyModifiers::empty()),
        kind.unwrap_or(KeyEventKind::Press),
    ))
}

fn test_insert_char(ch: char) {
    let event = code_event(KeyCode::Char(ch));
    expect_action(Mode::Insert, event, Some(Action::InsertChar(ch)));
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
    let event = code_event(KeyCode::Esc);
    expect_action(Mode::Insert, event, Some(Action::SelectMode(Mode::Normal)));
}

#[test]
fn insert() {
    let event = code_event(KeyCode::Char('a'));
    expect_action(Mode::Normal, event, Some(Action::SelectMode(Mode::Insert)));
}

#[test]
fn unsupported_key() {
    let event = code_event(KeyCode::Down);
    expect_action(Mode::Insert, event, None);
    expect_action(Mode::Normal, event, None);
}

#[test]
fn wrong_mode_key() {
    expect_action(Mode::Normal, code_event(KeyCode::Char('g')), None);
    expect_action(Mode::Normal, code_event(KeyCode::Esc), None);
}

#[test]
fn not_press() {
    for kind in [KeyEventKind::Release, KeyEventKind::Repeat] {
        let event = event(KeyCode::Char('x'), None, Some(kind));
        expect_action(Mode::Insert, event, None);
    }
}

#[test]
fn with_modifiers() {
    for modifier in [
        KeyModifiers::SHIFT,
        KeyModifiers::CONTROL,
        KeyModifiers::ALT,
        KeyModifiers::SUPER,
        KeyModifiers::HYPER,
        KeyModifiers::META,
    ] {
        let event = event(KeyCode::Char('i'), Some(modifier), None);
        expect_action(Mode::Normal, event, None);
        expect_action(Mode::Insert, event, None);
    }
}
