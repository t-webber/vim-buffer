/// Defines the types for the different modes.
mod all;
/// Handles keypresses in insert mode
mod insert;
/// Handles keypresses in normal mode
mod normal;
/// Handles keypresses in replace mode
mod replace;
/// Defines the types and traits to organise modes and how the process events.
mod traits;

pub use all::{BufferMode, Mode};
pub use traits::Actions;

#[cfg(test)]
mod tests;
