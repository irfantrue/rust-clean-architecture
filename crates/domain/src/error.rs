#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EmailError {
    #[error("email is empty")]
    Empty,

    #[error("email format is invalid")]
    InvalidFormat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub const MAX_LEN: usize = 254;

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_trusted(s: String) -> Self {
        Self(s)
    }

    pub fn parse(raw: &str) -> Result<Self, EmailError> {
        let email = raw.trim().to_ascii_lowercase();

        if email.chars().any(char::is_whitespace) {
            return Err(EmailError::Empty);
        }

        let Some((local, domain)) = email.split_once('@') else {
            return Err(EmailError::InvalidFormat);
        };

        if local.is_empty()
            || !domain.contains('.')
            || domain.starts_with('.')
            || domain.ends_with('.')
        {
            return Err(EmailError::InvalidFormat);
        }

        Ok(Self(email))
    }
}
