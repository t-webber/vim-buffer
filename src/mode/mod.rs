/// Handles keypresses in insert mode
mod insert;
/// Handles keypresses in normal mode
mod normal;

use crossterm::event::Event;

use crate::action::Action;

/// Represents the vim mode of the buffer.
#[non_exhaustive]
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
    Normal,
}

/// Handle incomming terminal events, like keypresses.
trait HandleEvent {
    /// Handle incomming terminal events, like keypresses.
    fn handle_event(self, event: Event) -> Option<Action>;
}
