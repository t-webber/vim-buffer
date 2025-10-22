/// Action that is pending for another keypress
#[derive(Debug, PartialEq, Eq)]
pub enum OPending {
    /// Find next char
    FindNext,
}
