/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

 */

use crate::{
    error::Error,
    writer::{Arguments, Output},
    reader::NodeTypesFile,
};
use std::io::Write;
use tera::Tera;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantsFile;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Output for ConstantsFile {
    const DEFAULT_FILE_NAME: &str = "nodes";
    const DEFAULT_DIRECTORY: &str = "bindings";
    type InputFile = NodeTypesFile;

    fn write<W>(&self, arguments: Arguments<Self::InputFile>, w: &mut W) -> Result<(), Error> where W: Write {
        let tera = Tera::new("templates/**/constants.*")?;

        let mut context = tera::Context::new();
        context.insert("super_node_names", &arguments.input_file.super_type_node_type_names());
        context.insert("node_names", &arguments.input_file.regular_node_type_names());
        context.insert("field_names", &arguments.input_file.field_names());
        context.insert("terminal_names", &arguments.input_file.terminal_node_type_names());

        let rendered = tera.render(&format!("constants.{}", arguments.for_language), &context).unwrap();
        w.write(rendered.as_bytes())?;

        Ok(())
    }
}
