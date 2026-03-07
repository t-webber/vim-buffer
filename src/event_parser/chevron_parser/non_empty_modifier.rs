use core::result;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::ModifiedKeyError;
use crate::event_parser::chevron_parser::char_modifier::ChevronModifier;
use crate::event_parser::chevron_parser::modified_key::Chars;
use crate::event_parser::chevron_parser::named_key::build_named_key;
use crate::utils::array::Array;

/// Result type for [`NonEmptyModifiers`]
type Result<T, E = ModifiedKeyError> = result::Result<T, E>;

/// Struct to represent a state where at least one key modifier has been found.
#[derive(Copy, Clone, Debug)]
pub struct NonEmptyModifiers(Array<ChevronModifier, 5>);

impl NonEmptyModifiers {
    /// Builds an [`Event`] from a [`NonEmptyModifiers`]
    pub fn build_event(mut self) -> Result<Event> {
        self.0.pop().map_or(Err(ModifiedKeyError::MissingChar), |ch| {
            Ok(Self::fix_char_case(ch.to_char(), self.into_modifiers()))
        })
    }

    /// Builds an [`Event`] from a [`NonEmptyModifiers`] and a [`Chars`]
    pub fn build_event_with_chars(
        self,
        chars: Chars,
    ) -> Result<Event, ModifiedKeyError> {
        chars.as_lone().map_or_else(
            || {
                let key_name = chars.concat();
                build_named_key(&key_name).map_or(
                    Err(ModifiedKeyError::InvalidKeyName(key_name)),
                    |code| {
                        Ok(Event::Key(KeyEvent::new(
                            code,
                            self.into_modifiers(),
                        )))
                    },
                )
            },
            |ch| Ok(Self::fix_char_case(ch, self.into_modifiers())),
        )
    }

    /// Builds an [`Event`] from a `char` and [`KeyModifiers`].
    const fn fix_char_case(mut ch: char, mut modifiers: KeyModifiers) -> Event {
        #[expect(clippy::else_if_without_else, reason = "not needed")]
        if modifiers.contains(KeyModifiers::SHIFT) {
            ch = ch.to_ascii_uppercase();
        } else if ch.is_uppercase() {
            modifiers = modifiers.union(KeyModifiers::SHIFT);
        }
        Event::Key(KeyEvent::new(KeyCode::Char(ch), modifiers))
    }

    /// Returns the [`KeyModifiers`] associated to the [`NonEmptyModifiers`]
    fn into_modifiers(mut self) -> KeyModifiers {
        let mut modifiers = KeyModifiers::NONE;
        while let Some(modifier) = self.0.pop() {
            modifiers |= modifier.to_modifier();
        }
        modifiers
    }

    /// Creates a new [`NonEmptyModifiers`] from a `char`, if possible.
    pub const fn maybe_from(ch: char) -> Option<Self> {
        if let Some(last_modifier) = ChevronModifier::maybe_from(ch)
            && let Some(arr) = Array::maybe_from(&[last_modifier])
        {
            Some(Self(arr))
        } else {
            None
        }
    }

    /// Pops the last character
    ///
    /// # Panics
    ///
    /// If the data structure is empty
    #[expect(clippy::unwrap_used, reason = "caller's responsibility")]
    pub const fn pop_unchecked(&mut self) -> char {
        self.0.pop().unwrap().to_char()
    }

    /// Tries to push a char into a [`NonEmptyModifiers`].
    ///
    /// The push is successful iff the char represents a valid modifier.
    ///
    /// Returns `true` iff the push was successful.
    pub const fn try_push_char(&mut self, ch: char) -> bool {
        if let Some(new_modifier) = ChevronModifier::maybe_from(ch) {
            self.0.push(new_modifier)
        } else {
            false
        }
    }
}
