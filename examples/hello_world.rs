use core::iter::repeat_n;
use std::io::{Write, stdout};

use crossterm::event::{self, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use vim_buffer::crossterm::terminal::enable_raw_mode;
use vim_buffer::{Buffer, Mode};

const NORMAL: &str = "\x1b[32mnormal >>> \x1b[0m";
const INSERT: &str = "\x1b[36minsert >>> \x1b[0m";

const _: () = assert!(NORMAL.len() == INSERT.len());
const DISPLAY_MODE_LEN: usize = NORMAL.len();

const fn display_mode(mode: Mode) -> &'static str {
    match mode {
        Mode::Normal => NORMAL,
        Mode::Insert => INSERT,
        _ => panic!("unsupported"),
    }
}


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    println!("Press <C-c> to exit");
    enable_raw_mode()?;

    let mut buffer = Buffer::default();
    let mut previous_len = 0;

    loop {
        let spaces = repeat_n(' ', previous_len + DISPLAY_MODE_LEN)
            .collect::<Box<str>>();
        print!(
            "\r{spaces}\r{}{}",
            display_mode(buffer.as_mode()),
            buffer.as_content()
        );
        stdout().flush()?;

        previous_len = buffer.as_content().len();

        let event = event::read()?;
        if let Some(key_event) = event.as_key_press_event()
            && key_event.modifiers & KeyModifiers::CONTROL
                == KeyModifiers::CONTROL
            && let Some(ch) = key_event.code.as_char()
            && ch == 'c'
        {
            disable_raw_mode()?;
            println!();
            break Ok(());
        }

        buffer.update(&event);
    }
}
