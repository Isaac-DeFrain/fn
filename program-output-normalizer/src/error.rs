use crate::version::Version;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Error {
    version: Version,
    error: String,
}

impl Error {
    pub fn new(version: Version, error: String) -> Self {
        Self { version, error }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}
