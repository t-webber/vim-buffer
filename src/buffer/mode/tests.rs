use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::buffer::keymaps::{Action, CombinablePending, GoToAction, OPending};
use crate::buffer::mode::all::Mode;
use crate::buffer::mode::normal::Normal;
use crate::buffer::mode::{Actions, BufferMode};

const NORMAL: BufferMode = BufferMode::Normal(Normal::new());

fn expect_action(mut mode: BufferMode, event: Event, action: &[Action]) {
    assert_eq!(mode.handle_event(&event), action.to_vec().into());
}

fn expect_no_action(mut mode: BufferMode, event: Event) {
    assert_eq!(mode.handle_event(&event), Actions::Unsupported);
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
    expect_action(BufferMode::Insert, event, &[Action::InsertChar(ch)]);
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
    expect_action(BufferMode::Insert, event, &[
        GoToAction::Left.into(),
        Mode::Normal.into(),
    ]);
}

#[test]
fn insert() {
    let event = code_event(KeyCode::Char('i'));
    expect_action(NORMAL, event, &[Mode::Insert.into()]);
}

#[test]
fn unsupported_key() {
    let event = code_event(KeyCode::Down);
    expect_no_action(BufferMode::Insert, event);
    expect_no_action(NORMAL, event);
}

#[test]
fn wrong_mode_key() {
    expect_no_action(NORMAL, code_event(KeyCode::Char('z')));
    expect_no_action(NORMAL, code_event(KeyCode::Esc));
}

#[test]
fn with_modifiers_char() {
    for modifier in [
        KeyModifiers::CONTROL,
        KeyModifiers::ALT,
        KeyModifiers::SUPER,
        KeyModifiers::HYPER,
        KeyModifiers::META,
    ] {
        let event = event(KeyCode::Char('i'), Some(modifier), None);
        expect_no_action(NORMAL, event);
        expect_no_action(BufferMode::Insert, event);
    }
    let event = event(KeyCode::Char('i'), Some(KeyModifiers::SHIFT), None);
    expect_action(NORMAL, event, &[
        GoToAction::FirstNonSpace.into(),
        Mode::Insert.into(),
    ]);
    expect_action(BufferMode::Insert, event, &[Action::InsertChar('I')]);
}

#[test]
fn with_modifiers_esc() {
    for modifier in [
        KeyModifiers::CONTROL,
        KeyModifiers::ALT,
        KeyModifiers::SUPER,
        KeyModifiers::HYPER,
        KeyModifiers::META,
        KeyModifiers::SHIFT,
    ] {
        let event = event(KeyCode::Esc, Some(modifier), None);
        expect_no_action(NORMAL, event);
        expect_no_action(BufferMode::Insert, event);
    }
}

#[test]
fn not_press() {
    for kind in [KeyEventKind::Release, KeyEventKind::Repeat] {
        let event = event(KeyCode::Char('x'), None, Some(kind));
        expect_no_action(BufferMode::Insert, event);
    }
}

#[test]
fn combinable_pending_cancelled() {
    let mut mode = BufferMode::Normal(Normal::Pending(
        None,
        OPending::CombinablePending(CombinablePending::FindNext),
    ));
    let event = code_event(KeyCode::Esc);
    let actions = mode.handle_event(&event);
    assert_eq!(actions, Actions::Unsupported);
    assert_eq!(mode, NORMAL);
}

#[test]
fn pending_cancelled() {
    let mut mode =
        BufferMode::Normal(Normal::Pending(None, OPending::ReplaceOne));
    let event = code_event(KeyCode::Esc);
    let actions = mode.handle_event(&event);
    assert_eq!(actions, Actions::Unsupported);
    assert_eq!(mode, NORMAL);
}
