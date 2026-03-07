/// API to interact with the [`Buffer`]
mod api;
/// Logic to hold the history of the buffer
mod history;
/// Handles the checks to delimitate a vim word.
mod is_indent;
/// Defines the actions that can be made on the buffer
mod keymaps;
/// Useful macros scoped with this module.
mod macros;
/// Handles the vim modes and the keypresses on those modes
mod mode;
/// Methods to update the [`Buffer`] with keymaps.
mod update;

pub use api::Buffer;
pub use mode::Mode;

#[cfg(test)]
mod tests;
