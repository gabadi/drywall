use drywall::ast::{RUST_CONFIG, build_normalized_subtree, extract_functions, make_parser};
use proptest::prelude::*;

proptest! {
    #[test]
    fn hashing_deterministic_for_valid_fn(name in "[a-z][a-z0-9_]{0,10}") {
        let src = format!("fn {}(x: i32) -> i32 {{ let a = x + 1; a * 2 }}", name);
        let mut p1 = make_parser();
        let mut p2 = make_parser();
        let tree1 = p1.parse(&src, None).unwrap();
        let tree2 = p2.parse(&src, None).unwrap();
        let mut fns1 = Vec::new();
        let mut fns2 = Vec::new();
        extract_functions(tree1.root_node(), &src, "test.rs", &mut fns1);
        extract_functions(tree2.root_node(), &src, "test.rs", &mut fns2);
        prop_assert_eq!(fns1.len(), fns2.len());
        for (f1, f2) in fns1.iter().zip(fns2.iter()) {
            prop_assert_eq!(&f1.node_hashes, &f2.node_hashes, "same source must produce same hashes");
        }
    }

    #[test]
    fn normalized_subtree_deterministic(name in "[a-z][a-z0-9_]{0,10}") {
        let src = format!("fn {}() {{}}", name);
        let mut parser = make_parser();
        let tree = parser.parse(&src, None).unwrap();
        let root = tree.root_node();
        let s1 = build_normalized_subtree(root, &src, &RUST_CONFIG);
        let s2 = build_normalized_subtree(root, &src, &RUST_CONFIG);
        prop_assert_eq!(s1, s2, "build_normalized_subtree must be deterministic");
    }

    #[test]
    fn extract_functions_count_stable(name in "[a-z][a-z0-9_]{0,8}") {
        let src = format!(
            "fn {0}(x: i32) -> i32 {{ x + 1 }}\nfn {0}_b(y: i32) -> i32 {{ y * 2 }}",
            name
        );
        let mut parser = make_parser();
        let tree = parser.parse(&src, None).unwrap();
        let mut fns1 = Vec::new();
        let mut fns2 = Vec::new();
        extract_functions(tree.root_node(), &src, "f.rs", &mut fns1);
        extract_functions(tree.root_node(), &src, "f.rs", &mut fns2);
        prop_assert_eq!(fns1.len(), fns2.len(), "repeated extraction must yield same count");
    }
}
