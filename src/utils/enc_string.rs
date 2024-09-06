use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type, Default)]
#[sqlx(transparent)]
pub struct EncString(String);

impl EncString {
    pub fn plain(&self) -> Result<String, Error> {
        Ok(urlencoding::decode(self.0.as_str())?.to_string())
    }

    pub fn encoded(&self) -> &String {
        &self.0
    }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn encode(string: &str) -> Self {
        Self(urlencoding::encode(string).to_string())
    }
}

impl From<String> for EncString {
    fn from(value: String) -> Self {
        Self::encode(value.as_str())
    }
}

impl From<&str> for EncString {
    fn from(value: &str) -> Self {
        Self::encode(value)
    }
}

impl Debug for EncString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Display for EncString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.plain().unwrap().as_str())
    }
}

impl Deref for EncString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}