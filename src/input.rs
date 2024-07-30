use crossterm::event::{self, Event, KeyCode, KeyEvent};

pub struct Input;

impl Input {
    pub fn receive(&self) -> Option<String> {
        loop {
            if event::poll(std::time::Duration::from_millis(500)).ok()? {
                if let Event::Key(key_event) = event::read().ok()? {
                    return key_event.to_string();
                }
            };
        }
    }
}

trait KeyEventToString {
    fn to_string(&self) -> Option<String>;
}

impl KeyEventToString for KeyEvent {
    fn to_string(&self) -> Option<String> {
        let key: String = match self.code {
            KeyCode::Char(c) => Some(c.to_string()),
            KeyCode::Enter => Some("enter".to_string()),
            KeyCode::Backspace => Some("backspace".to_string()),
            _ => None,
        }?;

        let modifiers = self
            .modifiers
            .iter_names()
            .filter_map(|(s, _)| match s {
                "SHIFT" => None,
                "CONTROL" => Some("ctrl"),
                "ALT" => Some("alt"),
                "SUPER" => Some("super"),
                "HYPER" => Some("hyper"),
                "META" => Some("meta"),
                _ => None,
            })
            .collect::<Vec<&str>>();

        if modifiers.is_empty() {
            Some(key)
        } else {
            Some(format!("{}+{}", modifiers.join("+"), key))
        }
    }
}
