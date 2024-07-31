const QUOTES: [&str; 1] = ["The quick brown fox jumps over a lazy dog."];

pub struct Target {
    quote: &'static str,
}

impl Target {
    pub fn new() -> Self {
        Self { quote: QUOTES[0] }
    }

    pub fn current_as_chars(&self) -> Vec<char> {
        self.quote.chars().collect()
    }
}
