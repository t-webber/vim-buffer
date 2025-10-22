/// Defines the actions that can be made on the buffer
mod action;
/// Defines a bounded usize newtype, to safely increment, decrement a cursor.
mod bounded_usize;
/// Handles the vim modes and the keypresses on those modes
mod mode;
/// Pending keys that need more keys before doing an action.
mod o_pending;

#[cfg(test)]
mod tests;

use crossterm::event::Event;
pub use mode::Mode;

use crate::buffer::action::{Action, GoToAction};
use crate::buffer::bounded_usize::BoundedUsize;
use crate::buffer::o_pending::OPending;
use crate::event_parser::{EventParsingError, parse_events};

/// Buffer that supports vim keymaps
#[derive(Debug, Default)]
pub struct Buffer {
    /// Content of the buffer
    content: String,
    /// Position of the cursor within the buffer
    cursor:  BoundedUsize,
    /// Vim mode of the buffer
    mode:    Mode,
    /// Pending actions that require more keymaps
    pending: Option<OPending>,
}

impl Buffer {
    /// Returns the inner text content of the buffer
    #[must_use]
    pub const fn as_content(&self) -> &String {
        &self.content
    }

    /// Returns the cursor position in the buffer
    #[must_use]
    pub const fn as_cursor(&self) -> usize {
        self.cursor.as_value()
    }

    /// Returns the vim mode of the buffer (insert, normal, etc.)
    #[must_use]
    pub const fn as_mode(&self) -> Mode {
        self.mode
    }

    /// Updates the buffer with a terminal event
    ///
    /// # Returns
    ///
    /// `true` if the buffer was changed, and `false` if the [`Event`] is
    /// ignored.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer exceeds [`usize::MAX`]
    pub fn update(&mut self, event: &Event) -> bool {
        let events = self.mode.handle_event(event, &mut self.pending);

        for action in &events {
            self.update_once(*action);
        }

        !events.is_empty()
    }

    /// Updates the cursor position with a [`GoToAction`]
    fn update_cursor(&mut self, goto_action: GoToAction) {
        match goto_action {
            GoToAction::Right => self.cursor.increment(),
            GoToAction::Left => self.cursor.decrement(),
            GoToAction::Eol => self.cursor.set(self.content.len()),
            GoToAction::FirstNonSpace => {
                self.cursor.set(0);
                let mut chars = self.content.chars();
                loop {
                    match chars.next() {
                        Some(current) if current.is_whitespace() => (),
                        None | Some(_) => break,
                    }
                    self.cursor.increment();
                }
            }
            GoToAction::NextOccurrenceOf(ch) => {
                let mut chars =
                    self.content.chars().skip(self.cursor.as_value());
                loop {
                    if let Some(next) = chars.next()
                        && next == ch
                    {
                        break;
                    }
                    self.cursor.increment();
                }
            }
        }
    }

    /// Updates the buffer using a string with all the keymaps.
    ///
    /// The string must be in the format that are valid according to the
    /// `vim.keymap` documentation.
    ///
    /// For example, `"i0<Esc><C-A>a0<Esc>I0"` will create a buffer whose
    /// content is `"020"`.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is invalid, and the parser failed to
    /// convert it to a list of events.
    pub fn update_from_string(
        &mut self,
        keymaps: &str,
    ) -> Result<(), EventParsingError> {
        for event in parse_events(keymaps)? {
            self.update(&event);
        }
        Ok(())
    }

    /// Updates the buffer with one [`Action`]
    fn update_once(&mut self, action: Action) {
        match action {
            Action::InsertChar(ch) => {
                self.content.insert(self.cursor.as_value(), ch);
                self.cursor.increment_with_capacity_unchecked();
            }
            Action::SelectMode(mode) => self.mode = mode,
            Action::Backspace =>
                if self.cursor.as_value() != 0 {
                    self.cursor.decrement_with_capacity();
                    self.content.remove(self.cursor.as_value());
                },
            Action::GoTo(goto_action) => self.update_cursor(goto_action),
        }
    }
}
