#[derive(Debug)]
pub enum ErrorKind {
    OutOfRange, OutOfMemory, BrokenPacket, EOF, Deprecated, NotImplement, Other
}
 
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Error {
        Error { kind, message: String::from(msg) }
    }
}

pub type Result<T> = core::result::Result<T, Error>;