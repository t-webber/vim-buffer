use crate::Mode;

/// Defines functions
macro_rules! operator_impl {
    ($($t:tt: $c:tt,)*) => {
        /// Char that represents this operator. It is the char needed to apply
        /// the operator to the whole line.
        pub(super) const fn as_char(self) -> char {
            match self {
                $(Self::$t => $c),*
            }
        }

        pub (super) const fn maybe_from(ch: char) -> Option<Self> {
            match ch {
                $($c => Some(Self::$t),)*
                _ => None
            }
        }
    };
}

/// Action to be done on the buffer
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Moves the cursor
    GoTo(GoToAction),
    /// Inserts a char at the current cursor
    InsertChar(char),
    /// Applies an operator motion
    Operator(Operator, OperatorScope),
    /// Pastes the content of the clipboard after the cursor
    PasteAfter,
    /// Pastes the content of the clipboard before the cursor
    PasteBefore,
    /// Undoes the last undo action
    Redo,
    /// Repeats the last action
    Repeat,
    /// Inserts the char if the cursor is at the end of the buffer, otherwise
    /// replace the current char with the given one.
    ReplaceOrInsert(char),
    /// Replaces the char under the cursor with
    ReplaceWith(char),
    /// Switches to a new mode
    SelectMode(Mode),
    /// Undoes the last edition
    Undo,
}

impl From<(Operator, OperatorScope)> for Action {
    fn from((op, scope): (Operator, OperatorScope)) -> Self {
        Self::Operator(op, scope)
    }
}

impl From<GoToAction> for Action {
    fn from(value: GoToAction) -> Self {
        Self::GoTo(value)
    }
}

impl From<Mode> for Action {
    fn from(value: Mode) -> Self {
        Self::SelectMode(value)
    }
}

/// Actions to move the cursor
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GoToAction {
    /// Beginning of line (column 0), reached with `0`
    BeginningOfLine,
    /// Move to the beginning of the previous WORD
    BeginningOfWORD,
    /// Move to the beginning of the previous word
    BeginningOfWord,
    /// End of line, like with `$` and `A`
    EndOfLine,
    /// Move to the end of the previous WORD, reached with `gE`
    EndOfPreviousWORD,
    /// Move to the end of the previous word, reached with `ge`
    EndOfPreviousWord,
    /// End of current or next WORD, reached with `E`
    EndWORD,
    /// End of current or next word, reached with `e`
    EndWord,
    /// First non space character, like with `I` and `^`
    FirstNonSpace,
    /// Move the cursor left by one character
    Left,
    /// Move the cursor right by one character, stopping at the last character
    ///
    /// Differs from [`Self::Right`] as it will never go beyond the last
    /// character.
    NextChar,
    /// Find next occurrence of char and place cursor on it
    NextOccurrenceOf(char),
    /// Move to the beginning of the next WORD
    NextWORD,
    /// Move to the beginning of the next word
    NextWord,
    /// Find previous occurrence of char and place cursor on it
    PreviousOccurrenceOf(char),
    /// Move the cursor right by one character
    Right,
}

/// Action that is pending for another keypress
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OPending {
    /// Pending action that only requires 1 character to form a goto action.
    ///
    /// Combinable with an [`Operator`], see [`Self::OperatorAction`].
    CombinablePending(CombinablePending),
    /// Applies a single char action to a motion.
    GoTo,
    /// Operator action, like `d`, `c`, `g~`
    Operator(Operator),
    /// Operator action that has the motion pending, like `df`, `cf`, `g~f`
    OperatorAction(Operator, CombinablePending),
    /// Replace one character
    ReplaceOne,
}

impl From<Operator> for OPending {
    fn from(value: Operator) -> Self {
        Self::Operator(value)
    }
}

impl From<CombinablePending> for OPending {
    fn from(value: CombinablePending) -> Self {
        Self::CombinablePending(value)
    }
}

/// Operator actions that can contain a motion and applied a function to that
/// motion (delete, yank, change, etc.)
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    /// Capitalise content of motion
    Capitalise,
    /// Change content of motion
    Change,
    /// Delete content of motion
    Delete,
    /// Lowers the case of content of motion
    LowerCase,
    /// Lowers capitals and capitalises lower case letters
    ToggleCase,
    /// Copies the content of motion in clipboard
    Yank,
}

impl Operator {
    operator_impl! {
        Capitalise: 'U',
        Change: 'c',
        Delete: 'd',
        LowerCase: 'u',
        ToggleCase: '~',
        Yank: 'y',
    }
}

/// Scope that an operator can be applied to, usually denotated by a list of
/// goto actions. It can also be applied to a whole line.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorScope {
    /// Apply the operator on simply those actions
    Goto(GoToAction, Option<GoToAction>),
    /// Apply operator on the whole line
    WholeLine,
}

impl From<GoToAction> for OperatorScope {
    fn from(value: GoToAction) -> Self {
        Self::Goto(value, None)
    }
}

/// Pending action that only requires 1 character to form a goto action.
///
/// Can be combined with an [`Operator`] (change, delete, toggle case, etc.),
/// see [`OPending::OperatorAction`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CombinablePending {
    /// Find next char that is equal to...
    FindNext,
    /// Find next char that is equal to... and decrement
    FindNextDecrement,
    /// Find previous char that is equal to...
    FindPrevious,
    /// Find previous char that is equal to... and increment
    FindPreviousIncrement,
}
