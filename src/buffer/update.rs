use core::mem::take;

use crossterm::event::Event;

use crate::buffer::api::Buffer;
use crate::buffer::keymaps::{Action, GoToAction};
use crate::event_parser::{EventParsingError, parse_events};

impl Buffer {
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
    ///
    /// # Examples
    ///
    /// ```
    /// use vim_buffer::Buffer;
    /// use vim_buffer::crossterm::event::{Event, KeyCode, KeyEvent};
    ///
    /// let mut buffer = Buffer::default();
    ///
    /// // Update it with crossterm events
    /// buffer.update(&Event::Key(KeyEvent::from(KeyCode::Char('i'))));
    /// for ch in "hello".chars() {
    ///     buffer.update(&Event::Key(KeyEvent::from(KeyCode::Char('h'))));
    /// }
    /// buffer.update(&Event::Key(KeyEvent::from(KeyCode::Esc)));
    /// buffer.update(&Event::Key(KeyEvent::from(KeyCode::Char('^'))));
    /// buffer.update(&Event::Key(KeyEvent::from(KeyCode::Char('s'))));
    /// buffer.update(&Event::Key(KeyEvent::from(KeyCode::Char('H'))));
    /// ```
    pub fn update(&mut self, event: &Event) -> bool {
        let events = self.as_mode().handle_event(event, &mut self.pending);

        for action in &events {
            if !self.update_once(*action) {
                return false;
            }
        }

        !events.is_empty()
    }

    /// Updates the cursor position with a [`GoToAction`]
    ///
    /// Returns `true` if the action was successful.
    #[must_use]
    fn update_cursor(&mut self, goto_action: GoToAction) -> bool {
        match goto_action {
            GoToAction::Right => self.cursor.increment(),
            GoToAction::Left => self.cursor.decrement(),
            GoToAction::Bol => self.cursor.set(0),
            GoToAction::Eol => self.cursor.set_to_max(),
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
                } else {
                    return false;
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
                    .find(|&(_idx, next)| next == ch)
                {
                    self.cursor.set(idx);
                } else {
                    return false;
                }
            }
            GoToAction::NextWord => {
                let mut chars =
                    self.as_content().char_indices().skip(self.as_cursor());
                if let Some((_, cursor_ch)) = chars.next()
                    && let Some((idx, next_ch)) =
                        chars.find(|(_idx, ch)| xor_ident_char(cursor_ch, *ch))
                {
                    if next_ch.is_whitespace() {
                        if let Some((non_space_idx, _)) =
                            chars.find(|(_idx, ch)| !ch.is_whitespace())
                        {
                            self.cursor.set(non_space_idx);
                        } else {
                            self.cursor.set_to_max();
                        }
                    } else {
                        self.cursor.set(idx);
                    }
                } else {
                    self.cursor.set_to_max();
                }
            }
        }
        true
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
    ///
    /// # Examples
    ///
    /// ```
    /// use vim_buffer::Buffer;
    ///
    /// let mut buffer = Buffer::default();
    ///
    /// buffer.update_from_string("iHello, World!");
    /// assert_eq!(buffer.as_content(), "Hello, World!");
    ///
    /// buffer.update_from_string("<Esc>F,xllrwFHrhf!x");
    /// assert_eq!(buffer.as_content(), "hello world");
    /// ```
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
    ///
    /// Returns `true` iff the update was successful.
    #[must_use]
    fn update_once(&mut self, action: Action) -> bool {
        match action {
            Action::InsertChar(ch) => {
                self.content.insert(self.as_cursor(), ch);
                self.cursor.increment_with_capacity_unchecked();
                true
            }
            Action::SelectMode(mode) => {
                self.mode = mode;
                true
            }
            Action::DeleteNextChar =>
                if self.is_empty() {
                    false
                } else {
                    self.content.remove(self.as_cursor());
                    self.cursor.decrement_with_capacity();
                    true
                },
            Action::DeletePreviousChar =>
                if self.as_cursor() != 0 {
                    self.cursor.decrement_with_capacity();
                    self.content.remove(self.as_cursor());
                    true
                } else {
                    false
                },
            Action::DeleteLine => {
                self.content.clear();
                take(&mut self.cursor);
                true
            }
            Action::ReplaceWith(ch) => {
                // PERF: string characters are copied twice.
                self.content.remove(self.as_cursor());
                self.content.insert(self.as_cursor(), ch);
                true
            }
            Action::GoTo(goto_action) => self.update_cursor(goto_action),
        }
    }
}

/// Checks that first or second is ident valid, but not both.
const fn xor_ident_char(first: char, second: char) -> bool {
    is_ident_char(first) ^ is_ident_char(second)
}

/// Returns `true` if the given char is valid for an identifier
const fn is_ident_char(ch: char) -> bool {
    matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_')
}
