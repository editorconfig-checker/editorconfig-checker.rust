use std::str::FromStr;

use crate::error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_architecture() {
        assert!("HALLO".parse::<Architecture>().is_err());
        assert_eq!(
            "amd64".parse::<Architecture>().unwrap(),
            Architecture::Amd64
        );
        assert_eq!(
            "Amd64".parse::<Architecture>().unwrap(),
            Architecture::Amd64
        );
        assert_eq!("386".parse::<Architecture>().unwrap(), Architecture::I386);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Architecture {
    Amd64,
    I386,
    Unsupported,
}

impl Architecture {
    pub fn stringify(self) -> &'static str {
        use Architecture::*;
        match self {
            Amd64 => "amd64",
            I386 => "386",
            Unsupported => "Unsupported",
        }
    }
}

impl FromStr for Architecture {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            // TODO: test if this actually matches
            "amd64" => Ok(Architecture::Amd64),
            // TODO: test if this actually matches
            "386" => Ok(Architecture::I386),
            _ => Err(Error::Architecture(lower)),
        }
    }
}

// TODO: How to use cfg to pass a value into this function to be able to test it?
// TODO: Test
pub fn get_architecture() -> Architecture {
    // TODO: This is not sufficient and needs to care for more cases
    if cfg!(target_pointer_width = "64") {
        Architecture::Amd64
    } else if cfg!(target_pointer_width = "32") {
        Architecture::I386
    } else {
        Architecture::Unsupported
    }
}
