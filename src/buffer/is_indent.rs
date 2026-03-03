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
pub struct IsIdentChar(bool);

impl IsIdentChar {
    /// Returns `true` if the given char is valid for an identifier
    const fn is(ch: char) -> bool {
        matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_')
    }

    /// Creates a new [`IsIdentChar`] checker from the given char. This is the
    /// char that will be compared to the others given through
    /// [`Self::xor`].
    pub const fn new(ch: char) -> Self {
        Self(Self::is(ch))
    }

    /// Checks that first or second is ident valid, but not both.
    pub const fn xor(self, other: char) -> bool {
        self.0 ^ Self::is(other)
    }
}
