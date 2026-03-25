use crossterm::event::{Event, KeyCode};

use crate::buffer::keymaps::{
    Action, CombinablePending, Delimitation, GoToAction, OPending, Operator, OperatorPendingScope, OperatorScope
};
use crate::buffer::macros::actions;
use crate::buffer::mode::all::Mode;
use crate::buffer::mode::traits::{Actions, HandleKeyPress};

/// Struct to handle keypresses in normal mode
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub enum Normal {
    /// Normal mode but no operations are pending
    #[default]
    None,
    /// A digit was pressed, and is pending for a action.
    NumberPending(usize),
    /// Pending keymaps that wait for further keypresses
    Pending(Option<usize>, OPending),
}

impl Normal {
    /// Returns a default [`Normal`]
    pub const fn new() -> Self {
        Self::None
    }

    /// Triggers a new number pending.
    const fn num(&mut self, num: usize) -> Actions {
        if let Self::NumberPending(old) = self {
            *old = old.saturating_mul(10).saturating_add(num);
        } else {
            *self = Self::NumberPending(num);
        }
        Actions::NONE
    }

    /// Triggers a new pending action.
    fn pend(&mut self, pending: impl Into<OPending>) -> Actions {
        *self = if let Self::NumberPending(num) = *self {
            Self::Pending(Some(num), pending.into())
        } else {
            Self::Pending(None, pending.into())
        };
        Actions::NONE
    }
}

impl Normal {
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

    /// Handle a keypress when an [`OPending`] is in progress and waiting for
    /// keys.
    fn handle_opending_event(
        &mut self,
        opending: OPending,
        event: Event,
        ch: char,
    ) -> Actions {
        match opending {
            OPending::GoTo if ch == 'e' => GoToAction::EndOfPreviousWord.into(),
            OPending::GoTo if ch == 'E' => GoToAction::EndOfPreviousWORD.into(),
            OPending::GoTo => Operator::maybe_from(ch)
                .map_or(Actions::Unsupported, |op| self.pend(op)),
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
            OPending::Operator(op, None) => {
                if let Some(scope) = OperatorPendingScope::maybe_from(ch) {
                    self.pend(OPending::Operator(op, Some(scope)))
                } else if ch == op.as_char() {
                    actions![(op, OperatorScope::WholeLine)]
                } else {
                    self.handle_operator(event, op)
                }
            }
            OPending::Operator(op, Some(scope)) => Delimitation::maybe_from(ch)
                .map_or(Actions::Unsupported, |delim| {
                    actions![(op, scope, delim)]
                }),
        }
    }

