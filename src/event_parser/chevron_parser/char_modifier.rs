use crossterm::event::KeyModifiers;

/// Defines all the valid modifiers for modifier group keys.
#[derive(Copy, Clone)]
pub enum ChevronModifier {
    /// `'A'`
    Alt,
    /// `'D'`
    Command,
    /// `'C'`
    Control,
    /// `'M'`
    Meta,
    /// `'S'`
    Shift,
    /// `'T'`
    TrueMeta,
}
impl ChevronModifier {
    /// Creates a new [`ChevronModifier`] from a `char`, if possible.
    pub const fn maybe_from(ch: char) -> Option<Self> {
        Some(match ch {
            'C' => Self::Control,
            'M' => Self::Meta,
            'A' => Self::Alt,
            'T' => Self::TrueMeta,
            'D' => Self::Command,
            'S' => Self::Shift,
            _ => return None,
        })
    }

    /// Returns the `char` that created this [`ChevronModifier`] in the first
    /// place.
    pub const fn to_char(self) -> char {
        match self {
            Self::Control => 'C',
            Self::Meta => 'M',
            Self::Alt => 'A',
            Self::TrueMeta => 'T',
            Self::Command => 'D',
            Self::Shift => 'S',
        }
    }

    /// Returns the [`KeyModifiers`] that this [`ChevronModifier`]
    /// represents
    pub const fn to_modifier(self) -> KeyModifiers {
        match self {
            Self::Control => KeyModifiers::CONTROL,
            Self::Meta | Self::Alt => KeyModifiers::ALT,
            Self::TrueMeta => KeyModifiers::META,
            Self::Command => KeyModifiers::SUPER,
            Self::Shift => KeyModifiers::SHIFT,
        }
    }
}
