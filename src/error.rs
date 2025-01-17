use archive;
use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, ArchiveError>;

#[derive(Debug)]
pub struct ErrCode(pub i32);

impl fmt::Display for ErrCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum ArchiveError {
    Consumed,
    HeaderPosition,
    Sys(ErrCode, String),
}

impl error::Error for ArchiveError {}

impl fmt::Display for ArchiveError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArchiveError::Consumed => write!(fmt, "Builder already consumed"),
            ArchiveError::HeaderPosition => write!(fmt, "Header position expected to be 0"),
            ArchiveError::Sys(ref code, ref msg) => {
                write!(fmt, "{} (libarchive err_code={})", msg, code)
            }
        }
    }
}

impl<'a> From<&'a dyn archive::Handle> for ArchiveError {
    fn from(handle: &'a dyn archive::Handle) -> ArchiveError {
        ArchiveError::Sys(handle.err_code(), handle.err_msg().to_owned())
    }
}

impl<'a> From<&'a dyn archive::Handle> for Result<()> {
    fn from(handle: &'a dyn archive::Handle) -> Result<()> {
        match handle.err_code() {
            ErrCode(0) => Ok(()),
            _ => Err(ArchiveError::from(handle)),
        }
    }
}
