use std::io;

#[derive(Debug)]
pub enum SizeTreeError {
    PathDoesNotExist,
    UnsuportedFileType,
    PermissionDenied,
    OtherIoError,
}

pub type Result<T> = std::result::Result<T, SizeTreeError>;

impl From<io::Error> for SizeTreeError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::NotFound => Self::PathDoesNotExist,
            io::ErrorKind::PermissionDenied => Self::PermissionDenied,
            _ => Self::OtherIoError,
        }
    }
}