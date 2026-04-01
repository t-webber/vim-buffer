use crossterm::event::{Event, KeyCode};

use crate::buffer::keymaps::{
    Action, CombinablePending, Delimitation, GoToAction, OPending, Operator, OperatorPendingScope, OperatorScope
};
use crate::buffer::macros::actions;
use crate::buffer::mode::all::Mode;
use crate::buffer::mode::traits::{Actions, HandleKeyPress};

/// Struct to handle keypresses in normal mode
#[expect(clippy::arbitrary_source_item_ordering, reason = "timeline")]
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub enum Normal {
    /// Normal mode but no operations are pending
    #[default]
    None,
    /// A digit was pressed, and is pending for an action or a register.
    PreNum(usize),
    /// Register pending on the action
    Register(Option<usize>, Option<char>),
    /// A digit was pressed, and is pending for an action.
    MidNum(Option<usize>, Option<char>, usize),
    /// Pending keymaps that wait for further keypresses
    Pending(Option<usize>, Option<char>, Option<usize>, OPending),
    /// A digit was pressed in the middle of pending keymaps
    PostNum(Option<usize>, Option<char>, Option<usize>, OPending, usize),
}

impl Normal {
    /// Returns a default [`Normal`]
    pub const fn new() -> Self {
        Self::None
    }

    /// Triggers a new number pending.
    ///
    /// # Panics
    ///
    /// If `ch` isn't an ascii digit.
    #[expect(
        clippy::as_conversions,
        clippy::arithmetic_side_effects,
        reason = "in bounds"
    )]
    fn num(&mut self, ch: char) -> Actions {
        debug_assert!(ch.is_ascii_digit(), "invalid arguments");
        let num = ((ch as u32) - ('0' as u32)) as usize;
        *self = match *self {
            Self::None => Self::PreNum(num),
            Self::PreNum(old) =>
                Self::PreNum(old.saturating_mul(10).saturating_add(num)),
            Self::Register(pre, reg) => Self::MidNum(pre, reg, num),
            Self::MidNum(pre, reg, old) => Self::MidNum(
                pre,
                reg,
                old.saturating_mul(10).saturating_add(num),
            ),
            Self::Pending(pre, reg, mid, opending) =>
                Self::PostNum(pre, reg, mid, opending, num),
            Self::PostNum(pre, reg, mid, pend, old) => Self::PostNum(
                pre,
                reg,
                mid,
                pend,
                old.saturating_mul(10).saturating_add(num),
            ),
        };
        Actions::None
    }

    /// Triggers a new pending action, or continues building the previous one.
    fn pend(&mut self, pending: impl Copy + Into<OPending>) -> Actions {
        *self = match *self {
            Self::None => Self::Pending(None, None, None, pending.into()),
            Self::PreNum(num) =>
                Self::Pending(None, None, Some(num), pending.into()),
            Self::Register(pre, reg) =>
                Self::Pending(pre, reg, None, pending.into()),
            Self::MidNum(pre, reg, mid) =>
                Self::Pending(pre, reg, Some(mid), pending.into()),
            Self::Pending(pre, reg, mid, _) =>
                Self::Pending(pre, reg, mid, pending.into()),
            Self::PostNum(pre, reg, mid, _, post) =>
                Self::PostNum(pre, reg, mid, pending.into(), post), /* TODO: investigate this */
        };
        Actions::None
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
            Actions::None => {
                if let Self::Pending(
                    ..,
                    OPending::CombinablePending(combinable),
                ) = normal
                {
                    self.pend(OPending::OperatorAction(op, combinable))
                } else {
                    Actions::Unsupported
                }
            }
            Actions::List(list, _) =>
                if let &[list_action] = list.as_slice()
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

    /// Handles a keypress when [`Normal`] is [`Normal::Pending`]
    fn handle_pending(
        &mut self,
        event: Event,
        has_num: bool,
        opending: OPending,
    ) -> Actions {
        if let Some(key_event) = event.as_key_press_event()
            && let KeyCode::Char(ch) = key_event.code
        {
            if (matches!(ch, '1'..='9') || (ch == '0' && has_num))
                && matches!(opending, OPending::Operator(_, None))
            {
                self.num(ch)
            } else {
                self.handle_opending_event(opending, event, ch)
            }
        } else {
            Actions::Unsupported
        }
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
            KeyCode::Char('0')
                if !matches!(
                    self,
                    Self::PreNum(..) | Self::MidNum(..) | Self::PostNum(..)
                ) =>
                GoToAction::BeginningOfLine.into(),
            KeyCode::Char(ch @ '0'..='9') => self.num(ch),
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
        let ch = event.as_key_press_event().and_then(|ev| ev.code.as_char());
        let actions = match *self {
            Self::None if ch == Some('"') => {
                *self = Self::Register(None, None);
                Actions::None
            }
            Self::PreNum(pre) if ch == Some('"') => {
                *self = Self::Register(Some(pre), None);
                Actions::None
            }
            Self::None => self.default_handle_key(event),
            Self::PreNum(num) => self.default_handle_key(event).repeat(num),
            Self::Register(pre, None) if ch.is_some() => {
                *self = Self::Register(pre, ch);
                Actions::None
            }
            Self::Register(num, reg @ Some(_)) => self
                .default_handle_key(event)
                .with_reg(reg)
                .repeat(num.unwrap_or(1)),
            Self::Register(..) => Actions::Unsupported,
            Self::MidNum(pre, reg, mid) => self
                .default_handle_key(event)
                .repeat(pre.unwrap_or(1).saturating_mul(mid))
                .with_reg(reg),
            Self::Pending(pre, reg, mid, opending) => self
                .handle_pending(event, mid.is_some(), opending) //TODO: number after
                                                                //pending but before
                                                                //here
                .with_reg(reg)
                .repeat(pre.unwrap_or(1).saturating_mul(mid.unwrap_or(1))),
            Self::PostNum(pre, reg, mid, opending, post) =>
                self.handle_pending(event, true, opending).with_reg(reg).repeat(
                    pre.unwrap_or(1)
                        .saturating_mul(mid.unwrap_or(1))
                        .saturating_mul(post),
                ),
        };
        if actions != Actions::None {
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
