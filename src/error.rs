use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),

    LimitExcedeed,

    // jpeg
    FirstTwoBytesNotSOI,

    // riff
    NoRiffHeader,

    // webp
    NoWebpCC,
    NoChunk([u8; 4]),
    InvalidFormat([u8; 4]),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),

            Error::LimitExcedeed => write!(f, "length exceeded the provided limit"),

            Error::FirstTwoBytesNotSOI => write!(f, "first two bytes is not a SOI marker"),

            Error::NoRiffHeader => write!(f, "no riff header"),
            Error::NoWebpCC => write!(f, "no webp cc"),
            Error::NoChunk(id) => write!(f, "no chunk of id: {:X?}", id),
            Error::InvalidFormat(format) => write!(f, "invalid format: {:X?}", format),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
