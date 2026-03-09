use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::buffer::keymaps::{
    Action, CombinablePending, GoToAction, OPending, Operator, OperatorScope
};
use crate::buffer::macros::actions;
use crate::buffer::mode::BufferMode;
use crate::buffer::mode::traits::{Actions, HandleKeyPress as _};

impl BufferMode {
    /// Handles opending event for [`CombinablePending`]
    const fn handle_combinable_opending_char_event(
        combinable_pending: CombinablePending,
        ch: char,
    ) -> (GoToAction, Option<GoToAction>) {
        match combinable_pending {
            CombinablePending::FindNext =>
                (GoToAction::NextOccurrenceOf(ch), None),
            CombinablePending::FindNextDecrement =>
                (GoToAction::NextOccurrenceOf(ch), Some(GoToAction::Left)),
            CombinablePending::FindPrevious =>
                (GoToAction::PreviousOccurrenceOf(ch), None),
            CombinablePending::FindPreviousIncrement =>
                (GoToAction::PreviousOccurrenceOf(ch), Some(GoToAction::Right)),
        }
    }

    /// Handle incoming terminal events on any kind.
    pub fn handle_event(
        self,
        event: &Event,
        pending: Option<OPending>,
    ) -> Actions {
        pending.map_or_else(
            || self.handle_non_opending_event(event),
            |old_pending| self.handle_opending_event(old_pending, event),
        )
    }

    /// Handles the terminal events when not in [`OPending`] mode
    fn handle_non_opending_event(self, event: &Event) -> Actions {
        event.as_key_press_event().map_or_else(
            Actions::default,
            |mut key_event| {
                fix_shift_modifier(&mut key_event);
                match key_event.modifiers {
                    KeyModifiers::NONE =>
                        self.handle_blank_key_press(key_event.code),
                    KeyModifiers::CONTROL =>
                        self.handle_ctrl_key_press(key_event.code),
                    KeyModifiers::SHIFT =>
                        self.handle_shift_key_press(key_event.code),
                    _ => Actions::default(),
                }
            },
        )
    }

    /// Handle a keypress when an [`OPending`] is in progress and waiting for
    /// keys.
    fn handle_opending_event(
        self,
        opending: OPending,
        event: &Event,
    ) -> Actions {
        if let Some(key_event) = event.as_key_press_event()
            && let KeyCode::Char(ch) = key_event.code
        {
            match opending {
                OPending::GoTo if ch == 'e' =>
                    GoToAction::EndOfPreviousWord.into(),
                OPending::GoTo if ch == 'E' =>
                    GoToAction::EndOfPreviousWORD.into(),
                OPending::GoTo =>
                    Operator::maybe_from(ch).map(Into::into).unwrap_or_default(),
                OPending::CombinablePending(action) => {
                    let (first, maybe_second) =
                        Self::handle_combinable_opending_char_event(action, ch);
                    maybe_second.map_or_else(
                        || first.into(),
                        |second| actions![first, second],
                    )
                }
                OPending::ReplaceOne => Action::ReplaceWith(ch).into(),
                OPending::OperatorAction(op, combinable) =>
                    Self::handle_operator_action(op, combinable, ch),
                OPending::Operator(op) => self.handle_operator(event, op, ch),
            }
        } else {
            Actions::default()
        }
    }

    /// Handle operator events (`d`, `c`, etc.)
    fn handle_operator(self, event: &Event, op: Operator, ch: char) -> Actions {
        if op.as_char() == ch {
            return actions![(op, OperatorScope::WholeLine)];
        }
        match self.handle_non_opending_event(event) {
            Actions::List(list) =>
                if let &[list_action] = list.as_slice()
                    && let Action::GoTo(goto) = list_action
                {
                    actions![(op, goto.into())]
                } else {
                    list.into()
                },
            Actions::OPending(OPending::CombinablePending(combinable)) =>
                OPending::OperatorAction(op, combinable).into(),
            Actions::OPending(_) => Actions::default(),
        }
    }

    /// Handle operator action events (`dw`, `cw`, etc.)
    fn handle_operator_action(
        op: Operator,
        action: CombinablePending,
        ch: char,
    ) -> Actions {
        let (first, maybe_second) =
            Self::handle_combinable_opending_char_event(action, ch);
        let second = match action {
            CombinablePending::FindNext => Some(GoToAction::Right),
            CombinablePending::FindNextDecrement => None,
            CombinablePending::FindPrevious
            | CombinablePending::FindPreviousIncrement => maybe_second,
        };
        actions![(op, OperatorScope::Goto(first, second))]
    }
}

/// Adds [`KeyModifiers::SHIFT`] if the event is a capital char, and capitalises
/// the char if the modifiers contain shift.
const fn fix_shift_modifier(key_event: &mut KeyEvent) {
    #[expect(clippy::else_if_without_else, reason = "checked")]
    if let KeyCode::Char(ch) = &mut key_event.code {
        if ch.is_ascii_uppercase() {
            key_event.modifiers =
                key_event.modifiers.union(KeyModifiers::SHIFT);
        } else if key_event.modifiers.contains(KeyModifiers::SHIFT) {
            *ch = ch.to_ascii_uppercase();
        }
    }
}
