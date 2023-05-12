#[derive(Debug)]
pub struct Set {
    key: String,
    val: String,
    // todo: TTL
}

impl Set {
    pub fn from_key_val(key: impl ToString, value: impl ToString) -> Self {
        Set {
            key: key.to_string(),
            val: value.to_string(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn val(&self) -> &str {
        &self.val
    }
}
