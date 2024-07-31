use crossterm::style::Stylize;

use super::Letters;

pub struct Comparer {
    correct_letters: Vec<(char, Option<bool>)>,
}

impl Comparer {
    pub fn new() -> Self {
        Self {
            correct_letters: vec![],
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.correct_letters.clear();
        self
    }

    pub fn reset_with(&mut self, vec: &[char]) -> &mut Self {
        self.correct_letters = vec.iter().cloned().map(|c| (c, None)).collect();
        self
    }

    pub fn compare(&mut self, letters: &Letters) -> &mut Self {
        let letters = letters.to_chars();

        self.correct_letters = self
            .correct_letters
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, (actual, _correct))| {
                let user_input = letters.get(i);

                match user_input {
                    None => (actual, None),
                    Some(input) => {
                        if *input == actual {
                            (actual, Some(true))
                        } else {
                            (actual, Some(false))
                        }
                    }
                }
            })
            .collect();

        self
    }

    pub fn print_colored(&self) {
        self.correct_letters.iter().for_each(|(l, correct_state)| {
            let l = match correct_state {
                Some(true) => l.green(),
                Some(false) => l.red(),
                None => l.dim(),
            };
            print!("{l}");
        });
    }
}
