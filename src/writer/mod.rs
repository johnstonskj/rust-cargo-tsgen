/*!
One-line description.

 */

use crate::{error::Error, reader::InputFile};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::{BufWriter, Write, stdout},
    path::{Path, PathBuf},
    str::FromStr,
};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Output {
    const DEFAULT_FILE_NAME: &str;
    const DEFAULT_DIRECTORY: &str;
    type InputFile: InputFile;

    fn write<W>(&self, arguments: Arguments<Self::InputFile>, w: &mut W) -> Result<(), Error>
    where
        W: Write;

    fn print(&self, arguments: Arguments<Self::InputFile>) -> Result<(), Error> {
        self.write(arguments, &mut stdout())
    }

    fn write_to_file<P>(&self, arguments: Arguments<Self::InputFile>, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let file_path = path.as_ref().display().to_string();
        let file = match File::create(path) {
            Ok(file) => file,
            Err(e) => {
                error!("Could not create output file, check directory exists in {file_path}");
                return Err(e.into());
            }
        };
        let mut writer = BufWriter::new(file);

        self.write(arguments, &mut writer)
    }

    fn output_file(&self, for_language: ForLanguage) -> String {
        format!(
            "{}.{}",
            Self::DEFAULT_FILE_NAME,
            for_language.file_extension(),
        )
    }

    fn output_directory(
        &self,
        for_language: ForLanguage,
        override_directory: Option<&PathBuf>,
    ) -> String {
        if let Some(override_directory) = override_directory {
            override_directory.display().to_string()
        } else {
            format!("{}/{}", Self::DEFAULT_DIRECTORY, for_language.output_dir(),)
        }
    }

    fn file_path(&self, for_language: ForLanguage, override_directory: Option<&PathBuf>) -> String {
        format!(
            "{}/{}",
            self.output_directory(for_language, override_directory),
            self.output_file(for_language),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Arguments<F: InputFile> {
    input_file: F,
    for_language: ForLanguage,
    output_directory: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ForLanguage {
    #[default]
    Rust,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Arguments
// ------------------------------------------------------------------------------------------------

impl<F: InputFile> Arguments<F> {
    pub fn new(
        input_file: F,
        for_language: ForLanguage,
        output_directory: Option<PathBuf>,
    ) -> Self {
        Self {
            input_file,
            for_language,
            output_directory,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ ForLanguage
// ------------------------------------------------------------------------------------------------

impl Display for ForLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Rust => "rust",
            }
        )
    }
}

impl FromStr for ForLanguage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" => Ok(Self::Rust),
            _ => Err(Error::Unknown {
                message: format!("could not parse `{s}` into `ForLanguage`"),
            }),
        }
    }
}

impl ForLanguage {
    pub const fn output_dir(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
        }
    }

    pub const fn file_extension(&self) -> &'static str {
        match self {
            Self::Rust => "rs",
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod constants;
pub use constants::ConstantsFile;

pub mod wrapper;
pub use wrapper::WrapperFile;
