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
    ///
    /// To type in content.
    ///
    /// Press `<Esc>` to exit it.
    Insert,
    /// Normal mode
    ///
    /// To move and edit with vim motions.
    ///
    /// Press a, i, A, or I to exit it.
    #[default]
    Normal,
}

impl Mode {
    /// Handle incomming terminal events on any kind.
    pub fn handle_event(
        &self,
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
            |old_pending| {
                Self::handle_opending_event(old_pending, event)
                    .map(|action| vec![action])
                    .unwrap_or_default()
            },
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
    fn handle_opending_event(
        opending: OPending,
        event: &Event,
    ) -> Option<Action> {
        if let Some(key_event) = event.as_key_press_event()
            && let KeyCode::Char(ch) = key_event.code
        {
            match opending {
                OPending::FindNext =>
                    Some(Action::GoTo(GoToAction::NextOccurrenceOf(ch))),
                OPending::FindPrevious =>
                    Some(Action::GoTo(GoToAction::PreviousOccurrenceOf(ch))),
            }
        } else {
            None
        }
    }
}

impl HandleKeyPress for Mode {
    fn handle_blank_key_press(&self, code: KeyCode) -> Actions {
        #[expect(clippy::wildcard_enum_match_arm, reason = "take only a few")]
        match code {
            KeyCode::Left =>
                return vec![Action::GoTo(GoToAction::Left)].into(),
            KeyCode::Right =>
                return vec![Action::GoTo(GoToAction::Right)].into(),
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
fn fix_shift_modifier(key_event: &mut KeyEvent) {
    #[expect(clippy::else_if_without_else, reason = "checked")]
    if let KeyCode::Char(ch) = &mut key_event.code {
        if ch.is_ascii_uppercase() {
            key_event.modifiers |= KeyModifiers::SHIFT;
        } else if key_event.modifiers & KeyModifiers::SHIFT
            == KeyModifiers::SHIFT
        {
            *ch = ch.to_ascii_uppercase();
        }
    }
}
