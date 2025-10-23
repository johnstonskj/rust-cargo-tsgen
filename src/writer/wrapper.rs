/*!
One-line description.

 */

use crate::{
    error::Error,
    reader::GrammarFile,
    writer::{Arguments, Output},
};
use std::io::Write;
use tera::Tera;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct WrapperFile;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Output for WrapperFile {
    const DEFAULT_FILE_NAME: &str = "wrapper";
    const DEFAULT_DIRECTORY: &str = "bindings";
    type InputFile = GrammarFile;

    fn write<W>(&self, arguments: Arguments<Self::InputFile>, w: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        let tera = Tera::new("templates/**/wrapper.*")?;

        let mut context = tera::Context::new();
        context.insert("name", arguments.input_file.name());
        context.insert("root_node", &String::new());
        context.insert("compound_nodes", &Vec::<String>::default());
        context.insert("value_nodes", &vec!["IdentifierValue", "TokenValue"]);

        let rendered = tera
            .render(&format!("wrapper.{}", arguments.for_language), &context)
            .unwrap();
        w.write_all(rendered.as_bytes())?;

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
