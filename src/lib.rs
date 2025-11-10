#![doc = include_str!("../README.md")]
#![deny(
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo
)]
#![expect(clippy::doc_include_without_cfg, reason = "see issue #13918")]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "I want them all")]
#![expect(
    clippy::pub_use,
    clippy::field_scoped_visibility_modifiers,
    clippy::multiple_inherent_impl,
    reason = "better API"
)]
#![expect(
    clippy::implicit_return,
    clippy::mod_module_files,
    clippy::single_call_fn,
    clippy::question_mark_used,
    clippy::pattern_type_mismatch,
    clippy::enum_variant_names,
    clippy::missing_trait_methods,
    reason = "chosen style"
)]
#![expect(clippy::missing_inline_in_public_items, reason = "bad lint")]

/// Publicly interfaced buffer to handle vim keymap and modes
mod buffer;
/// Parser to convert a vim-like keymap string to a list of events
mod event_parser;

pub use buffer::{Buffer, Mode};
pub use crossterm;
pub use event_parser::{
    ChevronGroupError, EventParsingError, ModifiedKeyError, parse_events
};
