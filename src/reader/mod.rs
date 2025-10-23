/*!
One-line description.

 */

use crate::error::Error;
use std::path::{Path, PathBuf};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait InputFile {
    const DEFAULT_FILE_NAME: &str;
    const DEFAULT_DIRECTORY: &str;

    fn from_default() -> Result<Self, Error>
    where
        Self: Sized,
    {
        Self::from_file(format!(
            "{}/{}",
            Self::DEFAULT_DIRECTORY,
            Self::DEFAULT_FILE_NAME
        ))
    }

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error>
    where
        Self: Sized;

    fn file_path(override_directory: Option<&PathBuf>) -> String {
        format!(
            "{}/{}",
            if let Some(override_directory) = override_directory {
                override_directory.display().to_string()
            } else {
                Self::DEFAULT_DIRECTORY.to_string()
            },
            Self::DEFAULT_FILE_NAME
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod grammar;
pub use grammar::GrammarFile;

pub mod node_types;
pub use node_types::NodeTypesFile;
