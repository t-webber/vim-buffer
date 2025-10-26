/// API to interact with the [`Buffer`]
mod api;
/// Defines a bounded usize newtype, to safely increment, decrement a cursor.
mod bounded_usize;
/// Defines the actions that can be made on the buffer
mod keymaps;
/// Handles the vim modes and the keypresses on those modes
mod mode;
/// Methods to update the [`Buffer`] with keymaps.
mod update;

pub use api::Buffer;
pub use mode::Mode;
