/*!
Creates command-line configuration and abstracts the execution of commands using the [`Command`] trait.
 */

use crate::{
    error::Error,
    reader::{GrammarFile, InputFile, NodeTypesFile},
    writer::{Arguments, ConstantsFile, ForLanguage, Output, WrapperFile},
};
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_mangen::Man;
use clap_markdown::{MarkdownOptions, help_markdown_custom};
use clap_verbosity_flag::Verbosity;
use human_panic::setup_panic;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    path::PathBuf,
    process::ExitCode,
};
use tracing::{info, subscriber::SetGlobalDefaultError};
use tracing_subscriber::filter::{EnvFilter, LevelFilter, ParseError};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Abstraction for `clap::Parser` and `clap::Subcommand` implementations to abstract execution
/// of the functionality provided. It is expected that a parser, if it has sub-commands will
/// then defer to the sub-command implementation of `Command`.
///
pub trait Command {
    fn execute(&self) -> Result<ExitCode, Error>;
}

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = r#"This command can generate additional, type-safe, features for tree-sitter bindings."#
)]
pub struct Cli {
    #[command(flatten)]
    verbosity: Verbosity,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(hide = true)]
    MarkdownHelp,
    #[command(hide = true)]
    ManPage,
    /// Create a constants file from node-types.json
    Constants(GenerateArgs),
    /// Create a type-safe wrapper around the tree-sitter CST using grammar.json
    Wrapper(GenerateArgs),
    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

#[derive(Debug, Args)]
struct GenerateArgs {
    /// Generate output for the specified language binding. Default: "rust".
    #[arg(short = 'l', long)]
    for_language: Option<ForLanguage>,

    /// Override the default source directory. Default: "src"
    #[arg(short = 'i', long)]
    input_directory: Option<PathBuf>,

    /// Override the default binding directory. Default: "bindings/<language>/..."
    #[arg(short = 'o', long)]
    output_directory: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TracingError {
    EnvFilterParse(String),
    SetGlobalDefault(String),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Command for Cli {
    fn execute(&self) -> Result<ExitCode, Error> {
        setup_panic!();

        initialize_tracing(self.verbosity.tracing_level_filter(), Some(module_path!()))?;

        self.cmd.execute()
    }
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

// ------------------------------------------------------------------------------------------------

impl Command for Commands {
    fn execute(&self) -> Result<ExitCode, Error> {
        match self {
            Self::MarkdownHelp => println!(
                "{}",
                help_markdown_custom::<Cli>(&MarkdownOptions::default().show_footer(false))
            ),
            Self::ManPage => {
                let man = Man::new(Cli::command()).section("1");
                man.render(&mut std::io::stdout())
                    .map_err(|e| Error::Unknown {
                        message: format!("Failed to render man page; source: {e}"),
                    })?
            }
            Self::Completions { shell } => {
                shell.generate(&mut Cli::command(), &mut std::io::stdout())
            }
            Self::Constants(args) => {
                let input_file_name = NodeTypesFile::file_path(args.input_directory.as_ref());
                info!("Read source from {input_file_name:?}");
                let input = NodeTypesFile::from_file(input_file_name)?;

                let for_language = args.for_language.unwrap_or_default();
                let arguments = Arguments::new(input, for_language, args.output_directory.clone());
                info!("Created arguments {arguments:#?}");

                let output = ConstantsFile;
                let file_name = output.file_path(for_language, args.output_directory.as_ref());
                info!("Will write to file {file_name:?}");

                output.write_to_file(arguments, file_name.clone())?;
                println!("Node constants file written to {file_name:?}");
            }
            Self::Wrapper(args) => {
                let input_file_name = GrammarFile::file_path(args.input_directory.as_ref());
                info!("Read source from {input_file_name:?}");
                let input = GrammarFile::from_file(input_file_name)?;

                let for_language = args.for_language.unwrap_or_default();
                let arguments = Arguments::new(input, for_language, args.output_directory.clone());
                info!("Created arguments {arguments:#?}");

                let output = WrapperFile;
                let file_name = output.file_path(for_language, args.output_directory.as_ref());
                info!("Will write to file {file_name:?}");

                output.write_to_file(arguments, file_name.clone())?;
                println!("Node wrapper file written to {file_name:?}");
            }
        }
        Ok(ExitCode::SUCCESS)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for TracingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::EnvFilterParse(e) => e,
                Self::SetGlobalDefault(e) => e,
            }
        )
    }
}

impl StdError for TracingError {}

impl From<ParseError> for TracingError {
    fn from(e: ParseError) -> Self {
        Self::EnvFilterParse(e.to_string())
    }
}

impl From<SetGlobalDefaultError> for TracingError {
    fn from(e: SetGlobalDefaultError) -> Self {
        Self::SetGlobalDefault(e.to_string())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn initialize_tracing(level: LevelFilter, this_name: Option<&str>) -> Result<(), TracingError> {
    let mut filter = EnvFilter::from_default_env();

    if let Some(name) = this_name {
        filter = filter.add_directive(format!("{name}={level}").parse()?);
    }

    let subscriber = tracing_subscriber::fmt().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)?;
    info!("Log level set to `LevelFilter::{level:?}`");

    Ok(())
}
