use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};

mod buffer;
mod display;
mod input;

use buffer::Letters;
use display::Display;
use input::Input;

fn main() -> anyhow::Result<()> {
    // Enable raw mode
    enable_raw_mode()?;

    let mut letters = Letters::new();
    let mut display = Display::new();
    let input = Input;

    display.position_cursor(0, 0).clear();

    loop {
        display.position_cursor(0, 0);

        if let Some(combo) = input.receive().as_deref() {
            match combo {
                key if key.len() == 1 => {
                    letters.push(key);
                    display.move_cursor(Some(-2), None);
                }
                "ctrl+c" => {
                    display.clear().println("Goodbye");
                    disable_raw_mode()?;
                    break;
                }
                "ctrl+w" => letters.pop_word(),
                "ctrl+u" => letters.pop_line(),
                "enter" => letters.newline(),
                "backspace" => letters.pop(),
                _ => continue,
            }
        }

        display.clear();
        letters.print();

        //TODO: move cursor to end. (using line_index and somehow current line length)

        // Flush stdout to ensure the message is printed immediately
        io::stdout().flush()?;
    }

    // Disable raw mode before exiting
    disable_raw_mode()?;

    Ok(())
}
