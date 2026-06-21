use drywall::Lang;
use drywall::ast::{
    JS_CONFIG, RUST_CONFIG, TS_CONFIG, build_normalized_subtree, extract_functions,
    extract_functions_with_config, make_parser, make_parser_for,
};
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

    #[test]
    fn js_extraction_deterministic(name in "[a-z][a-z0-9_]{0,8}") {
        let src = format!(
            "function {0}(a, b) {{\n  let c = a + b;\n  let d = c * 2;\n  let e = d + a;\n  return e;\n}}\n",
            name
        );
        let mut p1 = make_parser_for(Lang::JavaScript);
        let mut p2 = make_parser_for(Lang::JavaScript);
        let tree1 = p1.parse(&src, None).unwrap();
        let tree2 = p2.parse(&src, None).unwrap();
        let mut fns1 = Vec::new();
        let mut fns2 = Vec::new();
        extract_functions_with_config(tree1.root_node(), &src, "f.js", &JS_CONFIG, &mut fns1);
        extract_functions_with_config(tree2.root_node(), &src, "f.js", &JS_CONFIG, &mut fns2);
        prop_assert_eq!(fns1.len(), fns2.len(), "js extraction must be deterministic");
        for (f1, f2) in fns1.iter().zip(fns2.iter()) {
            prop_assert_eq!(&f1.node_hashes, &f2.node_hashes, "same js source must produce same hashes");
        }
    }

    #[test]
    fn ts_extraction_deterministic(name in "[a-z][a-z0-9_]{0,8}") {
        let src = format!(
            "function {0}(a: number, b: number): number {{\n  let c = a + b;\n  let d = c * 2;\n  let e = d + a;\n  return e;\n}}\n",
            name
        );
        let mut p1 = make_parser_for(Lang::TypeScript);
        let mut p2 = make_parser_for(Lang::TypeScript);
        let tree1 = p1.parse(&src, None).unwrap();
        let tree2 = p2.parse(&src, None).unwrap();
        let mut fns1 = Vec::new();
        let mut fns2 = Vec::new();
        extract_functions_with_config(tree1.root_node(), &src, "f.ts", &TS_CONFIG, &mut fns1);
        extract_functions_with_config(tree2.root_node(), &src, "f.ts", &TS_CONFIG, &mut fns2);
        prop_assert_eq!(fns1.len(), fns2.len(), "ts extraction must be deterministic");
        for (f1, f2) in fns1.iter().zip(fns2.iter()) {
            prop_assert_eq!(&f1.node_hashes, &f2.node_hashes, "same ts source must produce same hashes");
        }
    }

    #[test]
    fn js_normalized_subtree_deterministic(name in "[a-z][a-z0-9_]{0,8}") {
        let src = format!("function {}() {{}}", name);
        let mut parser = make_parser_for(Lang::JavaScript);
        let tree = parser.parse(&src, None).unwrap();
        let root = tree.root_node();
        let s1 = build_normalized_subtree(root, &src, &JS_CONFIG);
        let s2 = build_normalized_subtree(root, &src, &JS_CONFIG);
        prop_assert_eq!(s1, s2, "js build_normalized_subtree must be deterministic");
    }

    #[test]
    fn js_twin_functions_hashes_differ_from_ts(name in "[a-z][a-z0-9_]{0,8}") {
        // JS and TS grammars produce different node kinds for typed parameters,
        // so their hash multisets must differ for typed functions.
        let js_src = format!(
            "function {0}(a, b) {{\n  let c = a + b;\n  let d = c * 2;\n  return d;\n}}\n",
            name
        );
        let ts_src = format!(
            "function {0}(a: number, b: number): number {{\n  let c = a + b;\n  let d = c * 2;\n  return d;\n}}\n",
            name
        );
        let mut js_parser = make_parser_for(Lang::JavaScript);
        let mut ts_parser = make_parser_for(Lang::TypeScript);
        let js_tree = js_parser.parse(&js_src, None);
        let ts_tree = ts_parser.parse(&ts_src, None);
        // Both must parse without error (grammar sanity)
        prop_assume!(js_tree.is_some() && ts_tree.is_some());
        let js_tree = js_tree.unwrap();
        let ts_tree = ts_tree.unwrap();
        prop_assume!(!js_tree.root_node().has_error() && !ts_tree.root_node().has_error());
        let mut js_fns = Vec::new();
        let mut ts_fns = Vec::new();
        extract_functions_with_config(js_tree.root_node(), &js_src, "f.js", &JS_CONFIG, &mut js_fns);
        extract_functions_with_config(ts_tree.root_node(), &ts_src, "f.ts", &TS_CONFIG, &mut ts_fns);
        prop_assert_eq!(js_fns.len(), 1, "js must extract 1 function");
        prop_assert_eq!(ts_fns.len(), 1, "ts must extract 1 function");
        // TS hashes include type annotations; they must differ from the untyped JS hashes
        prop_assert_ne!(
            &js_fns[0].node_hashes, &ts_fns[0].node_hashes,
            "typed ts function must hash differently from untyped js function"
        );
    }
}
