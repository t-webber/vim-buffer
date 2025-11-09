use core::mem::take;

use crossterm::event::Event;

use crate::Mode;
use crate::buffer::api::Buffer;
use crate::buffer::keymaps::{Action, GoToAction};
use crate::event_parser::{EventParsingError, parse_events};

impl Buffer {
    /// Returns the index of the cursor, starting from the end of the string.
    #[expect(clippy::arithmetic_side_effects, reason = "cursor <= len")]
    const fn as_end_index(&self) -> usize {
        self.len() - self.as_cursor()
    }

    /// Moves the cursor to the beginning of the next WORD.
    #[expect(non_snake_case, reason = "vim wording")]
    fn goto_next_WORD(&mut self) {
        let mut chars = self.as_content().char_indices().skip(self.as_cursor());
        if let Some((_, cursor_ch)) = chars.next()
            && cursor_ch.is_whitespace()
            && let Some((idx, _)) = chars.find(|(_, ch)| !ch.is_whitespace())
        {
            self.cursor.set(idx);
        } else if self.update_cursor(GoToAction::NextOccurrenceOf(' ')) {
            self.goto_next_WORD();
        } else {
            self.cursor.set_to_max();
        }
    }

    /// Moves the cursor to the beginning of the next word.
    fn goto_next_word(&mut self) {
        let mut chars = self.as_content().char_indices().skip(self.as_cursor());
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

    /// Moves the cursor to the beginning of the previous WORD.
    #[expect(non_snake_case, reason = "vim wording")]
    fn goto_previous_WORD(&mut self) {
        let mut chars =
            self.as_content().char_indices().rev().skip(self.as_end_index());
        if let Some((_, cursor_ch)) = chars.next()
            && cursor_ch.is_whitespace()
        {
            if let Some((idx, _)) = chars.find(|(_, ch)| !ch.is_whitespace()) {
                self.cursor.set(idx);
                self.goto_previous_WORD();
            } else {
                self.cursor.set(0);
            }
        } else if self.update_cursor(GoToAction::PreviousOccurrenceOf(' ')) {
            self.cursor.increment();
        } else {
            self.cursor.set(0);
        }
    }

    /// Moves the cursor to the beginning of the previous word.
    fn goto_previous_word(&mut self) {
        let mut chars =
            self.as_content().char_indices().rev().skip(self.as_end_index());
        if let Some((_, cursor_ch)) = chars.next() {
            if cursor_ch.is_whitespace()
                && let Some((idx, _)) =
                    chars.find(|(_, ch)| !ch.is_whitespace())
            {
                self.cursor.set(idx);
                return self.goto_previous_word();
            }
            if let Some((idx, _)) =
                chars.find(|(_, ch)| xor_ident_char(cursor_ch, *ch))
                && idx < self.as_cursor()
            {
                self.cursor.set(idx);
                self.cursor.increment();
            } else {
                self.cursor.set(0);
            }
        }
    }

    /// Pops from history the first different  buffer value
    fn pop_from_history(&mut self) -> bool {
        if let Some(previous) = self.history.undo(&self.content) {
            self.content = previous.to_owned();
            self.cursor.set_max(self.len());
            true
        } else {
            false
        }
    }

    /// Adds the current buffer to the history, if it is different from the last
    /// entry.
    fn save_to_history(&mut self) {
        if matches!(self.as_mode(), Mode::Normal) {
            self.history.save(&self.content);
        }
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
        let success = self.update_no_save(event);
        self.save_to_history();
        success
    }

    /// Updates the cursor position with a [`GoToAction`]
    ///
    /// Returns `true` if the action was successful.
    #[must_use]
    fn update_cursor(&mut self, goto_action: GoToAction) -> bool {
        match goto_action {
            GoToAction::Right => drop(self.cursor.increment()),
            GoToAction::Left => drop(self.cursor.decrement()),
            GoToAction::Bol => self.cursor.set(0),
            GoToAction::Eol => self.cursor.set_to_max(),
            GoToAction::FirstNonSpace => self.cursor.set(
                self.as_content()
                    .char_indices()
                    .find(|(_idx, ch)| !ch.is_whitespace())
                    .map_or_else(|| self.len(), |(idx, _ch)| idx),
            ),
            GoToAction::NextOccurrenceOf(ch) => self.cursor.set(
                if let Some((idx, _ch)) = self
                    .as_content()
                    .char_indices()
                    .skip(self.as_cursor())
                    .skip(1)
                    .find(|(_idx, next)| *next == ch)
                {
                    idx
                } else {
                    return false;
                },
            ),
            GoToAction::PreviousOccurrenceOf(ch) => self.cursor.set(
                if let Some((idx, _ch)) = self
                    .as_content()
                    .char_indices()
                    .rev()
                    .skip(self.as_end_index())
                    .find(|&(_idx, next)| next == ch)
                {
                    idx
                } else {
                    return false;
                },
            ),
            GoToAction::NextWORD => self.goto_next_WORD(),
            GoToAction::NextWord => self.goto_next_word(),
            GoToAction::PreviousWORD => self.goto_previous_WORD(),
            GoToAction::PreviousWord => self.goto_previous_word(),
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
            self.update_no_save(&event);
        }
        self.save_to_history();
        Ok(())
    }

    /// Same as [`Self::update`] but without updating the history.
    fn update_no_save(&mut self, event: &Event) -> bool {
        let events = self.as_mode().handle_event(event, &mut self.pending);

        for action in &events {
            if !self.update_once(*action) {
                return false;
            }
        }

        !events.is_empty()
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
            Action::Undo => self.pop_from_history(),
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
