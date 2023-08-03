use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Version(u32);

impl Version {
    pub fn new(version: u32) -> Self {
        Self(version)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl std::str::FromStr for Version {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u32>().map(Self)
    }
}
