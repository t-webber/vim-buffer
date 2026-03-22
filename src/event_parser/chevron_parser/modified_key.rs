use crossterm::event::Event;

use crate::event_parser::EventParser;
use crate::event_parser::chevron_parser::non_empty_modifier::NonEmptyModifiers;
use crate::utils::array::Array;

/// List of chars that can be transformed into a keycode
///
/// # Examples
///
/// - Backspace is `['B', 'S', None]`
/// - Enter is `['C', 'R', None]`
/// - Escape is `['E', 's', 'c']`
pub type Chars = Array<char, 3>;

/// Parsing state to parse a chevron group for a modified key.
#[derive(Copy, Clone, Debug)]
pub enum ModifiedKeyParsingState {
    /// A non-modifier char has just been read.
    ReadChars(NonEmptyModifiers, Chars),
    /// A hyphen has just been read.
    ReadHyphen(NonEmptyModifiers),
    /// A modifier has just been read.
    ReadLetter(NonEmptyModifiers),
}

impl ModifiedKeyParsingState {
    /// Initialises a [`ModifiedKeyParsingState`] from a char that is found
    /// before a hyphen
    ///
    /// # Errors
    ///
    /// Returns an error if the char before the hyphen isn't a valid modifier
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

    #[expect(clippy::unwrap_used, reason = "pushing less than 5 items")]
    fn parse_char(&mut self, ch: char) -> Result<Option<Event>, Self::Error> {
        match (*self, ch) {
            // ReadChar //
            (Self::ReadChars(..), '-') =>
                Err(ModifiedKeyError::ExpectedChevron { got: '-' }),
            (Self::ReadChars(mods, chars), '>') =>
                mods.build_event_with_chars(chars).map(Some),
            (Self::ReadChars(mods, mut chars), _) =>
                if chars.push(ch) {
                    *self = Self::ReadChars(mods, chars);
                    Ok(None)
                } else {
                    Err(ModifiedKeyError::InvalidKeyNamePrefix(chars.concat()))
                },
            // ReadLetter //
            (Self::ReadLetter(mods), '>') =>
                mods.build_event_unchecked().map(Some),
            (Self::ReadLetter(mods), '-') => {
                *self = Self::ReadHyphen(mods);
                Ok(None)
            }
            (Self::ReadLetter(mut mods), _) => {
                let last = mods.pop_unchecked();
                *self = Self::ReadChars(
                    mods,
                    Array::maybe_from(&[last, ch]).unwrap(),
                );
                Ok(None)
            }
            // ReadHyphen //
            (Self::ReadHyphen(_), '>' | '-') => Err(Self::Error::MissingChar),
            (Self::ReadHyphen(mut mods), _) => {
                if mods.try_push_char(ch) {
                    *self = Self::ReadLetter(mods);
                } else {
                    *self = Self::ReadChars(
                        mods,
                        Array::maybe_from(&[ch]).unwrap(),
                    );
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
    /// No key with this name
    InvalidKeyName(String),
    /// No key starts this way as the prefix is too long
    InvalidKeyNamePrefix(String),
    /// Found an invalid modifier
    InvalidModifier(char),
    /// The chevron group is missing a char
    MissingChar,
    /// The chevron group is missing a modifier.
    MissingModifier,
    /// The given modifier is used multiple times, like in `<C-C-a>`
    SameModifierUsedTwice(char),
}
