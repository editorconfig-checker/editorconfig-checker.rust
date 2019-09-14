use std::fmt;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    UnknownArch,
    UnknownOS(sys_info::Error),
    InvalidPathName(std::ffi::OsString),
    Network(reqwest::Error),
    Encoding(std::str::Utf8Error),
    Output(fmt::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            IO(err) => write!(fmt, "IO({})", err),
            UnknownArch => write!(fmt, "Unknown Architecture"),
            UnknownOS(err) => write!(fmt, "Unknown Operating System ({})", err),
            InvalidPathName(err) => write!(fmt, "Invalid Path Name ({:?})", err),
            Network(err) => write!(fmt, "Error downloading the file ({})", err),
            Encoding(err) => write!(fmt, "Encoding error ({})", err),
            Output(err) => write!(fmt, "Output error ({})", err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<sys_info::Error> for Error {
    fn from(err: sys_info::Error) -> Self {
        Error::UnknownOS(err)
    }
}

impl From<std::ffi::OsString> for Error {
    fn from(err: std::ffi::OsString) -> Self {
        Error::InvalidPathName(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Network(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::Encoding(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::Output(err)
    }
}
