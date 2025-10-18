# vim-buffer

![Clippy](https://github.com/t-webber/vim-buffer/actions/workflows/clippy.yml/badge.svg?branch=main)
![Build](https://github.com/t-webber/vim-buffer/actions/workflows/build.yml/badge.svg?branch=main)
![Tests](https://github.com/t-webber/vim-buffer/actions/workflows/tests.yml/badge.svg?branch=main)
![Docs](https://github.com/t-webber/vim-buffer/actions/workflows/docs.yml/badge.svg?branch=main)
![Fmt](https://github.com/t-webber/vim-buffer/actions/workflows/fmt.yml/badge.svg?branch=main)
![Coverage](https://github.com/t-webber/vim-buffer/actions/workflows/coverage.yml/badge.svg?branch=main)

[![github](https://img.shields.io/badge/GitHub-t--webber/vim--buffer-blue?logo=GitHub)](https://github.com/t-webber/vim-buffer)
[![license](https://img.shields.io/badge/Licence-MIT%20or%20Apache%202.0-darkgreen)](https://github.com/t-webber/vim-buffer?tab=MIT-2-ov-file)
[![coverage](https://img.shields.io/badge/Coverage-100%25-purple)](https://github.com/t-webber/vim-buffer/actions/workflows/nightly.yml)
[![rust-edition](https://img.shields.io/badge/Rust--edition-2024-darkred?logo=Rust)](https://doc.rust-lang.org/stable/edition-guide/rust-2024/)

A buffer that listens for vim keymaps.

## Development status

The buffer only supports a single line for now.

Supported keymaps:

- Insert mode
  - Characters, `Escape`, `Backspace`, `LeftArrow`, `RightArrow`
- Normal mode
  - `i`, `a`, `h`, `l`, `I`, `A`, `LeftArrow`, `RightArrow`

## CLI demo

You can play with an example of this library in the terminal with

```sh
cargo run --example cli
```
