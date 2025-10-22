/// Handles keypresses in insert mode
mod insert;
/// Handles keypresses in normal mode
mod normal;

use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::buffer::action::{Action, GoToAction};
use crate::buffer::mode::insert::Insert;
use crate::buffer::mode::normal::Normal;
use crate::buffer::o_pending::OPending;

/// Handle incomming terminal events, like keypresses.
trait HandleKeyPress {
    /// Handle incomming terminal events that are keypresses with no modifiers.
    fn handle_blank_key_press(
        &self,
        code: KeyCode,
        pending: &mut Option<OPending>,
    ) -> Vec<Action>;

    /// Handle incomming terminal events that are keypresses with no modifiers.
    fn handle_shift_key_press(
        &self,
        code: KeyCode,
        pending: &mut Option<OPending>,
    ) -> Vec<Action>;
}


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
        event.as_key_press_event().map_or_else(Vec::new, |key_event| {
            match key_event.modifiers {
                KeyModifiers::NONE =>
                    self.handle_blank_key_press(key_event.code, pending),
                KeyModifiers::SHIFT =>
                    self.handle_shift_key_press(key_event.code, pending),
                _ => vec![],
            }
        })
    }
}

impl HandleKeyPress for Mode {
    fn handle_blank_key_press(
        &self,
        code: KeyCode,
        pending: &mut Option<OPending>,
    ) -> Vec<Action> {
        #[expect(clippy::wildcard_enum_match_arm, reason = "take only a few")]
        match code {
            KeyCode::Left => return vec![Action::GoTo(GoToAction::Left)],
            KeyCode::Right => return vec![Action::GoTo(GoToAction::Right)],
            _ => (),
        }
        match *self {
            Self::Insert => Insert.handle_blank_key_press(code, pending),
            Self::Normal => Normal.handle_blank_key_press(code, pending),
        }
    }

    fn handle_shift_key_press(
        &self,
        code: KeyCode,
        pending: &mut Option<OPending>,
    ) -> Vec<Action> {
        match *self {
            Self::Insert => Insert.handle_shift_key_press(code, pending),
            Self::Normal => Normal.handle_shift_key_press(code, pending),
        }
    }
}

#[cfg(test)]
mod tests;
