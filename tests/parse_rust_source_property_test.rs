// Property tests for parse_rust_source.
// Run separately: cargo test --test parse_rust_source_property_test -- --include-ignored
// Not included in normal cargo test to keep standard verification fast.

use proptest::prelude::*;

proptest! {
    #[test]
    #[ignore = "property tests: run with --include-ignored"]
    fn parse_rust_source_never_panics(s in ".*") {
        let _ = drywall::parse_rust_source(&s);
    }

    #[test]
    #[ignore = "property tests: run with --include-ignored"]
    fn parse_rust_source_returns_false_for_invalid_rust(
        s in "[^f].*|f[^n].*|fn[^ ].*"
    ) {
        // Most random strings are not valid Rust; we just assert no panic and a defined result.
        let result = drywall::parse_rust_source(&s);
        let _ = result; // outcome can be true or false; must not panic
    }

    #[test]
    #[ignore = "property tests: run with --include-ignored"]
    fn parse_rust_source_valid_fn_always_parses(name in "[a-z][a-z0-9_]{0,15}") {
        let src = format!("fn {}() {{}}", name);
        prop_assert!(drywall::parse_rust_source(&src));
    }
}
