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
/// assert!(!x.xor('_'));
/// assert!(!x.xor(' '));
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Classifier<T>(T);

/// Trait for all checkers that can be used to classify and regroup chars
pub trait Checker: PartialEq + From<char> {}

impl<T: Checker> Classifier<T> {
    /// Creates a new [`IsIdentChar`] checker from the given char. This is the
    /// char that will be compared to the others given through
    /// [`Self::xor`].
    pub fn new(ch: char) -> Self {
        Self(T::from(ch))
    }

    /// Checks that first or second is ident valid, but not both.
    pub fn xor(self, other: char) -> bool {
        self.0 != T::from(other)
    }
}

/// Type of the char, to indicate to what group of characters it belongs
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IdentCharSpaceOrSymbol {
    /// Character valid for identifi$ers
    IdentChar,
    /// Whitespace
    Space,
    /// Non alphanumeric and non space character
    Symbol,
}

impl From<char> for IdentCharSpaceOrSymbol {
    fn from(value: char) -> Self {
        if matches!(value, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_') {
            Self::IdentChar
        } else if value.is_whitespace() {
            Self::Space
        } else {
            Self::Symbol
        }
    }
}

impl Checker for IdentCharSpaceOrSymbol {}

/// Groups chars into two categories: spaces and non spaces
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct SpaceOrNot(bool);

impl From<char> for SpaceOrNot {
    fn from(value: char) -> Self {
        Self(value.is_whitespace())
    }
}

impl Checker for SpaceOrNot {}

/// Compares on char to others, to check if they are in the same group. There
/// are 3 groups: ident chars, spaces, and symbols.
pub type IsIdentChar = Classifier<IdentCharSpaceOrSymbol>;

/// Compares on char to others, to check if they are in the same group. There
/// are 3 groups: ident chars, spaces, and symbols.
pub type IsSpace = Classifier<SpaceOrNot>;
