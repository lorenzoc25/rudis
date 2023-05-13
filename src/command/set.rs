#[derive(Debug)]
pub struct Set {
    key: String,
    val: String,
    valid: bool,
    // todo: TTL
}

impl Set {
    pub fn from_key_val(key: impl ToString, value: impl ToString) -> Self {
        Set {
            key: key.to_string(),
            val: value.to_string(),
            valid: true,
        }
    }

    pub fn new_invalid() -> Self {
        Set {
            key: String::from(""),
            val: String::from(""),
            valid: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn val(&self) -> &str {
        &self.val
    }
}
