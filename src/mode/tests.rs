use crossterm::event::{KeyCode, KeyModifiers};

use crate::action::{Action, GoToAction};
use crate::mode::{HandleKeyPress, Mode};

fn expect_action(
    mode: Mode,
    (code, modifiers): (KeyCode, KeyModifiers),
    action: &[Action],
) {
    let real_actions = mode.handle_key_press(code, modifiers);

    assert_eq!(real_actions, action);
}

fn code_event(code: KeyCode) -> (KeyCode, KeyModifiers) {
    event(code, None)
}

fn event(
    code: KeyCode,
    modifiers: Option<KeyModifiers>,
) -> (KeyCode, KeyModifiers) {
    (code, modifiers.unwrap_or(KeyModifiers::empty()))
}

fn test_insert_char(ch: char) {
    let event = code_event(KeyCode::Char(ch));
    expect_action(Mode::Insert, event, &[Action::InsertChar(ch)]);
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
    expect_action(Mode::Insert, event, &[
        Action::GoTo(GoToAction::Left),
        Action::SelectMode(Mode::Normal),
    ]);
}

#[test]
fn insert() {
    let event = code_event(KeyCode::Char('i'));
    expect_action(Mode::Normal, event, &[Action::SelectMode(Mode::Insert)]);
}

#[test]
fn unsupported_key() {
    let event = code_event(KeyCode::Down);
    expect_action(Mode::Insert, event, &[]);
    expect_action(Mode::Normal, event, &[]);
}

#[test]
fn wrong_mode_key() {
    expect_action(Mode::Normal, code_event(KeyCode::Char('g')), &[]);
    expect_action(Mode::Normal, code_event(KeyCode::Esc), &[]);
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
        let event = event(KeyCode::Char('i'), Some(modifier));
        expect_action(Mode::Normal, event, &[]);
        expect_action(Mode::Insert, event, &[]);
    }
}
