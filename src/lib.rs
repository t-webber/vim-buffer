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
    reason = "prevent a breaking change after refactoring the crate structure"
)]
#![expect(
    clippy::implicit_return,
    clippy::mod_module_files,
    clippy::question_mark_used,
    reason = "chosen style"
)]
#![cfg_attr(test, expect(clippy::single_call_fn, reason = "bad lint"))]
#![allow(dead_code, reason = "dev in progress")]

/// Defines the actions that can be made on the buffer
mod action;
/// Handles the vim modes and the keypresses on those modes
mod mode;

pub use mode::Mode;
