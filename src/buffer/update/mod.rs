/// Handles updates that modify only the cursor position
mod goto;
/// Handle history actions (save, undo, redo)
mod history;
/// Handles operator actions, like `dfx` and `ci(`
mod operator;
/// Useful utils to interact with the [`Buffer`]
mod utils;

use core::mem::take;

use crossterm::event::Event;

use crate::Buffer;
use crate::buffer::keymaps::Action;
use crate::buffer::mode::Actions;
use crate::event_parser::{EventParsingError, parse_events};

impl Buffer {
    /// Paste the copied content after the cursor
    #[must_use]
    #[expect(clippy::arithmetic_side_effects, reason = "smaller than len")]
    fn paste_after(&mut self) -> bool {
        let pos = if self.as_cursor() >= self.len() {
            self.len()
        } else {
            self.as_cursor() + 1
        };
        self.registers.get().is_some_and(|clip| {
            self.content.insert_str(pos, clip);
            true
        })
    }

    /// Remove the character under the current cursor and replace it by
    /// another one.
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
    /// buffer.update(Event::Key(KeyEvent::from(KeyCode::Char('i'))));
    /// for ch in "hello".chars() {
    ///     buffer.update(Event::Key(KeyEvent::from(KeyCode::Char('h'))));
    /// }
    /// buffer.update(Event::Key(KeyEvent::from(KeyCode::Esc)));
    /// buffer.update(Event::Key(KeyEvent::from(KeyCode::Char('^'))));
    /// buffer.update(Event::Key(KeyEvent::from(KeyCode::Char('s'))));
    /// buffer.update(Event::Key(KeyEvent::from(KeyCode::Char('H'))));
    /// ```
    pub fn update(&mut self, event: Event) -> bool {
        let success = self.update_no_save(event);
        self.save_to_history();
        success
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
            self.update(event);
        }
        Ok(())
    }

    /// Same as [`Self::update`] but without updating the history.
    pub fn update_no_save(&mut self, event: Event) -> bool {
        match self.mode.handle_event(event) {
            Actions::Unsupported => false,
            Actions::List(list) =>
                list.iter().all(|action| self.update_once(*action)) && {
                    self.last_action.update(list, self.as_mode());
                    true
                },
        }
    }

    /// Updates the buffer with one [`Action`]
    ///
    /// Returns `true` iff the update was successful.
    #[must_use]
    pub(crate) fn update_once(&mut self, action: Action) -> bool {
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
            Action::PasteAfter => return self.paste_after(),
            Action::PasteBefore =>
                if let Some(clip) = self.registers.get() {
                    let idx = self.as_cursor().saturating_sub(1);
                    self.content.insert_str(idx, clip);
                } else {
                    return false;
                },
            Action::Repeat => {
                let last = take(&mut self.last_action);
                let ok = last.perform(self);
                self.last_action = last;
                return ok;
            }
        }
        true
    }
}
