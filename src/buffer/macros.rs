#![expect(
    clippy::arbitrary_source_item_ordering,
    reason = "macro must be first"
)]

/// Creates an [`crate::buffer::mode::Actions`] from a list of actions.
macro_rules! actions {
    ($($action:expr),*) => {
        vec![$($action.into()),*].into()
    };
}

pub(super) use actions;
