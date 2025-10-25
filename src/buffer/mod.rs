/// Defines a bounded usize newtype, to safely increment, decrement a cursor.
mod bounded_usize;
/// Defines the actions that can be made on the buffer
mod keymaps;
/// Handles the vim modes and the keypresses on those modes
mod mode;

#[cfg(test)]
mod tests;

use crossterm::event::Event;

use crate::buffer::bounded_usize::BoundedUsize;
use crate::buffer::keymaps::{Action, GoToAction, OPending};
pub use crate::buffer::mode::Mode;
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

    /// Returns `true` if the buffer is empty, and `false` otherwise.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Returns the length of the buffer
    #[must_use]
    pub const fn len(&self) -> usize {
        self.content.len()
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
            GoToAction::Bol => self.cursor.set(0),
            GoToAction::Eol => self.cursor.set(self.len()),
            GoToAction::FirstNonSpace => {
                let idx = self
                    .as_content()
                    .char_indices()
                    .find(|(_idx, ch)| !ch.is_whitespace())
                    .map_or_else(|| self.len(), |(idx, _ch)| idx);
                self.cursor.set(idx);
            }
            GoToAction::NextOccurrenceOf(ch) => {
                if let Some((idx, _ch)) = self
                    .as_content()
                    .char_indices()
                    .skip(self.as_cursor())
                    .skip(1)
                    .find(|(_idx, next)| *next == ch)
                {
                    self.cursor.set(idx);
                }
            }
            GoToAction::PreviousOccurrenceOf(ch) => {
                #[expect(
                    clippy::arithmetic_side_effects,
                    reason = "cursor <= len"
                )]
                if let Some((idx, _ch)) = self
                    .as_content()
                    .char_indices()
                    .rev()
                    .skip(self.len() - self.as_cursor())
                    .skip(1)
                    .find(|&(_idx, next)| next == ch)
                {
                    self.cursor.set(idx);
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
            Action::DeleteChar =>
                if self.cursor.as_value() != 0 {
                    self.cursor.decrement_with_capacity();
                    self.content.remove(self.cursor.as_value());
                },
            Action::GoTo(goto_action) => self.update_cursor(goto_action),
        }
    }
}

impl From<String> for Buffer {
    fn from(value: String) -> Self {
        Self {
            cursor: BoundedUsize::with_capacity(value.len()),
            content: value,
            ..Default::default()
        }
    }
}

impl From<&str> for Buffer {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}
