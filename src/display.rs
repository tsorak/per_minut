use core::fmt;
use std::io::{stdout, Stdout};

use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp},
    execute,
    terminal::{Clear, ClearType},
};

pub struct Display(pub Stdout);

impl Display {
    pub fn new() -> Self {
        Self(stdout())
    }

    pub fn println(&mut self, s: impl Into<String> + fmt::Display) -> &mut Self {
        self.move_cursor(None, Some(1));
        print!("{s}");
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        execute!(self.0, Clear(ClearType::All)).unwrap();
        self
    }

    pub fn position_cursor(&mut self, x: u16, y: u16) -> &mut Self {
        execute!(self.0, MoveTo(x, y)).unwrap();
        self
    }

    pub fn move_cursor(&mut self, x: Option<i32>, y: Option<i32>) -> &mut Self {
        match x {
            Some(x) if x.gt(&0) => execute!(self.0, MoveRight(x as u16)).unwrap(),
            Some(x) if x.lt(&0) => execute!(self.0, MoveLeft(-x as u16)).unwrap(),
            _ => (),
        }

        match y {
            Some(y) if y.gt(&0) => execute!(self.0, MoveUp(y as u16)).unwrap(),
            Some(y) if y.lt(&0) => execute!(self.0, MoveDown(-y as u16)).unwrap(),
            _ => (),
        }
        self
    }
}
