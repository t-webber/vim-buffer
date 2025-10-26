/// Defines the valid modifiers that can be used within a chevron group.
mod char_modifier;
/// Defines the state to parse the chevron groups that represent modified keys
mod modified_key;
/// Defines the parser for simple keys inside chevron groups, like `<Esc>` and
/// `<CR>`.
mod named_key;
/// Defines logic to push into the modifiers, and get back the last one if
/// needed.
mod non_empty_modifier;

use crossterm::event::{Event, KeyEvent};

use crate::event_parser::EventParser;
pub use crate::event_parser::chevron_parser::modified_key::ModifiedKeyError;
use crate::event_parser::chevron_parser::modified_key::ModifiedKeyParsingState;
use crate::event_parser::chevron_parser::named_key::{
    LENGTHS, build_named_key
};

/// Parses one chevron group.
///
/// Chevron groups are used to denote keys with keymodifiers.
pub struct ChevronGroupParser(ChevronGroupParsingState);

impl ChevronGroupParser {
    /// Returns a default [`ChevronGroupParser`] for a new chevron group.
    pub const fn new() -> Self {
        Self(ChevronGroupParsingState::None)
    }
}

impl EventParser for ChevronGroupParser {
    type Error = ChevronGroupError;

    fn parse_char(&mut self, ch: char) -> Result<Option<Event>, Self::Error> {
        self.0.parse_char(ch)
    }
}

/// Parsing state to parse a chevron group.
#[derive(Copy, Clone, Debug)]
pub enum ChevronGroupParsingState {
    /// Key that has modifiers
    ModifiedKey(ModifiedKeyParsingState),
    /// Named key, like `<CR>` or `<Tab>`
    NamedKey([u8; LENGTHS.max], usize),
    /// Read only `<`
    None,
    /// Read one char
    ///
    /// We need 2 chars to decide the type
    Unknown(char),
}

const _: () = assert!(LENGTHS.min >= 2, "Reading 2 character before parsing");

impl ChevronGroupParsingState {
    /// The group is now certain to be a keycode
    pub fn evolve_to_named_key(
        &mut self,
        previous: char,
        next: char,
    ) -> Result<(), ChevronGroupError> {
        let mut buf = [0; LENGTHS.max];
        if let Ok(first) = u8::try_from(previous)
            && let Ok(second) = u8::try_from(next)
        {
            buf[0] = first;
            buf[1] = second;
            *self = Self::NamedKey(buf, 2);
            Ok(())
        } else {
            Err(ChevronGroupError::InvalidNamedKey)
        }
    }
}

impl EventParser for ChevronGroupParsingState {
    type Error = ChevronGroupError;

    #[expect(
        clippy::arithmetic_side_effects,
        reason = "smaller than LENGTHS.max: can't overflow"
    )]
    #[expect(clippy::indexing_slicing, reason = "explicit check")]
    fn parse_char(&mut self, ch: char) -> Result<Option<Event>, Self::Error> {
        match self {
            // NONE
            Self::None if ch.is_ascii_alphabetic() => {
                *self = Self::Unknown(ch);
                Ok(None)
            }
            Self::None => Err(Self::Error::ExpectedLetter { got: ch }),
            // Unknown
            &mut Self::Unknown(previous) if ch == '-' => {
                *self = Self::ModifiedKey(
                    ModifiedKeyParsingState::try_from_prehyphen_char(previous)
                        .map_err(Self::Error::ModifiedKey)?,
                );
                Ok(None)
            }
            &mut Self::Unknown(_) if ch == '>' =>
                Err(Self::Error::SingleCharGroup),
            &mut Self::Unknown(previous) =>
                self.evolve_to_named_key(previous, ch).map(|()| None),
            // Modified key
            Self::ModifiedKey(modified_key) =>
                modified_key.parse_char(ch).map_err(Self::Error::ModifiedKey),
            // Named key
            Self::NamedKey(buf, len) if ch == '>' =>
                build_named_key(&buf[0..*len])
                    .map_or(Err(Self::Error::InvalidNamedKey), |code| {
                        Ok(Some(Event::Key(KeyEvent::from(code))))
                    }),
            Self::NamedKey(buf, len) if *len < LENGTHS.max => u8::try_from(ch)
                .map_or(Err(Self::Error::InvalidNamedKey), |byte| {
                    buf[*len] = byte;
                    *len += 1;
                    Ok(None)
                }),
            Self::NamedKey(..) => Err(Self::Error::InvalidNamedKey),
        }
    }
}

/// Errors that may occur while parsing a chevron group
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum ChevronGroupError {
    /// Expected to found a letter at the beginning of the group
    ExpectedLetter {
        /// found this instead
        got: char,
    },
    /// Error for a named key
    InvalidNamedKey,
    /// Error for a modified key
    ModifiedKey(ModifiedKeyError),
    /// The group consists of a single char
    SingleCharGroup,
}
