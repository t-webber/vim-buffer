use std::io::{Write, stdout};

use crossterm::event::{self, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use vim_buffer::Buffer;
use vim_buffer::crossterm::terminal::enable_raw_mode;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    println!("Press <C-c> to exit");
    enable_raw_mode()?;

    let mut buffer = Buffer::default();

    loop {
        print!("\r>>> {}", buffer.as_content());
        stdout().flush()?;

        let event = event::read()?;
        if let Some(key_event) = event.as_key_press_event()
            && key_event.modifiers & KeyModifiers::CONTROL
                == KeyModifiers::CONTROL
            && let Some(ch) = key_event.code.as_char()
            && ch == 'c'
        {
            disable_raw_mode()?;
            break Ok(());
        }

        buffer.update(&event);
    }
}
