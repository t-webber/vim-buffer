/// Handles all the keypresses, dispatching them to the appropriate mode.
mod all;
/// Handles keypresses in insert mode
mod insert;
/// Handles keypresses in normal mode
mod normal;
/// Defines the types and traits to organise modes and how the process events.
mod traits;

pub use crate::buffer::mode::all::Mode;

#[cfg(test)]
mod tests;
