use core::mem::take;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::buffer::keymaps::{Action, GoToAction, OPending};
use crate::buffer::mode::insert::Insert;
use crate::buffer::mode::normal::Normal;
use crate::buffer::mode::traits::{Actions, HandleKeyPress};

/// Represents the vim mode of the buffer.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Insert mode
    Insert,
    /// Normal mode
    #[default]
    Normal,
}

impl Mode {
    /// Handle incoming terminal events on any kind.
    pub(crate) fn handle_event(
        self,
        event: &Event,
        pending: &mut Option<OPending>,
    ) -> Vec<Action> {
        take(pending).map_or_else(
            || match self.handle_non_opending_event(event) {
                Actions::List(actions) => actions,
                Actions::OPending(opending) => {
                    *pending = Some(opending);
                    vec![]
                }
            },
            |old_pending| Self::handle_opending_event(old_pending, event),
        )
    }

    /// Handles the terminal events when not in [`OPending`] mode
    fn handle_non_opending_event(self, event: &Event) -> Actions {
        event.as_key_press_event().map_or_else(
            Actions::default,
            |mut key_event| {
                fix_shift_modifier(&mut key_event);
                match key_event.modifiers {
                    KeyModifiers::NONE =>
                        self.handle_blank_key_press(key_event.code),
                    KeyModifiers::SHIFT =>
                        self.handle_shift_key_press(key_event.code),
                    _ => Actions::default(),
                }
            },
        )
    }

    /// Handle a keypress when an [`OPending`] is in progress and waiting for
    /// keys.
    fn handle_opending_event(opending: OPending, event: &Event) -> Vec<Action> {
        if let Some(key_event) = event.as_key_press_event()
            && let KeyCode::Char(ch) = key_event.code
        {
            match opending {
                OPending::FindNext =>
                    vec![GoToAction::NextOccurrenceOf(ch).into()],
                OPending::FindNextDecrement => vec![
                    GoToAction::NextOccurrenceOf(ch).into(),
                    GoToAction::Left.into(),
                ],
                OPending::FindPrevious =>
                    vec![GoToAction::PreviousOccurrenceOf(ch).into()],
                OPending::FindPreviousIncrement => vec![
                    GoToAction::PreviousOccurrenceOf(ch).into(),
                    GoToAction::Right.into(),
                ],
            }
        } else {
            vec![]
        }
    }
}

impl HandleKeyPress for Mode {
    fn handle_blank_key_press(&self, code: KeyCode) -> Actions {
        #[expect(clippy::wildcard_enum_match_arm, reason = "take only a few")]
        match code {
            KeyCode::Left => return vec![GoToAction::Left.into()].into(),
            KeyCode::Right => return vec![GoToAction::Right.into()].into(),
            _ => (),
        }
        match *self {
            Self::Insert => Insert.handle_blank_key_press(code),
            Self::Normal => Normal.handle_blank_key_press(code),
        }
    }

    fn handle_shift_key_press(&self, code: KeyCode) -> Actions {
        match *self {
            Self::Insert => Insert.handle_shift_key_press(code),
            Self::Normal => Normal.handle_shift_key_press(code),
        }
    }
}

/// Adds [`KeyModifiers::SHIFT`] if the event is a capital char, and capitalises
/// the char if the modifiers contain shift.
const fn fix_shift_modifier(key_event: &mut KeyEvent) {
    #[expect(clippy::else_if_without_else, reason = "checked")]
    if let KeyCode::Char(ch) = &mut key_event.code {
        if ch.is_ascii_uppercase() {
            key_event.modifiers =
                key_event.modifiers.union(KeyModifiers::SHIFT);
        } else if key_event.modifiers.contains(KeyModifiers::SHIFT) {
            *ch = ch.to_ascii_uppercase();
        }
    }
}
