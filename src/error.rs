/*!
Provides this crate's [`Error`] and [`Result`] types as well as helper functions.

 */

use flat_error::FlatError;
use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

use tera::Error as TeraError;

#[cfg(feature = "cli")]
use crate::cli::TracingError;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// I/O error from the `std::io` library.
    Io { source: FlatError },
    /// An error occurred parsing JSON.
    Json { source: FlatError },
    /// An error occurred rendering a Tera template.
    Render { source: FlatError },
    /// An error occurred initializing tracing.
    #[cfg(feature = "cli")]
    TracingInitError { source: TracingError },
    /// Multiple errors were aggregated from some function below.
    MultipleErrors { sources: Vec<Error> },
    /// An unknown error occurred.
    Unknown { message: String },
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type Result<T> = StdResult<Error, T>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Construct an `Error` from the provided message.
#[inline]
pub fn unknown_error<S: Into<String>>(message: S) -> Error {
    Error::Unknown {
        message: message.into(),
    }
}

/// Construct an `Error` from the standard library error.
#[inline]
pub fn io_error(error: ::std::io::Error) -> Error {
    Error::Io {
        source: FlatError::from_any(&error),
    }
}

/// Construct an `Error` from a `serde_json` error.
#[inline]
pub fn json_error(error: ::serde_json::Error) -> Error {
    Error::Io {
        source: FlatError::from_any(&error),
    }
}

/// Construct an `Error` from a `serde_json` error.
#[inline]
pub fn tera_error(error: TeraError) -> Error {
    Error::Io {
        source: FlatError::from_any(&error),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Io { source: _ } => format!("An I/O error occurred in the standard library."),
                Self::Json { source: _ } =>
                    format!("An error occurred parsing an input file as JSON."),
                Self::Render { source: _ } =>
                    format!("An error occurred rendering an output template."),
                #[cfg(feature = "cli")]
                Self::TracingInitError { source: _ } =>
                    format!("Error occurred initializing a tracing subscriber."),
                Self::MultipleErrors { sources } => {
                    format!(
                        "Multiple errors occurred:\n{}",
                        sources
                            .iter()
                            .enumerate()
                            .map(|(i, e)| format!("{i:<3}. {e}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                }
                Self::Unknown { message } =>
                    format!("An unknown error occurred; message: {message}"),
            }
        )
    }
}

impl StdError for Error {
    #[cfg(feature = "cli")]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::TracingInitError { source } => Some(source),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ From ❱ Libraries
// ------------------------------------------------------------------------------------------------

impl From<::std::io::Error> for Error {
    fn from(source: ::std::io::Error) -> Self {
        io_error(source)
    }
}

impl From<::serde_json::Error> for Error {
    fn from(source: ::serde_json::Error) -> Self {
        json_error(source)
    }
}

#[cfg(feature = "cli")]
impl From<TracingError> for Error {
    fn from(e: TracingError) -> Self {
        Self::TracingInitError { source: e }
    }
}

impl From<TeraError> for Error {
    fn from(e: TeraError) -> Self {
        tera_error(e)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ From ❱ Multiples
// ------------------------------------------------------------------------------------------------

impl From<Vec<Error>> for Error {
    fn from(sources: Vec<Error>) -> Self {
        Self::MultipleErrors { sources }
    }
}

impl From<&[Error]> for Error {
    fn from(sources: &[Error]) -> Self {
        Self::MultipleErrors {
            sources: sources.to_vec(),
        }
    }
}

impl FromIterator<Error> for Error {
    fn from_iter<I: IntoIterator<Item = Error>>(iter: I) -> Self {
        Self::MultipleErrors {
            sources: iter.into_iter().collect(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ From ❱ Unknown
// ------------------------------------------------------------------------------------------------

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        unknown_error(message)
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        unknown_error(message)
    }
}
