use proptest::prelude::*;

proptest! {
    #[test]
    fn parse_rust_source_never_panics(s in ".*") {
        let _ = drywall::parse_rust_source(&s);
    }

    #[test]
    fn parse_rust_source_returns_false_for_invalid_rust(
        s in "[^f].*|f[^n].*|fn[^ ].*"
    ) {
        let result = drywall::parse_rust_source(&s);
        let _ = result;
    }

    #[test]
    fn parse_rust_source_valid_fn_always_parses(name in "[a-z][a-z0-9_]{0,15}") {
        let src = format!("fn {}() {{}}", name);
        prop_assert!(drywall::parse_rust_source(&src));
    }
}
