# vim-buffer

![Clippy](https://github.com/t-webber/vim-buffer/actions/workflows/clippy.yml/badge.svg?branch=main)
![Build](https://github.com/t-webber/vim-buffer/actions/workflows/build.yml/badge.svg?branch=main)
![Tests](https://github.com/t-webber/vim-buffer/actions/workflows/tests.yml/badge.svg?branch=main)
![Rustdoc](https://github.com/t-webber/vim-buffer/actions/workflows/rustdoc.yml/badge.svg?branch=main)
![Rusfmt](https://github.com/t-webber/vim-buffer/actions/workflows/rustfmt.yml/badge.svg?branch=main)
![Coverage](https://github.com/t-webber/vim-buffer/actions/workflows/coverage.yml/badge.svg?branch=main)
![Taplo](https://github.com/t-webber/vim-buffer/actions/workflows/taplo.yml/badge.svg?branch=main)
![Spelling](https://github.com/t-webber/vim-buffer/actions/workflows/spelling.yml/badge.svg?branch=main)

[![github](https://img.shields.io/badge/GitHub-t--webber/vim--buffer-blue?logo=GitHub)](https://github.com/t-webber/vim-buffer)
[![license](https://img.shields.io/badge/Licence-MIT%20or%20Apache%202.0-darkgreen)](https://github.com/t-webber/vim-buffer?tab=MIT-2-ov-file)
[![coverage](https://img.shields.io/badge/Coverage-100%25-purple)](https://github.com/t-webber/vim-buffer/actions/workflows/nightly.yml)
[![rust-edition](https://img.shields.io/badge/Rust--edition-2024-darkred?logo=Rust)](https://doc.rust-lang.org/stable/edition-guide/rust-2024/)

A buffer that listens for vim keymaps.

## Development status

The buffer only supports a single line for now.

Refer to [keys.rs](tests/keys.rs) to see the list of supported keymaps.

## CLI demo

You can play with an example of this library in the terminal with

```sh
cargo run --example cli
```

## Usage

```rust
use vim_buffer::{Buffer, Mode};
use vim_buffer::crossterm::event::{Event, KeyEvent, KeyCode};

let mut buffer = Buffer::default();
assert_eq!(buffer.as_mode(), Mode::Normal);

// Update it with crossterm events
buffer.update(&Event::Key(KeyEvent::from(KeyCode::Char('i'))));
for ch in "hello".chars() {
    buffer.update(&Event::Key(KeyEvent::from(KeyCode::Char(ch))));
}
assert_eq!(buffer.as_content(), "hello");

// Update with Vim string
buffer.update_from_string("<Esc>0sH<Esc>A, World!");
assert_eq!(buffer.as_content(), "Hello, World!");
```
