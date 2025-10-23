use tree_sitter::Parser;

mod grammar;
use grammar::{ModuleNode, TypedRootNode};

#[test]
fn test_construct_tree() {
    const TEST_SOURCE: &str = r#"module test is
end"#;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_sdml::LANGUAGE.into())
        .expect("Error loading SDML grammar");

    let tree = parser
        .parse(TEST_SOURCE, None)
        .expect("Could not parse test example");
    let module = ModuleNode::from_tree(tree, TEST_SOURCE.as_bytes());
    assert_eq!(module.field_name().as_ref(), "test");
    assert_eq!(module.field_base(), None);
    assert_eq!(module.member_module_version(), None);
}
