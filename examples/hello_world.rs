use std::io::{Write, stdout};

use crossterm::event::{self, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use vim_buffer::crossterm::terminal::enable_raw_mode;
use vim_buffer::{Buffer, Mode};

const fn display_mode(mode: Mode) -> &'static str {
    match mode {
        Mode::Normal => "\x1b[32mnormal >>> \x1b[0m",
        Mode::Insert => "\x1b[36minsert >>> \x1b[0m",
        _ => panic!("unsupported"),
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    println!("Press <C-c> to exit");
    enable_raw_mode()?;

    let mut buffer = Buffer::default();

    loop {
        print!("\r{}{}", display_mode(buffer.as_mode()), buffer.as_content());
        stdout().flush()?;

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
