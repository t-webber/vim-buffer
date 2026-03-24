use core::mem::take;

use crate::buffer::is_indent::{IsIdentChar, IsSpace};
use crate::buffer::keymaps::{
    Delimitation, GoToAction, Operator, OperatorScope
};
use crate::utils::bounded_usize::BoundedUsize;
use crate::{Buffer, Mode};

impl Buffer {
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
            self.registers.insert(&old_content[min_cursor..max_cursor], true);
        }
        self.cursor = BoundedUsize::with_capacity(self.content.len());
        self.cursor.set(min_cursor);
        true
    }

    /// Returns the indices that bound the [`Delimitation`]
    ///
    /// It can include the bounds (meaning the delimiters like `(` or `}` will
    /// be included in the operator) or excluded.
    #[expect(clippy::arithmetic_side_effects, reason = "smaller than len")]
    fn get_delimitation_indices(
        &self,
        delimitation: Delimitation,
        include_bounds: bool,
    ) -> Option<(usize, usize)> {
        match delimitation {
            Delimitation::Group(open, close) if include_bounds => self
                .get_delimitation_indices_fn(
                    |ch| ch == open,
                    |ch| ch == close,
                    false,
                )
                .map(|(start, end)| {
                    (
                        start.saturating_sub(1),
                        if end == self.len() { end } else { end + 1 },
                    )
                }),
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
        if matches!(
            first,
            GoToAction::EndWord | GoToAction::EndWORD | GoToAction::NextGroup
        ) {
            Some((min, max.saturating_add(1).min(self.len())))
        } else {
            Some((min, max))
        }
    }

    /// Updates the buffer with an [`Operator`] action.
    pub(super) fn update_with_operator(
        &mut self,
        op: Operator,
        scope: OperatorScope,
    ) -> bool {
        let Some((min, max)) = (match scope {
            OperatorScope::WholeLine => Some((0, self.len())),
            OperatorScope::Goto(first, second) =>
                self.get_motion_delimination_indices(first, second),
            OperatorScope::Inner(delim) =>
                self.get_delimitation_indices(delim, false),
            OperatorScope::Around(delim) =>
                self.get_delimitation_indices(delim, true),
        }) else {
            return false;
        };
        self.cursor.set(min);
        let fun = match op {
            Operator::Delete => return self.delete(min, max),
            Operator::Yank => {
                #[expect(clippy::string_slice, reason = "utf8 not supported")]
                self.registers.insert(&self.content[min..max], false);
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
