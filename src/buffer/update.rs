use core::iter::{Rev, Skip};
use core::mem::take;
use core::str::CharIndices;

use crossterm::event::Event;

use crate::Mode;
use crate::buffer::api::Buffer;
use crate::buffer::is_indent::{IsIdentChar, IsSpace};
use crate::buffer::keymaps::{
    Action, Delimitation, GoToAction, Operator, OperatorScope
};
use crate::buffer::mode::Actions;
use crate::event_parser::{EventParsingError, parse_events};
use crate::utils::bounded_usize::BoundedUsize;

impl Buffer {
    /// Capitalise part of the buffer
    fn apply<F>(&mut self, start: usize, end: usize, apply: F)
    where F: Fn(&char) -> char {
        self.content = self
            .as_content()
            .char_indices()
            .map(
                |(idx, ch)| {
                    if idx < start || idx >= end { ch } else { apply(&ch) }
                },
            )
            .collect();
    }

    /// Returns the char pointed by the cursor
    #[expect(clippy::unwrap_used, reason = "in bound")]
    fn as_char(&self) -> char {
        self.content.chars().nth(self.as_cursor()).unwrap()
    }

    /// Returns the index of the cursor, starting from the end of the string.
    #[expect(clippy::arithmetic_side_effects, reason = "cursor <= len")]
    const fn as_end_index(&self) -> usize {
        self.len() - self.as_cursor()
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
    fn delete(&mut self, min_cursor: usize, max_cursor: usize) -> bool {
        let old_content = take(&mut self.content);
        self.content.reserve(old_content.len());
        #[expect(clippy::string_slice, reason = "non-ascii not yet supported")]
        // TODO: add support for UTF-8
        {
            self.content.push_str(&old_content[0..min_cursor]);
            self.content.push_str(&old_content[max_cursor..]);
        };

        #[expect(clippy::string_slice, reason = "non-ascii not yet supported")]
        // TODO: add support for UTF-8
        if max_cursor != min_cursor {
            old_content[min_cursor..max_cursor].clone_into(&mut self.clipboard);
        }
        self.cursor = BoundedUsize::with_capacity(self.content.len());
        self.cursor.set(min_cursor);
        true
    }

    /// Returns the indices that bound the [`Delimitation`]
    fn get_delimitation_indices(
        &self,
        delimitation: Delimitation,
    ) -> Option<(usize, usize)> {
        match delimitation {
            Delimitation::Group(open, close) => self
                .get_delimitation_indices_fn(
                    |ch| ch == open,
                    |ch| ch == close,
                    false,
                ),
            Delimitation::Word => {
                let cursor = IsIdentChar::new(self.as_char());
                let good = |ch| cursor.xor(ch);
                self.get_delimitation_indices_fn(good, good, true)
            }
            Delimitation::WORD => {
                let cursor = IsSpace::new(self.as_char());
                let good = |ch| cursor.xor(ch);
                self.get_delimitation_indices_fn(good, good, true)
            }
        }
    }

    /// Returns the indices that bound some chars, delimited by a function
    #[expect(clippy::arithmetic_side_effects, reason = "in bound")]
    fn get_delimitation_indices_fn(
        &self,
        is_start: impl Fn(char) -> bool,
        is_end: impl Fn(char) -> bool,
        aggressive: bool,
    ) -> Option<(usize, usize)> {
        let mut after = self.chars_after_cursor();
        let mut before = self.chars_before_cursor_rev();

        let at_end = self.as_cursor() == self.len();
        let maybe_start = if at_end || !is_start(self.as_char()) {
            before.find(|ch| is_start(ch.1)).map(|ch| ch.0 + 1)
        } else {
            Some(self.as_cursor() + 1)
        };
        let maybe_end = after.find(|(_, ch)| is_end(*ch)).map(|(idx, _)| idx);

        if aggressive {
            return Some((
                maybe_start.unwrap_or(0),
                maybe_end.unwrap_or(self.len()),
            ));
        }

        match (maybe_start, maybe_end) {
            (Some(start), Some(end)) => Some((start, end)),
            (None | Some(_), None) => None,
            (None, Some(end)) =>
            // PERF: iterating for the second time
                if let Some((start, _)) =
                    self.chars_after_cursor().find(|(_, ch)| is_start(*ch))
                    && start <= end
                {
                    Some((start + 1, end))
                } else {
                    None
                },
        }
    }

    /// Get the cursor indices that describe the part of the buffer to be edited
    /// by the motion of an operator.
    fn get_motion_delimination_indices(
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

    /// Paste the copied content after the cursor
    #[expect(clippy::arithmetic_side_effects, reason = "smaller than len")]
    fn paste_after(&mut self) {
        let pos = if self.as_cursor() >= self.len() {
            self.len()
        } else {
            self.as_cursor() + 1
        };
        self.content.insert_str(pos, &self.clipboard);
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
    fn replace_ch(&mut self, ch: char, can_insert: bool, save: bool) -> bool {
        // PERF: string characters are copied twice.
        let last_char_idx = self.as_cursor();
        let old = if last_char_idx == self.len() {
            if can_insert {
                self.content.push(ch);
                self.cursor.increment_with_capacity_unchecked();
                None
            } else if self.is_empty() {
                return false;
            } else {
                let old = self.content.pop();
                self.content.push(ch);
                old
            }
        } else {
            let old = self.content.remove(last_char_idx);
            self.content.insert(last_char_idx, ch);
            Some(old)
        };
        if save {
            self.pre_replace_content.push(old);
        }
        true
    }

    /// Adds the current buffer to the history, if it is different from the last
    /// entry.
    pub(super) fn save_to_history(&mut self) {
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
    pub fn update_no_save(&mut self, event: &Event) -> bool {
        match self.mode.handle_event(event) {
            Actions::Unsupported => false,
            Actions::List(list) => {
                for action in &list {
                    if !self.update_once(*action) {
                        return false;
                    }
                }
                self.last_action.update(list, self.as_mode());
                true
            }
        }
    }

    /// Updates the buffer with one [`Action`]
    ///
    /// Returns `true` iff the update was successful.
    #[must_use]
    pub(super) fn update_once(&mut self, action: Action) -> bool {
        match action {
            Action::InsertChar(ch) => {
                self.content.insert(self.as_cursor(), ch);
                self.cursor.increment_with_capacity_unchecked();
            }
            Action::SelectMode(mode) => self.mode.switch_to(mode),
            Action::ReplaceWith(ch) =>
                return self.replace_ch(ch, false, false),
            Action::ReplaceOrInsert(ch) =>
                return self.replace_ch(ch, true, true),
            Action::ClearUndoReplace => self.pre_replace_content.clear(),
            Action::UndoReplace =>
                return match self.pre_replace_content.pop() {
                    Some(Some(ch)) =>
                        self.cursor.decrement()
                            && self.replace_ch(ch, false, false),
                    Some(None) => {
                        let hadsome = self.content.pop().is_none();
                        self.cursor.set_max(self.content.len());
                        hadsome
                    }
                    None => self.cursor.decrement(),
                },
            Action::Undo => return self.undo(),
            Action::Redo => return self.redo(),
            Action::GoTo(goto_action) =>
                return self.update_cursor(goto_action),
            Action::Operator(op, scope) =>
                return self.update_with_operator(op, scope),
            Action::PasteAfter => self.paste_after(),
            Action::PasteBefore => self.content.insert_str(
                self.as_cursor().saturating_sub(1),
                &self.clipboard,
            ),
            Action::Repeat => {
                let last = take(&mut self.last_action);
                let ok = last.perform(self);
                self.last_action = last;
                return ok;
            }
        }
        true
    }

    /// Updates the buffer with an [`Operator`] action.
    fn update_with_operator(
        &mut self,
        op: Operator,
        scope: OperatorScope,
    ) -> bool {
        let Some((min, max)) = (match scope {
            OperatorScope::WholeLine => Some((0, self.len())),
            OperatorScope::Goto(first, second) =>
                self.get_motion_delimination_indices(first, second),
            OperatorScope::Delimitation(delim) =>
                self.get_delimitation_indices(delim),
        }) else {
            return false;
        };
        self.cursor.set(min);
        let fun = match op {
            Operator::Delete => return self.delete(min, max),
            Operator::Yank => {
                #[expect(clippy::string_slice, reason = "utf8 not supported")]
                self.content[min..max].clone_into(&mut self.clipboard);
                return true;
            }
            Operator::Change =>
                return self.delete(min, max) && {
                    self.mode.switch_to(Mode::Insert);
                    true
                },
            Operator::Capitalise => char::to_ascii_uppercase,
            Operator::LowerCase => char::to_ascii_lowercase,
            Operator::ToggleCase => toggle_case,
        };
        self.apply(min, max, fun);
        true
    }
}

/// Toggles the case of a char: capitals will be lowered and lower case letters
/// will be capitalised.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "align with to_ascii_{upper,lower}case"
)]
const fn toggle_case(ch: &char) -> char {
    if ch.is_ascii_uppercase() {
        ch.to_ascii_lowercase()
    } else {
        ch.to_ascii_uppercase()
    }
}
