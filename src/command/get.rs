#[derive(Debug)]
pub struct Get {
    key: String,
    valid: bool,
}

impl Get {
    pub fn from_key(key: impl ToString) -> Self {
        Get {
            key: key.to_string(),
            valid: true,
        }
    }

    pub fn new_invalid() -> Self {
        Get {
            key: String::from(""),
            valid: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}
