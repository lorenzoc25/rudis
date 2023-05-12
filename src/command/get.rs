#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn from_key(key: impl ToString) -> Self {
        Get {
            key: key.to_string(),
        }
    }
    pub fn key(&self) -> &str {
        &self.key
    }
}
