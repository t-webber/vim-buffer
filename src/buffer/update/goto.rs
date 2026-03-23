use crate::Buffer;
use crate::buffer::is_indent::IsIdentChar;
use crate::buffer::keymaps::GoToAction;

impl Buffer {
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
            .map_or(0, |(idx, _)| idx);
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
                Some(_) => (),
            }
        }
        let idx =
            chars.find(|(_, ch)| !ch.is_whitespace()).map_or(0, |(idx, _)| idx);
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

    /// Moves the cursor to the end of the next group, formed by any kind of
    /// parenthesis
    fn goto_next_group(&mut self) -> bool {
        let mut after = self.chars_after_cursor();
        let compl = [('{', '}'), ('[', ']'), ('(', ')'), ('<', '>')];
        let cursor = self.as_char();

        match cursor {
            '{' | '[' | '(' | '<' =>
                after.find(|(_, ch)| compl.contains(&(cursor, *ch))),
            '}' | ']' | ')' | '>' => self
                .chars_before_cursor_rev()
                .find(|(_, ch)| compl.contains(&(*ch, cursor))),
            _ => after
                .find(|(_, ch)| {
                    matches!(ch, '{' | '}' | '[' | ']' | '(' | ')' | '<' | '>')
                })
                .and_then(|(_, found)| match found {
                    '{' | '[' | '(' | '<' =>
                        after.find(|(_, ch)| compl.contains(&(found, *ch))),
                    _ => None,
                }),
        }
        .is_some_and(|(idx, _)| {
            self.cursor.set(idx);
            true
        })
    }

    /// Moves the cursor to the beginning of the next word.
    fn goto_next_word(&mut self) {
        let mut chars = self.chars_after_cursor();

        let Some((_, cursor_ch)) = chars.next() else {
            self.cursor.set_to_max();
            return;
        };

        let cursor = IsIdentChar::new(cursor_ch);

        let Some((idx, next_ch)) = chars.find(|(_, ch)| cursor.xor(*ch)) else {
            self.cursor.set_to_max();
            return;
        };

        if !next_ch.is_whitespace() {
            self.cursor.set(idx);
            return;
        }

        match chars.find(|(_, ch)| !ch.is_whitespace()) {
            Some((non_space_idx, _)) => self.cursor.set(non_space_idx),
            None => self.cursor.set_to_max(),
        }
    }

    /// Updates the cursor position with a [`GoToAction`]
    ///
    /// Returns `true` if the action was successful.
    #[must_use]
    pub(super) fn update_cursor(&mut self, goto_action: GoToAction) -> bool {
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
                    .chars_after_cursor()
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
                    .chars_before_cursor_rev()
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
            GoToAction::NextGroup => return self.goto_next_group(),
        }
        true
    }
}
