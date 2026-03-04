use core::iter::{Rev, Skip};
use core::mem::take;
use core::str::CharIndices;

use crossterm::event::Event;

use crate::Mode;
use crate::buffer::api::Buffer;
use crate::buffer::bounded_usize::BoundedUsize;
use crate::buffer::is_indent::IsIdentChar;
use crate::buffer::keymaps::{Action, GoToAction, OperatorScope};
use crate::buffer::mode::Actions;
use crate::event_parser::{EventParsingError, parse_events};

impl Buffer {
    /// Returns the index of the cursor, starting from the end of the string.
    #[expect(clippy::arithmetic_side_effects, reason = "cursor <= len")]
    const fn as_end_index(&self) -> usize {
        self.len() - self.as_cursor()
    }

    /// Capitalise part of the buffer
    fn capitalise(&mut self, start: usize, end: usize) {
        self.content = self
            .content
            .char_indices()
            .map(|(idx, ch)| {
                if idx < start || idx >= end {
                    ch
                } else {
                    ch.to_ascii_uppercase()
                }
            })
            .collect();
        self.cursor.set(start);
    }

    /// Returns [`CharIndices`] iterator for all chars located after the cursor
    /// in the buffer.
    fn chars_after_cursor(&self) -> Skip<CharIndices<'_>> {
        self.as_content().char_indices().skip(self.as_cursor())
    }

    /// Returns [`CharIndices`] iterator for all chars located before the cursor
    /// in the buffer, and this in a reverse order.
    fn chars_before_cursor_rev(&self) -> Skip<Rev<CharIndices<'_>>> {
        self.as_content().char_indices().rev().skip(self.as_end_index())
    }

    /// Deletes the part of the buffer represented by one or two [`GoToAction`]
    ///
    /// The deleted part is from the current cursor to the cursor after the
    /// [`GoToAction`].
    fn delete(
        &mut self,
        first: GoToAction,
        maybe_second: Option<GoToAction>,
    ) -> bool {
        let Some((min_cursor, max_cursor)) =
            self.get_motion_delimination(first, maybe_second)
        else {
            return false;
        };
        let old_content = take(&mut self.content);
        self.content.reserve(old_content.len());
        #[expect(clippy::string_slice, reason = "")]
        {
            self.content.push_str(&old_content[0..min_cursor]);
            self.content.push_str(&old_content[max_cursor..]);
        };
        self.cursor = BoundedUsize::with_capacity(self.content.len());
        self.cursor.set(min_cursor);
        true
    }

    /// Get the cursor indices that describe the part of the buffer to be edited
    /// by the motion of an operator.
    fn get_motion_delimination(
        &mut self,
        first: GoToAction,
        maybe_second: Option<GoToAction>,
    ) -> Option<(usize, usize)> {
        let old_cursor = self.as_cursor();
        if !self.update_cursor(first)
            || maybe_second.is_some_and(|second| !self.update_cursor(second))
        {
            return None;
        }
        let new_cursor = self.as_cursor();
        let max = new_cursor.max(old_cursor);
        let min = new_cursor.min(old_cursor);
        if matches!(first, GoToAction::EndWord | GoToAction::EndWORD) {
            Some((min, max.saturating_add(1).min(self.len())))
        } else {
            Some((min, max))
        }
    }

    /// Moves the cursor to the beginning of the previous WORD.
    #[expect(non_snake_case, reason = "vim wording")]
    fn goto_beginning_of_WORD(&mut self) {
        let mut chars = self.chars_before_cursor_rev();
        if let Some(..) = chars.find(|(_, ch)| !ch.is_whitespace())
            && let Some((idx, _)) = chars.find(|(_, ch)| ch.is_whitespace())
        {
            self.cursor.set(idx);
            self.cursor.increment();
        } else {
            self.cursor.set(0);
        }
    }

    /// Moves the cursor to the beginning of the previous word.
    fn goto_beginning_of_word(&mut self) {
        let mut chars = self.chars_before_cursor_rev();
        if let Some((_, word_ch)) = chars.find(|(_, ch)| !ch.is_whitespace())
            && let cursor = IsIdentChar::new(word_ch)
            && let Some((idx, _)) = chars.find(|(_, ch)| cursor.xor(*ch))
        {
            self.cursor.set(idx);
            self.cursor.increment();
        } else {
            self.cursor.set(0);
        }
    }

    /// Moves the cursor to the end of the current or next word.
    #[expect(non_snake_case, reason = "vim wording")]
    fn goto_end_WORD(&mut self) {
        let mut chars = self.chars_after_cursor().skip(1);
        if let Some(..) = chars.find(|(_, ch)| !ch.is_whitespace())
            && let Some((idx, _)) = chars.find(|(_, ch)| ch.is_whitespace())
        {
            self.cursor.set(idx);
            self.cursor.decrement();
        } else {
            self.cursor.set_to_max();
        }
    }

    /// Moves the cursor to the end of the previous WORD.
    #[expect(non_snake_case, reason = "vim wording")]
    fn goto_end_of_previous_WORD(&mut self) {
        let idx = self
            .as_content()
            .char_indices()
            .rev()
            .skip(self.as_end_index().saturating_sub(1))
            .skip_while(|(_, ch)| !ch.is_whitespace())
            .find(|(_, ch)| !ch.is_whitespace())
            .map_or(0, |(i, _)| i);
        self.cursor.set(idx);
    }

    /// Moves the cursor to the end of the previous word.
    fn goto_end_of_previous_word(&mut self) {
        let mut chars = self
            .as_content()
            .char_indices()
            .rev()
            .skip(self.as_end_index().saturating_sub(1));
        let Some((_, cursor_ch)) = chars.next() else {
            return self.cursor.set(0);
        };
        if !cursor_ch.is_whitespace() {
            let cursor = IsIdentChar::new(cursor_ch);
            match chars.find(|(_, ch)| cursor.xor(*ch)) {
                None => return self.cursor.set(0),
                Some((idx, ch)) if !ch.is_whitespace() =>
                    return self.cursor.set(idx),
                Some(_) => {}
            }
        }
        let idx =
            chars.find(|(_, ch)| !ch.is_whitespace()).map_or(0, |(i, _)| i);
        self.cursor.set(idx);
    }

    /// Moves the cursor to the end of the current or next word.
    fn goto_end_word(&mut self) {
        let mut chars = self.chars_after_cursor().skip(1);
        if let Some((_, cursor_ch)) = chars.find(|(_, ch)| !ch.is_whitespace())
            && let cursor = IsIdentChar::new(cursor_ch)
            && let Some((idx, _)) = chars.find(|(_, ch)| cursor.xor(*ch))
        {
            self.cursor.set(idx);
            self.cursor.decrement();
        } else {
            self.cursor.set_to_max();
        }
    }

    /// Moves the cursor to the beginning of the next WORD.
    #[expect(non_snake_case, reason = "vim wording")]
    fn goto_next_WORD(&mut self) {
        let mut chars = self.chars_after_cursor();
        if let Some(..) = chars.find(|(_, ch)| ch.is_whitespace())
            && let Some((idx, _)) = chars.find(|(_, ch)| !ch.is_whitespace())
        {
            self.cursor.set(idx);
        } else {
            self.cursor.set_to_max();
        }
    }

    /// Moves the cursor to the beginning of the next word.
    fn goto_next_word(&mut self) {
        let mut chars = self.chars_after_cursor();
        if let Some((_, cursor_ch)) = chars.next()
            && let cursor = IsIdentChar::new(cursor_ch)
            && let Some((idx, next_ch)) = chars.find(|(_, ch)| cursor.xor(*ch))
        {
            if next_ch.is_whitespace() {
                if let Some((non_space_idx, _)) =
                    chars.find(|(_, ch)| !ch.is_whitespace())
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

    /// Undos the latest undo
    fn redo(&mut self) -> bool {
        if let Some(previous) = self.history.redo() {
            self.content = previous.to_owned().into_string();
            self.cursor.set_max(self.len());
            true
        } else {
            false
        }
    }

    /// Remove the character under the current cursor and replace it by another
    /// one.
    fn replace_ch<F: Fn(char) -> char>(&mut self, replace: F) -> bool {
        if self.is_empty() {
            false
        } else {
            // PERF: string characters are copied twice.
            let last_char_idx = self.as_cursor();
            let old = self.content.remove(last_char_idx);
            self.content.insert(last_char_idx, replace(old));
            true
        }
    }

    /// Adds the current buffer to the history, if it is different from the last
    /// entry.
    fn save_to_history(&mut self) {
        if matches!(self.as_mode(), Mode::Normal) {
            self.history.save(&self.content);
        }
    }

    /// Pops from history the first different  buffer value
    fn undo(&mut self) -> bool {
        if let Some(previous) = self.history.undo() {
            self.content = previous.to_owned().into_string();
            self.cursor.set_max(self.len());
            true
        } else {
            false
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
            GoToAction::NextChar =>
                if self.as_cursor().saturating_add(1) < self.len() {
                    self.cursor.increment();
                },
            GoToAction::Left => drop(self.cursor.decrement()),
            GoToAction::BeginningOfLine => self.cursor.set(0),
            GoToAction::EndOfLine => self.cursor.set_to_max(),
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
            GoToAction::BeginningOfWORD => self.goto_beginning_of_WORD(),
            GoToAction::BeginningOfWord => self.goto_beginning_of_word(),
            GoToAction::EndWord => self.goto_end_word(),
            GoToAction::EndWORD => self.goto_end_WORD(),
            GoToAction::EndOfPreviousWord => self.goto_end_of_previous_word(),
            GoToAction::EndOfPreviousWORD => self.goto_end_of_previous_WORD(),
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
    /// buffer.update_from_string("<Esc>F,xlrwFHrhf!x");
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

    /// Same as [`Self::update`] but without updating the history.
    fn update_no_save(&mut self, event: &Event) -> bool {
        match self.as_mode().handle_event(event, take(&mut self.pending)) {
            Actions::OPending(new_pending) => {
                self.pending = Some(new_pending);
                true
            }
            Actions::List(list) => {
                for action in &list {
                    if !self.update_once(*action) {
                        return false;
                    }
                }
                !list.is_empty()
            }
        }
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
            Action::Delete(OperatorScope::WholeLine) => {
                self.content.clear();
                take(&mut self.cursor);
                true
            }
            Action::ReplaceWith(ch) => self.replace_ch(|_| ch),
            Action::Undo => self.undo(),
            Action::Redo => self.redo(),
            Action::GoTo(goto_action) => self.update_cursor(goto_action),
            Action::Delete(OperatorScope::Goto(first, second)) =>
                self.delete(first, second),
            Action::ToggleCapitalisation => self.replace_ch(|old| {
                if old.is_ascii_lowercase() {
                    old.to_ascii_uppercase()
                } else {
                    old.to_ascii_lowercase()
                }
            }),
            Action::Capitalise(OperatorScope::WholeLine) => {
                self.capitalise(0, self.len());
                true
            }
            Action::Capitalise(OperatorScope::Goto(first, second)) =>
                if let Some((min, max)) =
                    self.get_motion_delimination(first, second)
                {
                    self.capitalise(min, max);
                    true
                } else {
                    false
                },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Buffer;
    use crate::buffer::keymaps::GoToAction;

    #[test]
    fn double_delete() {
        let mut x = Buffer::from("abc");
        x.delete(
            GoToAction::NextOccurrenceOf('c'),
            Some(GoToAction::NextOccurrenceOf('e')),
        );
    }
}
