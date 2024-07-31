use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};

mod buffer;
mod display;
mod input;

use buffer::{Comparer, Letters, Target};
use display::Display;
use input::Input;

fn main() -> anyhow::Result<()> {
    // Enable raw mode
    enable_raw_mode()?;

    let mut display = Display::new();
    let mut letters = Letters::new();
    let target = Target::new();
    let mut comparer = Comparer::new();
    let input = Input;

    display.hide_cursor().position_cursor(0, 0).clear();

    comparer
        .reset_with(&target.current_as_chars())
        .compare(&letters)
        .print_colored();

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
                    exit();
                    break;
                }
                "ctrl+w" => letters.pop_word(),
                "ctrl+u" => letters.pop_line(),
                "enter" => letters.newline(),
                "backspace" => letters.pop(),
                _ => continue,
            }
        }

        comparer.compare(&letters);

        display.clear();
        comparer.print_colored();

        //TODO: move cursor to end. (using line_index and somehow current line length)

        // Flush stdout to ensure the message is printed immediately
        io::stdout().flush()?;
    }

    // Disable raw mode before exiting
    exit();

    Ok(())
}

fn exit() {
    use crossterm::{cursor, execute};
    use std::io::stdout;

    let _ = execute!(stdout(), cursor::Show);
    let _ = disable_raw_mode();
}
