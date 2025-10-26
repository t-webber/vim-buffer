use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::event_parser::chevron_parser::char_modifier::ChevronModifier;

/// Struct to represent a state where at least one key modifier has been found.
#[derive(Copy, Clone, Debug)]
pub struct NonEmptyModifiers {
    /// Last modifier to have been read.
    last_modifier: ChevronModifier,
    /// All read modifiers, except the last one.
    previous_modifiers: KeyModifiers,
}

impl NonEmptyModifiers {
    /// Builds an [`Event`] from a [`NonEmptyModifiers`]
    pub const fn build_event(self) -> Event {
        Self::build_event_from_char_modifiers(
            self.last_modifier.to_char(),
            self.previous_modifiers,
        )
    }

    /// Builds an [`Event`] from a `char` and [`KeyModifiers`].
    const fn build_event_from_char_modifiers(
        mut ch: char,
        mut modifiers: KeyModifiers,
    ) -> Event {
        #[expect(clippy::else_if_without_else, reason = "not needed")]
        if modifiers.contains(KeyModifiers::SHIFT) {
            ch = ch.to_ascii_uppercase();
        } else if ch.is_uppercase() {
            modifiers = modifiers.union(KeyModifiers::SHIFT);
        }
        Event::Key(KeyEvent::new(KeyCode::Char(ch), modifiers))
    }

    /// Builds an [`Event`] from a [`NonEmptyModifiers`] applied to a `char`.
    pub const fn build_event_with_char(&mut self, ch: char) -> Event {
        self.previous_modifiers =
            self.previous_modifiers.union(self.last_modifier.to_modifier());
        Self::build_event_from_char_modifiers(ch, self.previous_modifiers)
    }

    /// Creates a new [`NonEmptyModifiers`] from a `char`, if possible.
    pub const fn maybe_from(ch: char) -> Option<Self> {
        if let Some(last_modifier) = ChevronModifier::maybe_from(ch) {
            Some(Self { last_modifier, previous_modifiers: KeyModifiers::NONE })
        } else {
            None
        }
    }

    /// Tries to push a char into a [`NonEmptyModifiers`].
    ///
    /// The push is successful iff the char represents a valid modifier.
    ///
    /// Returns `true` iff the push was successful.
    pub const fn try_push_char(&mut self, ch: char) -> bool {
        if let Some(new_modifier) = ChevronModifier::maybe_from(ch) {
            self.previous_modifiers =
                self.previous_modifiers.union(self.last_modifier.to_modifier());
            self.last_modifier = new_modifier;
            true
        } else {
            false
        }
    }
}
