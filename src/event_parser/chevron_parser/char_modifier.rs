use crossterm::event::KeyModifiers;

/// Valid modifiers for inside a chevron group
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChevronModifier {
    /// Control
    Control,
    /// Meta
    Meta,
    /// Shift
    Shift,
}

impl ChevronModifier {
    /// Creates a new [`ChevronModifier`] from a `char`, if possible.
    pub const fn maybe_from(ch: char) -> Option<Self> {
        Some(match ch {
            'C' => Self::Control,
            'M' => Self::Meta,
            'S' => Self::Shift,
            _ => return None,
        })
    }

    /// Returns the `char` that created this [`ChevronModifier`] in the first
    /// place.
    pub const fn to_char(self) -> char {
        match self {
            Self::Control => 'c',
            Self::Meta => 'm',
            Self::Shift => 's',
        }
    }

    /// Returns the [`KeyModifiers`] that this [`ChevronModifier`]
    /// represents
    pub const fn to_modifier(self) -> KeyModifiers {
        match self {
            Self::Control => KeyModifiers::CONTROL,
            Self::Meta => KeyModifiers::META,
            Self::Shift => KeyModifiers::SHIFT,
        }
    }
}
