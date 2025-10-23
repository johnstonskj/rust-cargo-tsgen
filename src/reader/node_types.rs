/*!
One-line description.

 */

use crate::{error::Error, reader::InputFile};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::BufReader,
    path::Path,
};
use tracing::{error, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeTypesFile(Vec<NodeTypeDefinition>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeTypeDefinition {
    #[serde(flatten)]
    node_type: NodeType,
    #[serde(flatten)]
    definition_kind: NodeTypeDefinitionKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeType {
    #[serde(rename = "type")]
    node_type: String,
    named: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeTypeDefinitionKind {
    SuperType(SuperTypeNodeDefinition),
    Regular(RegularNodeDefinition),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuperTypeNodeDefinition {
    subtypes: Vec<NodeType>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegularNodeDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fields: Option<BTreeMap<String, NodeChildren>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    children: Option<NodeChildren>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeChildren {
    multiple: bool,
    required: bool,
    types: Vec<NodeType>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ NodeTypeDefinition
// ------------------------------------------------------------------------------------------------

impl From<Vec<NodeTypeDefinition>> for NodeTypesFile {
    fn from(definitions: Vec<NodeTypeDefinition>) -> Self {
        Self(definitions)
    }
}

impl From<&[NodeTypeDefinition]> for NodeTypesFile {
    fn from(definitions: &[NodeTypeDefinition]) -> Self {
        Self(definitions.to_vec())
    }
}

impl InputFile for NodeTypesFile {
    const DEFAULT_DIRECTORY: &str = "src";
    const DEFAULT_FILE_NAME: &str = "node-types.json";

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file_path = path.as_ref().display().to_string();
        trace!("NodeTypesFile::from_file({file_path})");
        println!("NodeTypesFile::from_file({file_path})");
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

impl NodeTypesFile {
    pub fn definitions(&self) -> impl Iterator<Item = &NodeTypeDefinition> {
        self.0.iter()
    }

    pub fn super_type_definitions(&self) -> impl Iterator<Item = &NodeTypeDefinition> {
        self.0.iter().filter(|defn| defn.kind().is_super_type())
    }

    pub fn regular_definitions(&self) -> impl Iterator<Item = &NodeTypeDefinition> {
        self.0.iter().filter(|defn| defn.kind().is_regular())
    }

    pub fn terminal_definitions(&self) -> impl Iterator<Item = &NodeTypeDefinition> {
        self.0.iter().filter(|defn| defn.kind().is_terminal())
    }

    pub fn has_definitions(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn definition_count(&self) -> usize {
        self.0.len()
    }

    pub fn node_type_names(&self) -> BTreeSet<&String> {
        self.definitions()
            .map(|defn| defn.node_type_name())
            .collect()
    }

    pub fn super_type_node_type_names(&self) -> BTreeSet<&String> {
        self.super_type_definitions()
            .map(|defn| defn.node_type_name())
            .collect()
    }

    pub fn regular_node_type_names(&self) -> BTreeSet<&String> {
        self.regular_definitions()
            .map(|defn| defn.node_type_name())
            .collect()
    }

    pub fn terminal_node_type_names(&self) -> BTreeSet<&String> {
        self.terminal_definitions()
            .map(|defn| defn.node_type_name())
            .collect()
    }

    pub fn field_names(&self) -> BTreeSet<&String> {
        self.regular_definitions()
            .filter_map(|defn| defn.kind().as_regular().map(|defn| defn.field_names()))
            .flatten()
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ NodeTypeDefinition
// ------------------------------------------------------------------------------------------------

impl NodeTypeDefinition {
    pub fn new<K>(node_type: NodeType, definition_kind: K) -> Self
    where
        K: Into<NodeTypeDefinitionKind>,
    {
        Self {
            node_type,
            definition_kind: definition_kind.into(),
        }
    }

    pub fn new_named<S, K>(node_type: S, definition_kind: K) -> Self
    where
        S: Into<String>,
        K: Into<NodeTypeDefinitionKind>,
    {
        Self::new(NodeType::new_named(node_type), definition_kind)
    }

    pub fn new_unnamed<S, K>(node_type: S, definition_kind: K) -> Self
    where
        S: Into<String>,
        K: Into<NodeTypeDefinitionKind>,
    {
        Self::new(NodeType::new_unnamed(node_type), definition_kind)
    }

    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn node_type_name(&self) -> &String {
        self.node_type.node_type()
    }

    pub fn kind(&self) -> &NodeTypeDefinitionKind {
        &self.definition_kind
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ NodeType
// ------------------------------------------------------------------------------------------------

impl NodeType {
    pub fn new<S>(node_type: S, named: bool) -> Self
    where
        S: Into<String>,
    {
        Self {
            node_type: node_type.into(),
            named,
        }
    }

    pub fn new_named<S>(node_type: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(node_type, true)
    }

    pub fn new_unnamed<S>(node_type: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(node_type, false)
    }

    pub fn node_type(&self) -> &String {
        &self.node_type
    }

    pub fn is_named(&self) -> bool {
        self.named
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ NodeTypeDefinitionKind
// ------------------------------------------------------------------------------------------------

impl From<SuperTypeNodeDefinition> for NodeTypeDefinitionKind {
    fn from(defn: SuperTypeNodeDefinition) -> Self {
        Self::SuperType(defn)
    }
}

impl From<RegularNodeDefinition> for NodeTypeDefinitionKind {
    fn from(defn: RegularNodeDefinition) -> Self {
        Self::Regular(defn)
    }
}

impl NodeTypeDefinitionKind {
    pub fn is_super_type(&self) -> bool {
        matches!(self, Self::SuperType(_))
    }

    pub fn as_super_type(&self) -> Option<&SuperTypeNodeDefinition> {
        match self {
            Self::SuperType(defn) => Some(defn),
            _ => None,
        }
    }

    pub fn is_regular(&self) -> bool {
        match self {
            Self::Regular(defn) => !defn.is_terminal(),
            _ => false,
        }
    }

    pub fn as_regular(&self) -> Option<&RegularNodeDefinition> {
        match self {
            Self::Regular(defn) => Some(defn),
            _ => None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        match self {
            Self::Regular(defn) => defn.is_terminal(),
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ SuperTypeNodeDefinition
// ------------------------------------------------------------------------------------------------

impl From<Vec<NodeType>> for SuperTypeNodeDefinition {
    fn from(subtypes: Vec<NodeType>) -> Self {
        Self { subtypes }
    }
}

impl From<&[NodeType]> for SuperTypeNodeDefinition {
    fn from(subtypes: &[NodeType]) -> Self {
        Self {
            subtypes: subtypes.to_vec(),
        }
    }
}

impl SuperTypeNodeDefinition {
    pub fn subtypes(&self) -> impl Iterator<Item = &NodeType> {
        self.subtypes.iter()
    }

    pub fn has_subtypes(&self) -> bool {
        !self.subtypes.is_empty()
    }

    pub fn subtype_count(&self) -> usize {
        self.subtypes.len()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ RegularNodeDefinition
// ------------------------------------------------------------------------------------------------

impl RegularNodeDefinition {
    pub fn terminal() -> Self {
        Self {
            fields: None,
            children: None,
        }
    }

    pub fn regular(
        fields: Option<BTreeMap<String, NodeChildren>>,
        children: Option<NodeChildren>,
    ) -> Self {
        Self { fields, children }
    }

    pub fn is_terminal(&self) -> bool {
        self.fields.is_none() && self.children.is_none()
    }

    pub fn fields(&self) -> Option<&BTreeMap<String, NodeChildren>> {
        self.fields.as_ref()
    }

    pub fn has_fields(&self) -> bool {
        self.fields().map(|map| !map.is_empty()).unwrap_or_default()
    }

    pub fn field_names(&self) -> Box<dyn Iterator<Item = &String> + '_> {
        if let Some(field_map) = &self.fields {
            Box::new(field_map.keys())
        } else {
            Box::new(std::iter::empty())
        }
    }

    pub fn children(&self) -> Option<&NodeChildren> {
        self.children.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ NodeChildren
// ------------------------------------------------------------------------------------------------

impl NodeChildren {
    pub fn new(multiple: bool, required: bool, types: Vec<NodeType>) -> Self {
        Self {
            multiple,
            required,
            types,
        }
    }

    pub fn is_multiple(&self) -> bool {
        self.multiple
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    pub fn types(&self) -> impl Iterator<Item = &NodeType> {
        self.types.iter()
    }

    pub fn has_types(&self) -> bool {
        !self.types.is_empty()
    }

    pub fn type_count(&self) -> usize {
        self.types.len()
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    /*
    use pretty_assertions::assert_eq;
     */
    use crate::reader::{InputFile, NodeTypesFile};

    #[test]
    fn test_load_example_file() {
        let result =
            NodeTypesFile::from_file(format!("./tests/{}", NodeTypesFile::DEFAULT_FILE_NAME));
        assert!(result.is_ok());
    }

    #[test]
    fn test_loaded_node_names() {
        let file =
            NodeTypesFile::from_file(format!("./tests/{}", NodeTypesFile::DEFAULT_FILE_NAME))
                .unwrap();
        println!("All: {:#?}", file.node_type_names());
        println!("Super-Types: {:#?}", file.super_type_node_type_names());
        println!("Regular: {:#?}", file.regular_node_type_names());
        println!("Terminal: {:#?}", file.terminal_node_type_names());
    }

    #[test]
    fn test_loaded_field_names() {
        let file =
            NodeTypesFile::from_file(format!("./tests/{}", NodeTypesFile::DEFAULT_FILE_NAME))
                .unwrap();
        println!("{:#?}", file.field_names());
    }
}