    /// Handle operator events (`d`, `c`, etc.)
    fn handle_operator(&mut self, event: Event, op: Operator) -> Actions {
        let mut normal = Self::default();
        match normal.handle_key(event) {
            Actions::List(list) =>
                if list.is_empty()
                    && let Self::Pending(
                        _,
                        OPending::CombinablePending(combinable),
                    ) = normal
                {
                    self.pend(OPending::OperatorAction(op, combinable))
                } else if let &[list_action] = list.as_slice()
                    && let Action::GoTo(goto) = list_action
                {
                    actions![(op, goto.into())]
                } else {
                    Actions::Unsupported
                },
            Actions::Unsupported => Actions::Unsupported,
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

#[expect(clippy::wildcard_enum_match_arm, reason = "only support a few")]
impl HandleKeyPress for Normal {
    fn handle_blank_key_press(&mut self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Char('$') => GoToAction::EndOfLine.into(),
            KeyCode::Char('.') => Action::Repeat.into(),
            KeyCode::Char('^') => GoToAction::FirstNonSpace.into(),
            KeyCode::Char('a') => actions![GoToAction::Right, Mode::Insert],
            KeyCode::Char('b') => GoToAction::BeginningOfWord.into(),
            KeyCode::Char('c') => self.pend(Operator::Change),
            KeyCode::Char('d') => self.pend(Operator::Delete),
            KeyCode::Char('e') => GoToAction::EndWord.into(),
            KeyCode::Char('f') => self.pend(CombinablePending::FindNext),
            KeyCode::Char('g') => self.pend(OPending::GoTo),
            KeyCode::Char('h') | KeyCode::Backspace | KeyCode::Left =>
                GoToAction::Left.into(),
            KeyCode::Char('i') => Mode::Insert.into(),
            KeyCode::Char('l') | KeyCode::Right => GoToAction::NextChar.into(),
            KeyCode::Char('x') => actions![
                (Operator::Delete, GoToAction::Right.into()),
                GoToAction::Right,
                GoToAction::Left
            ],
            KeyCode::Char('p') => Action::PasteAfter.into(),
            KeyCode::Char('r') => self.pend(OPending::ReplaceOne),
            KeyCode::Char('s') => actions![
                (Operator::Delete, GoToAction::Right.into()),
                Mode::Insert
            ],
            KeyCode::Char('t') =>
                self.pend(CombinablePending::FindNextDecrement),
            KeyCode::Char('u') => Action::Undo.into(),
            KeyCode::Char('w') => GoToAction::NextWord.into(),
            KeyCode::Char('y') => self.pend(Operator::Yank),
            KeyCode::Char('%') => GoToAction::NextGroup.into(),
            KeyCode::Char('~') => actions![
                (Operator::ToggleCase, GoToAction::Right.into()),
                GoToAction::NextChar
            ],
            KeyCode::Char('0') if !matches!(self, Self::NumberPending(_)) =>
                GoToAction::BeginningOfLine.into(),
            KeyCode::Char(ch @ '0'..='9') =>
                usize::try_from(u32::from(ch).saturating_sub(u32::from('0')))
                    .map_or(Actions::Unsupported, |num| self.num(num)),
            _ => Actions::Unsupported,
        }
    }

    fn handle_ctrl_key_press(&mut self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Char('r') => Action::Redo.into(),
            _ => Actions::Unsupported,
        }
    }

    fn handle_key(&mut self, event: Event) -> Actions {
        let actions = match *self {
            Self::None => self.default_handle_key(event),
            Self::NumberPending(num) =>
                self.default_handle_key(event).repeat(num),
            Self::Pending(num, opending) =>
                if let Some(key_event) = event.as_key_press_event()
                    && let KeyCode::Char(ch) = key_event.code
                {
                    self.handle_opending_event(opending, event, ch)
                        .repeat(num.unwrap_or(1))
                } else {
                    Actions::Unsupported
                },
        };
        if actions != Actions::NONE {
            *self = Self::None;
        }
        actions
    }

    fn handle_shift_key_press(&mut self, code: KeyCode) -> Actions {
        match code {
            KeyCode::Char('A') => actions![GoToAction::EndOfLine, Mode::Insert],
            KeyCode::Char('B') => GoToAction::BeginningOfWORD.into(),
            KeyCode::Char('C') => actions![
                (Operator::Delete, GoToAction::EndOfLine.into()),
                Mode::Insert
            ],
            KeyCode::Char('D') =>
                vec![(Operator::Delete, GoToAction::EndOfLine.into()).into()]
                    .into(),
            KeyCode::Char('E') => GoToAction::EndWORD.into(),
            KeyCode::Char('F') => self.pend(CombinablePending::FindPrevious),
            KeyCode::Char('I') =>
                actions![GoToAction::FirstNonSpace, Mode::Insert],
            KeyCode::Char('P') => Action::PasteBefore.into(),
            KeyCode::Char('R') => Mode::Replace.into(),
            KeyCode::Char('S') => actions![
                (Operator::Delete, OperatorScope::WholeLine),
                Mode::Insert
            ],
            KeyCode::Char('T') =>
                self.pend(CombinablePending::FindPreviousIncrement),
            KeyCode::Char('W') => GoToAction::NextWORD.into(),
            KeyCode::Char('X') => actions![
                GoToAction::Left,
                (Operator::Delete, GoToAction::Right.into())
            ],
            KeyCode::Char('Y') =>
                actions![(Operator::Yank, GoToAction::EndOfLine.into())],
            _ => Actions::Unsupported,
        }
    }
}
