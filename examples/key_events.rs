use color_eyre as colour_eyre; // ignore-spell
use crossterm::event::{self, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};


fn main() -> colour_eyre::Result<()> {
    colour_eyre::install()?;
    enable_raw_mode()?;

    loop {
        let event = event::read()?;

        if let Some(key_event) = event.as_key_press_event()
            && key_event.modifiers == KeyModifiers::CONTROL
            && let Some(ch) = key_event.code.as_char()
            && ch == 'c'
        {
            disable_raw_mode()?;
            break Ok(());
        }

        println!("{event:?}\r");
    }
}
