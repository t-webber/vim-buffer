use core::fmt;
use core::iter::repeat_n;
use std::io::{Write, stdout};

use color_eyre as colour_eyre; // ignore-spell
use crossterm::event::{self, Event, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use vim_buffer::crossterm::terminal::enable_raw_mode;
use vim_buffer::{Buffer, Mode};

pub const RESET_COLOUR: &str = "\x1b[0m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const CYAN: &str = "\x1b[36m";

const NORMAL: ModePrompt = ModePrompt { colour: CYAN, prompt: "normal >>> " };
const INSERT: ModePrompt = ModePrompt { colour: GREEN, prompt: "insert >>> " };
const UNKNOWN: ModePrompt = ModePrompt { colour: RED, prompt: "unkown >>> " };

const _: () = assert!(NORMAL.prompt.len() == INSERT.prompt.len());
const _: () = assert!(UNKNOWN.prompt.len() == INSERT.prompt.len());
const MODE_PROMPT_LEN: usize = NORMAL.prompt.len();

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

/// Prints some spaces to clear the previous buffer, whose content was of length
/// `previous_len`.
fn clear_line(previous_len: usize) {
    let nb_spaces = previous_len + MODE_PROMPT_LEN;
    let spaces = repeat_n(' ', nb_spaces).collect::<String>();
    print!("\r{spaces}\r")
}

/// Prints the [`Buffer`] with the mode it is in, and it's contents.
fn print_buffer(buffer: &Buffer) {
    let cursor = buffer.as_cursor() + MODE_PROMPT_LEN;
    let mode = match buffer.as_mode() {
        Mode::Insert => INSERT,
        Mode::Normal => NORMAL,
        _ => panic!("unsupported"),
    };
    let content = buffer.as_content();
    print!("{mode}{content}\r\x1b[{cursor}C",);
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

fn main() -> colour_eyre::Result<()> {
    colour_eyre::install()?;
    println!("Press <C-c> to exit");
    enable_raw_mode()?;

    let mut previous_len = 0;
    let mut buffer = Buffer::default();

    loop {
        clear_line(previous_len);
        print_buffer(&buffer);
        stdout().flush()?;

        let event = event::read()?;
        if is_ctrl_c(&event) {
            break;
        }

        previous_len = buffer.as_content().len();
        buffer.update(&event);
    }

    disable_raw_mode()?;
    println!();
    Ok(())
}
