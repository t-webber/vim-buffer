# vim

![Clippy](https://github.com/t-webber/vim/actions/workflows/clippy.yml/badge.svg?branch=main)
![Build](https://github.com/t-webber/vim/actions/workflows/build.yml/badge.svg?branch=main)
![Tests](https://github.com/t-webber/vim/actions/workflows/tests.yml/badge.svg?branch=main)
![Rustdoc](https://github.com/t-webber/vim/actions/workflows/rustdoc.yml/badge.svg?branch=main)
![Rusfmt](https://github.com/t-webber/vim/actions/workflows/rustfmt.yml/badge.svg?branch=main)
![Coverage](https://github.com/t-webber/vim/actions/workflows/coverage.yml/badge.svg?branch=main)
![Taplo](https://github.com/t-webber/vim/actions/workflows/taplo.yml/badge.svg?branch=main)
![Spelling](https://github.com/t-webber/vim/actions/workflows/spelling.yml/badge.svg?branch=main)

[![github](https://img.shields.io/badge/GitHub-t--webber/vim--buffer-blue?logo=GitHub)](https://github.com/t-webber/vim)
[![license](https://img.shields.io/badge/Licence-MIT%20or%20Apache%202.0-darkgreen)](https://github.com/t-webber/vim?tab=MIT-2-ov-file)
[![coverage](https://img.shields.io/badge/Coverage-100%25-purple)](https://github.com/t-webber/vim/actions/workflows/nightly.yml)
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
buffer.update(Event::Key(KeyEvent::from(KeyCode::Char('i'))));
for ch in "hello".chars() {
    buffer.update(Event::Key(KeyEvent::from(KeyCode::Char(ch))));
}
assert_eq!(buffer.as_content(), "hello");

// Update with Vim string
buffer.update_from_string("<Esc>0rHA, World!");
assert_eq!(buffer.as_content(), "Hello, World!");
```

## Performance

The buffer is internally represented as a `String` and thus, editions can be costly. `String` was chosen because most of use cases are: 1) display the buffer, 2) update the buffer, 3) redisplay the buffer, and this over and over again. There are two choices: optimise the data structure for editions (that are only a subset of updates, e.g. you can move the cursor around, play with history, etc.) or optimise it for display (which is done at every update. That is why `String` was chosen.
