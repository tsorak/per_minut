pub struct Letters {
    inner: Vec<String>,
    line_index: usize,
}

impl Letters {
    pub fn new() -> Self {
        Self {
            inner: vec![],
            line_index: 0,
        }
    }

    pub fn push(&mut self, s: impl Into<String>) {
        self.inner.push(s.into());
    }

    pub fn newline(&mut self) {
        self.push("\n\r");
        self.line_index += 1;
    }

    pub fn pop(&mut self) {
        match self.inner.pop().as_deref() {
            Some("\n\r") => {
                self.line_index -= 1;
            }
            None | Some(_) => (),
        };
    }

    pub fn pop_word(&mut self) {
        let space = {
            let mut parts = self.inner.iter();

            // if the last character is a space, select the one before it.
            if parts.next_back() == Some(&" ".to_string()) {
                parts.rposition(|s| s == " ")
            } else {
                self.inner.iter().rposition(|s| s == " ")
            }
        };
        let newline = self.inner.iter().rposition(|s| s == "\n\r");

        match (space, newline) {
            (Some(space), Some(newline)) => {
                // if the space is further away than the newline, pop the line instead.
                if newline > space {
                    self.inner = self.inner.clone().into_iter().take(newline).collect();
                } else {
                    self.inner = self.inner.clone().into_iter().take(space + 1).collect();
                }
            }
            (Some(space), None) => {
                self.inner = self.inner.clone().into_iter().take(space + 1).collect();
            }
            (None, Some(newline)) => {
                self.inner = self.inner.clone().into_iter().take(newline).collect();
                self.line_index -= 1;
            }
            (None, None) if self.inner.is_empty() => (),
            (None, None) => self.inner.clear(),
        };
    }

    pub fn pop_line(&mut self) {
        let newline = self.inner.iter().rposition(|s| s == "\n\r");

        match newline {
            Some(i) => {
                self.inner = self.inner.clone().into_iter().take(i).collect();
                self.line_index -= 1;
            }
            None if self.inner.is_empty() => (),
            None => self.inner.clear(),
        }
    }

    pub fn print(&self) {
        for l in &self.inner {
            print!("{l}");
        }
    }

    pub fn to_chars(&self) -> Vec<char> {
        self.inner
            .iter()
            .cloned()
            .filter_map(|s| s.chars().next())
            .collect()
    }
}
