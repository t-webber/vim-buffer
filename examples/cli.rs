use core::iter::repeat_n;
use std::io::{Write, stdout};

use color_eyre as colour_eyre; // ignore-spell
use crossterm::event::{self, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use vim_buffer::crossterm::terminal::enable_raw_mode;
use vim_buffer::{Buffer, Mode};

const RESET: &str = "\x1b[0m";
const NORMAL: ModeDisplay =
    ModeDisplay { colour: "\x1b[32m", prompt: "normal >>> " };
const INSERT: ModeDisplay =
    ModeDisplay { colour: "\x1b[36m", prompt: "insert >>> " };
const _: () = assert!(NORMAL.prompt.len() == INSERT.prompt.len());
const DISPLAY_MODE_LEN: usize = NORMAL.prompt.len();

struct ModeDisplay {
    colour: &'static str,
    prompt: &'static str,
}

impl ModeDisplay {
    fn display(&self) -> String {
        let Self { colour, prompt } = self;
        format!("{colour}{prompt}{RESET}")
    }
}

impl From<Mode> for ModeDisplay {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Insert => INSERT,
            Mode::Normal => NORMAL,
            _ => panic!("unsupported"),
        }
    }
}

fn print_current(
    buffer: &Buffer,
    previous_len: usize,
) -> colour_eyre::Result<()> {
    let spaces =
        repeat_n(' ', previous_len + DISPLAY_MODE_LEN).collect::<Box<str>>();
    let cursor = buffer.as_cursor() + DISPLAY_MODE_LEN;
    print!(
        "\r{spaces}\r{}{}\r\x1b[{cursor}C",
        ModeDisplay::from(buffer.as_mode()).display(),
        buffer.as_content()
    );
    stdout().flush()?;
    Ok(())
}

fn main() -> colour_eyre::Result<()> {
    colour_eyre::install()?;
    println!("Press <C-c> to exit");
    enable_raw_mode()?;

    let mut buffer = Buffer::default();
    let mut previous_len = 0;

    loop {
        print_current(&buffer, previous_len)?;
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
