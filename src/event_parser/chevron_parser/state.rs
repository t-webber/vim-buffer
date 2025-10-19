use crossterm::event::Event;

use crate::event_parser::EventParser;
use crate::event_parser::chevron_parser::ChevronParsingError;
use crate::event_parser::chevron_parser::non_empty_modifier::NonEmptyModifiers;


/// Parsing state to parse a chevron group.
#[derive(Copy, Clone)]
pub enum ChevronGroupParsingState {
    /// The group was just opened, nothing was read yet.
    None,
    /// A non-modifier char has just been read.
    ReadChar(NonEmptyModifiers, char),
    /// A hypen has just been read.
    ReadHyphen(NonEmptyModifiers),
    /// A modifier has just been read.
    ReadModifier(NonEmptyModifiers),
}


impl EventParser for ChevronGroupParsingState {
    type Error = ChevronParsingError;

    fn parse_char(&mut self, ch: char) -> Result<Option<Event>, Self::Error> {
        match self {
            // None //
            Self::None => {
                if let Some(mods) = NonEmptyModifiers::maybe_from(ch) {
                    *self = Self::ReadModifier(mods);
                } else {
                    return Err(ChevronParsingError::MissingModifier);
                }
            }
            // ReadChar //
            Self::ReadChar(mods, last_char) if ch == '>' =>
                return Ok(Some(mods.build_event_with_char(*last_char))),
            Self::ReadChar(..) =>
                return Err(ChevronParsingError::ExpectedChevron { got: ch }),
            // ReadModifier //
            Self::ReadModifier(mods) if ch == '>' =>
                return match mods.build_event() {
                    Ok(event) => Ok(Some(event)),
                    Err(err) => Err(err),
                },
            Self::ReadModifier(mods) if ch == '-' =>
                *self = Self::ReadHyphen(*mods),
            Self::ReadModifier(_) =>
                return Err(ChevronParsingError::ExpectedChevronOrHyphen {
                    got: ch,
                }),
            // ReadHyphen //
            Self::ReadHyphen(_) if ch == '>' || ch == '-' =>
                return Err(ChevronParsingError::MissingChar),
            Self::ReadHyphen(mods) =>
                if mods.try_push_char(ch) {
                    *self = Self::ReadModifier(*mods);
                } else {
                    *self = Self::ReadChar(*mods, ch);
                },
        }
        Ok(None)
    }
}
