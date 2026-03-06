use core::fmt;
use core::iter::repeat_n;
use std::io::{Write, stdout};

use color_eyre as colour_eyre; // ignore-spell
use crossterm::event::{self, Event, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use vim_buffer::crossterm::terminal::enable_raw_mode;
use vim_buffer::{Buffer, Mode};

const RESET_COLOUR: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";

const NORMAL: ModePrompt = ModePrompt { colour: CYAN, prompt: "normal >>> " };
const INSERT: ModePrompt = ModePrompt { colour: GREEN, prompt: "insert >>> " };

/// Prompt displayed for a given vim mode.
struct ModePrompt {
    colour: &'static str,
    prompt: &'static str,
}

impl fmt::Display for ModePrompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { colour, prompt } = self;
        write!(f, "{colour}{prompt}{}", RESET_COLOUR)
    }
}

impl From<Mode> for ModePrompt {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Insert => INSERT,
            Mode::Normal => NORMAL,
            _ => unreachable!(),
        }
    }
}

/// Prints some spaces to clear the previous buffer, whose content was of length
/// `previous_len`.
fn clear_line(previous_len: usize) {
    let spaces = repeat_n(' ', previous_len).collect::<String>();
    print!("\r{spaces}\r")
}

/// Prints the [`Buffer`] with the mode it is in, and it's contents.
fn print_buffer(buffer: &Buffer) -> usize {
    let mode = ModePrompt::from(buffer.as_mode());
    let prompt_len = mode.prompt.len();
    let cursor = buffer.as_cursor() + prompt_len;
    let content = buffer.as_content();
    print!("{mode}{content}\r\x1b[{cursor}C");
    prompt_len + buffer.len()
}

/// Checks whether the given event is `<C-c>` or not.
fn is_ctrl_c(event: &Event) -> bool {
    if let Some(key_event) = event.as_key_press_event()
        && key_event.modifiers == KeyModifiers::CONTROL
        && let Some(ch) = key_event.code.as_char()
        && ch == 'c'
    {
        true
    } else {
        false
    }
}

fn raw_main() -> colour_eyre::Result<()> {
    let mut previous_len = 0;
    let mut buffer = Buffer::default();

    loop {
        clear_line(previous_len);
        previous_len = print_buffer(&buffer);
        stdout().flush()?;

        let event = event::read()?;
        if is_ctrl_c(&event) {
            break Ok(());
        }

        buffer.update(&event);
    }
}

fn main() -> colour_eyre::Result<()> {
    colour_eyre::install()?;
    println!("Press <C-c> to exit");

    enable_raw_mode()?;

    let res = raw_main();

    disable_raw_mode()?;
    println!();

    res
}
