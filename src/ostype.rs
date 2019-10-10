use std::str::FromStr;

use crate::error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_os_type() {
        assert!("HALLO".parse::<OsType>().is_err());
        assert_eq!("Linux".parse::<OsType>().unwrap(), OsType::Linux);
        assert_eq!("Darwin".parse::<OsType>().unwrap(), OsType::Darwin);
        assert_eq!("macos".parse::<OsType>().unwrap(), OsType::Darwin);
        assert_eq!("WiNdOwS".parse::<OsType>().unwrap(), OsType::Windows);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OsType {
    Darwin,
    Dragonfly,
    FreeBSD,
    Linux,
    NetBSD,
    OpenBSD,
    // currently there is no Plan9 target for Rust
    Plan9,
    Solaris,
    Windows,
}

impl OsType {
    pub fn stringify(self) -> &'static str {
        use OsType::*;
        match self {
            Darwin => "darwin",
            Dragonfly => "dragonfly",
            FreeBSD => "freebsd",
            Linux => "linux",
            NetBSD => "netbsd",
            OpenBSD => "openbsd",
            Plan9 => "plan9",
            Solaris => "solaris",
            Windows => "windows",
        }
    }
}

impl FromStr for OsType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "linux" => Ok(OsType::Linux),
            "darwin" => Ok(OsType::Darwin),
            "macos" => Ok(OsType::Darwin),
            // TODO: test if this actually matches
            "dragonfly" => Ok(OsType::Dragonfly),
            // TODO: test if this actually matches
            "freebsd" => Ok(OsType::FreeBSD),
            // TODO: test if this actually matches
            "netbsd" => Ok(OsType::NetBSD),
            // TODO: test if this actually matches
            "openbsd" => Ok(OsType::OpenBSD),
            // TODO: test if this actually matches
            "plan9" => Ok(OsType::Plan9),
            // TODO: test if this actually matches
            "solaris" => Ok(OsType::Solaris),
            "windows" => Ok(OsType::Windows),
            _ => Err(Error::ParseOS(lower)),
        }
    }
}
