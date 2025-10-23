/*!
One-line description.

 */

use crate::{error::Error, reader::InputFile};
use newstr::{is_valid_newstring, regex_is_valid};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::File, io::BufReader, path::Path};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const SCHEMA_URI: &str =
    "https://tree-sitter.github.io/tree-sitter/assets/schemas/grammar.schema.json";

is_valid_newstring!(pub Identifier, is_valid_identifier; Deserialize, Serialize);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrammarFile {
    /*** Required ***/
    #[serde(rename = "$schema")]
    schema: String,
    name: Identifier,
    rules: BTreeMap<Identifier, GrammarRule>,
    /*** Optional ***/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    inherits: Option<Identifier>,
    #[serde(default)]
    conflicts: Vec<Vec<Identifier>>,
    #[serde(default)]
    externals: Vec<GrammarRule>,
    #[serde(default)]
    extras: Vec<GrammarRule>,
    #[serde(default)]
    inline: Vec<Identifier>,
    // precedences [ string | symbol-rule ]
    #[serde(default)]
    reserved: BTreeMap<Identifier, Vec<GrammarRule>>,
    #[serde(default)]
    supertypes: Vec<Identifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    word: Option<Identifier>,
}

// `optional(opt_rule)` appears as `choice(opt_rule, blank())`

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GrammarRule {
    #[serde(rename = "SEQ")]
    Sequence {
        members: Vec<GrammarRule>,
    },
    Choice {
        members: Vec<GrammarRule>,
    },
    Field {
        name: Identifier,
        content: Box<GrammarRule>,
    },
    Token {
        content: Box<GrammarRule>,
    },
    ImmediateToken {
        content: Box<GrammarRule>,
    },
    Repeat {
        content: Box<GrammarRule>,
    },
    Repeat1 {
        content: Box<GrammarRule>,
    },
    Reserved {
        content: Box<GrammarRule>,
        context_name: Identifier,
    },
    #[serde(rename = "PREC")]
    Precedence {
        value: u32,
        content: Box<GrammarRule>,
    },
    #[serde(rename = "PREC_LEFT")]
    PrecedenceLeftAssoc {
        value: u32,
        content: Box<GrammarRule>,
    },
    #[serde(rename = "PREC_RIGHT")]
    PrecedenceRightAssoc {
        value: u32,
        content: Box<GrammarRule>,
    },
    #[serde(rename = "PREC_DYNAMIC")]
    PrecedenceDynamic {
        value: u32,
        content: Box<GrammarRule>,
    },
    String {
        value: String,
    },
    Pattern {
        value: String, /* regex */
        #[serde(default, skip_serializing_if = "Option::is_none")]
        flags: Option<String>,
    },
    Symbol {
        name: Identifier,
    },
    Alias {
        value: Identifier,
        named: bool,
        content: Box<GrammarRule>,
    },
    Blank,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ GrammarFile
// ------------------------------------------------------------------------------------------------

impl InputFile for GrammarFile {
    const DEFAULT_DIRECTORY: &str = "src";
    const DEFAULT_FILE_NAME: &str = "grammar.json";

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file_path = path.as_ref().display().to_string();
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                error!("Could not open input file, check file and directory exists in {file_path}");
                return Err(e.into());
            }
        };
        let reader = BufReader::new(file);

        let list = serde_json::from_reader(reader)?;

        Ok(list)
    }
}

impl GrammarFile {
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn inherits(&self) -> Option<&Identifier> {
        self.inherits.as_ref()
    }

    pub fn rules(&self) -> impl Iterator<Item = (&Identifier, &GrammarRule)> {
        self.rules.iter()
    }

    pub fn rule_names(&self) -> impl Iterator<Item = &Identifier> {
        self.rules.keys()
    }

    pub fn conflicts(&self) -> impl Iterator<Item = &[Identifier]> {
        self.conflicts.iter().map(|v| v.as_slice())
    }

    pub fn externals(&self) -> impl Iterator<Item = &GrammarRule> {
        self.externals.iter()
    }

    pub fn extras(&self) -> impl Iterator<Item = &GrammarRule> {
        self.extras.iter()
    }

    pub fn inline(&self) -> impl Iterator<Item = &Identifier> {
        self.inline.iter()
    }

    pub fn reserved(&self) -> impl Iterator<Item = (&Identifier, &[GrammarRule])> {
        self.reserved.iter().map(|(k, v)| (k, v.as_slice()))
    }

    pub fn reserved_names(&self) -> impl Iterator<Item = &Identifier> {
        self.reserved.keys()
    }

    pub fn supertypes(&self) -> impl Iterator<Item = &Identifier> {
        self.supertypes.iter()
    }

    pub fn word(&self) -> Option<&Identifier> {
        self.word.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

regex_is_valid!(pub is_valid_identifier, r"^[a-zA-Z_]\w*");

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::reader::{GrammarFile, InputFile};

    #[test]
    fn test_load_example_file() {
        let result = GrammarFile::from_file(format!("./tests/{}", GrammarFile::DEFAULT_FILE_NAME));
        assert!(result.is_ok());
    }

    #[test]
    fn test_all_rules() {
        let grammar =
            GrammarFile::from_file(format!("./tests/{}", GrammarFile::DEFAULT_FILE_NAME)).unwrap();
        println!("Name: {}", grammar.name());
        println!("Rules: {:?}", grammar.rule_names().collect::<Vec<_>>());
    }
}
