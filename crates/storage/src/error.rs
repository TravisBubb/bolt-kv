use std::{fmt, io, sync::PoisonError};

#[derive(Debug)]
pub enum StorageError {
    Io(io::Error),
    Poisoned,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::Io(err) => write!(f, "I/O error: {}", err),
            StorageError::Poisoned => write!(f, "Lock poisoned"),
        }
    }
}

impl From<io::Error> for StorageError {
    fn from(err: io::Error) -> Self {
        StorageError::Io(err)
    }
}

impl<T> From<PoisonError<T>> for StorageError {
    fn from(_err: PoisonError<T>) -> Self {
        StorageError::Poisoned
    }
}
