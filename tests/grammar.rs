#![allow(dead_code)]
use tree_sitter::{Node, Tree};

mod nodes;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! root_node {
    ($node_name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $node_name<'s> {
            tree: Tree,
            source: &'s [u8],
        }

        impl<'s> TypedRootNode<'s> for $node_name<'s> {
            fn from_tree(tree: Tree, source: &'s [u8]) -> Self {
                Self { tree, source }
            }
        }

        impl<'s> $node_name<'s> {
            fn tree(&self) -> &Tree {
                &self.tree
            }

            fn node<'t>(&'t self) -> Node<'t> {
                self.tree.root_node()
            }
         }
    };
}

macro_rules! compound_node {
    ($node_name:ident) => {
        // ----------------------------------------------------------------------------------------
        // Value Node :: $node_name
        // ----------------------------------------------------------------------------------------

        #[derive(Clone, Debug, PartialEq)]
        pub struct $node_name<'t, 's> {
            node: Node<'t>,
            source: &'s [u8],
        }

        impl<'t, 's> TypedNode<'t, 's> for $node_name<'t, 's> {
            fn from_node(node: Node<'t>, source: &'s [u8]) -> Self {
                Self { node, source }
            }
        }

        impl<'t, 's> $node_name<'t, 's> {
            fn node(&self) -> &Node<'t> {
                &self.node
            }
        }
    };
}

macro_rules! field {
    ($name:ident => root $node_type:ty) => {
        pastey::paste! {
            pub fn [< field_ $name >]<'t>(&'t self) -> $node_type<'t, 's> {
                field!(@required $name, self.tree.root_node(), self.source => $node_type)
            }
        }
    };
    ($name:ident => $node_type:ty) => {
        pastey::paste! {
            pub fn [< field_ $name >](&'t self) -> $node_type<'t, 's> {
                field!(@required $name, self.node(), self.source => $node_type)
            }
        }
    };
    ($name:ident => root value $node_type:ty) => {
        pastey::paste! {
            pub fn [< field_ $name >](&self) -> $node_type {
                field!(@required $name, self.tree.root_node(), self.source => $node_type)
            }
        }
    };
    ($name:ident => value $node_type:ty) => {
        pastey::paste! {
            pub fn [< field_ $name >](&self) -> $node_type {
                field!(@required $name, self.node(), self.source => $node_type)
            }
        }
    };
    ($name:ident => root optional $node_type:ty) => {
        pastey::paste! {
            pub fn [< field_ $name >]<'t>(&'t self) -> Option<$node_type<'t, 's>> {
                field!(@optional $name, self.tree.root_node(), self.source => $node_type)
            }
        }
    };
    ($name:ident => optional $node_type:ty) => {
        pastey::paste! {
            pub fn [< field_ $name >](&self) -> Option<$node_type<'t, 's>> {
                field!(@optional $name, self.node(), self.source => $node_type)
            }
        }
    };
    ($name:ident => root optional value $node_type:ty) => {
        pastey::paste! {
            pub fn [< field_ $name >](&self) -> Option<$node_type> {
                field!(@optional $name, self.tree.root_node(), self.source => $node_type)
            }
        }
    };
    ($name:ident => optional value $node_type:ty) => {
        pastey::paste! {
            pub fn [< field_ $name >](&self) -> Option<$node_type> {
                field!(@optional $name, self.node(), self.source => $node_type)
            }
        }
    };
    (@required $name:ident, $node:expr, $source:expr => $node_type:ty) => {
        pastey::paste! {{
            let field_name = nodes::[< FIELD_ $name:snake:upper >];
            let node = $node;
            let child = node
                .child_by_field_name(field_name)
                .expect("Missing required field named {field_name}");
            $node_type::from_node(child, $source)
        }}
    };
    (@optional $name:ident, $node:expr, $source:expr => $node_type:ty) => {
        pastey::paste! {{
            let field_name = nodes::[< FIELD_ $name:snake:upper >];
            let node = $node;
            if let Some(child) = node.child_by_field_name(field_name) {
                Some($node_type::from_node(child, $source))
            } else {
                None
            }
        }}
    };
}

macro_rules! value_node {
    ($node_name:ident) => {
        // ----------------------------------------------------------------------------------------
        // Value Node :: $node_name
        // ----------------------------------------------------------------------------------------

        #[derive(Clone, Debug, PartialEq)]
        pub struct $node_name(String); /* (rule type: STRING (value) @value) */

        impl From<$node_name> for String {
            fn from(node: $node_name) -> Self {
                node.0
            }
        }

        impl AsRef<str> for $node_name {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl<'t, 's> TypedNode<'t, 's> for $node_name {
            fn from_node(node: Node<'t>, source: &'s [u8]) -> Self
            where
                Self: Sized {
                Self(
                    node.utf8_text(source)
                        .expect("Could not convert Node content into string value.")
                        .to_string()
                )
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Traits
// ------------------------------------------------------------------------------------------------

pub trait TypedNode<'t, 's> {
    fn from_node(node: Node<'t>, source: &'s [u8]) -> Self
    where
        Self: Sized;
}

pub trait TypedRootNode<'s> {
    fn from_tree(tree: Tree, source: &'s [u8]) -> Self
    where
        Self: Sized;
}

// ------------------------------------------------------------------------------------------------
// Root Node
// ------------------------------------------------------------------------------------------------

root_node!(ModuleNode);

impl<'s> ModuleNode<'s> {
    field!(name => root value IdentifierValue);

    field!(base => root optional IriNode);

    field!(body => root ModuleBodyNode);

    pub fn member_module_version<'t>(&'t self) -> Option<ModuleVersionNode<'t, 's>> {
        let node_type = nodes::NODE_TYPE_MODULE_VERSION;
        let node = self.tree.root_node();
        for child in node.named_children(&mut node.walk()) {
            if child.grammar_name() == node_type {
                return Some(ModuleVersionNode::from_node(child, self.source))
            }
        }
        None
    }

    pub fn members<'t>(
        &'t self,
    ) -> (
        IdentifierValue,
        Option<IriNode<'t, 's>>,
        Option<ModuleVersionNode<'t, 's>>,
        ModuleBodyNode<'t, 's>,
    ) {
        (
            self.field_name(),
            self.field_base(),
            self.member_module_version(),
            self.field_body(),
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Compound Nodes
// ------------------------------------------------------------------------------------------------

compound_node!(ModuleVersionNode);

compound_node!(ModuleBodyNode);

compound_node!(IriNode);

// ------------------------------------------------------------------------------------------------
// Value Nodes
// ------------------------------------------------------------------------------------------------

value_node!(IdentifierValue);

value_node!(QuotedStringValue);

value_node!(TokenValue);
