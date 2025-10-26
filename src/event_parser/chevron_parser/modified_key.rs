use crossterm::event::Event;

use crate::event_parser::EventParser;
use crate::event_parser::chevron_parser::non_empty_modifier::NonEmptyModifiers;


/// Parsing state to parse a chevron group for a modified key.
#[derive(Copy, Clone, Debug)]
pub enum ModifiedKeyParsingState {
    /// A non-modifier char has just been read.
    ReadChar(NonEmptyModifiers, char),
    /// A hyphen has just been read.
    ReadHyphen(NonEmptyModifiers),
    /// A modifier has just been read.
    ReadLetter(NonEmptyModifiers),
}


impl ModifiedKeyParsingState {
    /// Initialises a [`ModifiedKeyParsingState`] from a char that is found
    /// before a hyphen
    pub const fn try_from_prehyphen_char(
        ch: char,
    ) -> Result<Self, ModifiedKeyError> {
        if let Some(mods) = NonEmptyModifiers::maybe_from(ch) {
            Ok(Self::ReadHyphen(mods))
        } else {
            Err(ModifiedKeyError::InvalidModifier(ch))
        }
    }
}

impl EventParser for ModifiedKeyParsingState {
    type Error = ModifiedKeyError;

    fn parse_char(&mut self, ch: char) -> Result<Option<Event>, Self::Error> {
        match self {
            // ReadChar //
            Self::ReadChar(mods, last_char) if ch == '>' =>
                Ok(Some(mods.build_event_with_char(*last_char))),
            Self::ReadChar(..) => Err(Self::Error::ExpectedChevron { got: ch }),
            // ReadLetter //
            Self::ReadLetter(mods) if ch == '>' => Ok(Some(mods.build_event())),
            Self::ReadLetter(mods) if ch == '-' => {
                *self = Self::ReadHyphen(*mods);
                Ok(None)
            }
            Self::ReadLetter(_) =>
                Err(Self::Error::ExpectedChevronOrHyphen { got: ch }),
            // ReadHyphen //
            Self::ReadHyphen(_) if ch == '>' || ch == '-' =>
                Err(Self::Error::MissingChar),
            Self::ReadHyphen(mods) => {
                if mods.try_push_char(ch) {
                    *self = Self::ReadLetter(*mods);
                } else {
                    *self = Self::ReadChar(*mods, ch);
                }
                Ok(None)
            }
        }
    }
}


/// Errors that may occur whilst parsing a chevron group.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ModifiedKeyError {
    /// Invalid character in the current context: expected a `>`
    ExpectedChevron {
        /// But got this character
        got: char,
    },
    /// Invalid character in the current context: expected a `>` or a '-'
    ExpectedChevronOrHyphen {
        /// But got this character
        got: char,
    },
    /// Found an invalid modifier
    InvalidModifier(char),
    /// The chevron group is missing a char
    MissingChar,
    /// The chevron group is missing a modifier.
    MissingModifier,
}
