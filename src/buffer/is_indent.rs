/// Compares one char to others, to find out what bounds the current word.
///
/// In vim, a word is either formed of alphanumeric characters and underscores,
/// or not, so `('"-` is a word and `ab_de` is a word but not `ab_de"()` (that's
/// 2 words).
///
/// # Examples
///
/// ```ignore
/// let x = IsIdentChar::new('a');
/// assert!(x.xor('.'));
/// assert!(!x.xor('.'));
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IsIdentChar(Type);

impl IsIdentChar {
    /// Returns `true` if the given char is valid for an identifier
    const fn is(ch: char) -> Type {
        if matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_') {
            Type::AlphaNum
        } else if ch.is_whitespace() {
            Type::Space
        } else {
            Type::Symbol
        }
    }

    /// Creates a new [`IsIdentChar`] checker from the given char. This is the
    /// char that will be compared to the others given through
    /// [`Self::xor`].
    pub const fn new(ch: char) -> Self {
        Self(Self::is(ch))
    }

    /// Checks that first or second is ident valid, but not both.
    pub fn xor(self, other: char) -> bool {
        self.0 != Self::is(other)
    }
}

/// Type of the char, to indicate to what group of characters it belongs
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Type {
    /// Alphanumeric character
    AlphaNum,
    /// Whitespace
    Space,
    /// Non alphanumeric and non space character
    Symbol,
}
