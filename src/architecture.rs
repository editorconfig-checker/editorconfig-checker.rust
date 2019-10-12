use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Architecture {
    Amd64,
    I386,
}

impl Architecture {
    pub fn stringify(self) -> &'static str {
        use Architecture::*;
        match self {
            Amd64 => "amd64",
            I386 => "386",
        }
    }
}

impl FromStr for Architecture {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "x86_64" => Ok(Architecture::Amd64),
            // TODO: test if this actually matches
            "x86" => Ok(Architecture::I386),
            _ => Err(Error::ParseArch(lower)),
        }
    }
}
