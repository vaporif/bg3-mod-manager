#![allow(unused)]

use std::error::Error as StdError;

use serde::Serialize;
use specta::Type;
use zip::result::ZipError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("failed to work with zip archive")]
    ZipError(#[from] ZipError),
    // FieldErrors(Vec<FieldError>),
    #[error("{0}")]
    Other(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Serialize, Debug)]
pub struct FieldError {
    field_name: String,
    error_reason_txt: String,
    inner_error: Option<String>,
}

// impl StdError for FieldError {
//     fn source(&self) -> Option<&(dyn StdError + 'static)> {
//         None
//     }
// }

// impl std::fmt::Display for FieldError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{:?}", self))
//     }
// }
