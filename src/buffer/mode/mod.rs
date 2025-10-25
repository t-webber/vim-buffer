/// Handles keypresses in insert mode
mod insert;
/// Handles keypresses in normal mode
mod normal;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::buffer::keymaps::{Action, GoToAction, OPending};
use crate::buffer::mode::insert::Insert;
use crate::buffer::mode::normal::Normal;

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
        event.as_key_press_event().map_or_else(Vec::new, |mut key_event| {
            fix_shift_modifier(&mut key_event);
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
        if pending.is_none() {
            #[expect(
                clippy::wildcard_enum_match_arm,
                reason = "take only a few"
            )]
            match code {
                KeyCode::Left => return vec![Action::GoTo(GoToAction::Left)],
                KeyCode::Right => return vec![Action::GoTo(GoToAction::Right)],
                _ => (),
            }
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

#[cfg(test)]
mod tests;
