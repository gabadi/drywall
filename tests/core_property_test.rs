use drywall::{
    DuplicatePair, FunctionInfo, PairEndpoint, find_duplicate_pairs, format_json, format_text,
    jaccard, source_lines,
};
use proptest::prelude::*;

fn make_fn(file: &str, start: usize, end: usize, hashes: Vec<u64>) -> FunctionInfo {
    FunctionInfo {
        file: file.to_string(),
        start_line: start,
        end_line: end,
        node_hashes: hashes,
    }
}

proptest! {
    #[test]
    fn jaccard_symmetry(
        a in prop::collection::vec(0u64..1000, 0..20),
        b in prop::collection::vec(0u64..1000, 0..20),
    ) {
        let score_ab = jaccard(&a, &b);
        let score_ba = jaccard(&b, &a);
        prop_assert!((score_ab - score_ba).abs() < 1e-9);
    }

    #[test]
    fn jaccard_identity_nonempty(a in prop::collection::vec(1u64..1000, 1..20)) {
        let score = jaccard(&a, &a);
        prop_assert!((score - 1.0).abs() < 1e-9);
    }

    #[test]
    fn jaccard_bounds(
        a in prop::collection::vec(0u64..1000, 0..20),
        b in prop::collection::vec(0u64..1000, 0..20),
    ) {
        let score = jaccard(&a, &b);
        prop_assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn find_duplicate_pairs_sorted_descending(
        threshold in 0.0f64..=1.0,
        seed in 0u64..100,
    ) {
        let f1 = make_fn("a.rs", 1, 20, (seed..seed+15).collect());
        let f2 = make_fn("b.rs", 1, 20, (seed..seed+15).collect());
        let f3 = make_fn("c.rs", 1, 20, ((seed+50)..(seed+65)).collect());
        let f4 = make_fn("d.rs", 1, 20, ((seed+50)..(seed+65)).collect());
        let fns = vec![f1, f2, f3, f4];
        let pairs = find_duplicate_pairs(&fns, threshold, 1, 1);
        for window in pairs.windows(2) {
            prop_assert!(window[0].score >= window[1].score);
        }
    }

    #[test]
    fn find_duplicate_pairs_no_self_pairs(
        start in 1usize..100,
        len in 5usize..20,
        threshold in 0.0f64..0.5,
    ) {
        let end = start + len;
        let hashes: Vec<u64> = (0..10).collect();
        let f = make_fn("a.rs", start, end, hashes);
        let pairs = find_duplicate_pairs(&[f], threshold, 1, 1);
        prop_assert_eq!(pairs.len(), 0);
    }

    #[test]
    fn find_duplicate_pairs_same_location_skipped(
        start in 1usize..100,
        len in 5usize..20,
    ) {
        let end = start + len;
        let hashes: Vec<u64> = (0..10).collect();
        let f1 = make_fn("a.rs", start, end, hashes.clone());
        let f2 = make_fn("a.rs", start, end, hashes);
        let pairs = find_duplicate_pairs(&[f1, f2], 0.0, 1, 1);
        prop_assert_eq!(pairs.len(), 0, "same-location pair must be excluded");
    }

    #[test]
    fn find_duplicate_pairs_threshold_monotone(
        seed in 0u64..50,
        lo in 0.0f64..0.5,
        hi in 0.5f64..=1.0,
    ) {
        let f1 = make_fn("a.rs", 1, 20, (seed..seed+15).collect());
        let f2 = make_fn("b.rs", 1, 20, (seed..seed+12).collect());
        let f3 = make_fn("c.rs", 1, 20, ((seed+50)..(seed+65)).collect());
        let fns = vec![f1, f2, f3];
        let count_lo = find_duplicate_pairs(&fns, lo, 1, 1).len();
        let count_hi = find_duplicate_pairs(&fns, hi, 1, 1).len();
        prop_assert!(count_lo >= count_hi, "raising threshold must not increase pair count");
    }

    #[test]
    fn format_json_always_valid(
        score in 0.0f64..=1.0,
        start in 1usize..100,
        end in 1usize..100,
        nodes in 1usize..50,
    ) {
        let pair = DuplicatePair {
            score,
            left: PairEndpoint { file: "a.rs".to_string(), start_line: start, end_line: end, nodes },
            right: PairEndpoint { file: "b.rs".to_string(), start_line: start, end_line: end, nodes },
        };
        let json = format_json(&[pair]);
        prop_assert!(serde_json::from_str::<serde_json::Value>(&json).is_ok(), "format_json must produce valid JSON");
    }

    #[test]
    fn format_text_contains_duplicate_header(
        score in 0.0f64..=1.0,
        start in 1usize..100,
        end in 1usize..100,
        nodes in 1usize..50,
    ) {
        let pair = DuplicatePair {
            score,
            left: PairEndpoint { file: "a.rs".to_string(), start_line: start, end_line: end, nodes },
            right: PairEndpoint { file: "b.rs".to_string(), start_line: start, end_line: end, nodes },
        };
        let text = format_text(&[pair]);
        prop_assert!(text.contains("DUPLICATE score="), "format_text must include DUPLICATE header");
    }

    #[test]
    fn source_lines_at_least_one_when_end_gte_start(
        start in 1usize..500,
        extra in 0usize..500,
    ) {
        let end = start + extra;
        let fi = FunctionInfo { file: "f.rs".to_string(), start_line: start, end_line: end, node_hashes: vec![] };
        prop_assert!(source_lines(&fi) >= 1, "source_lines must be >= 1 when end >= start");
    }

    #[test]
    fn jaccard_subset_monotone(
        shared in prop::collection::vec(1u64..500, 1..15),
        extra_b in prop::collection::vec(500u64..1000, 0..10),
    ) {
        // score(shared, shared) >= score(shared, shared ∪ extra_b)
        // Adding elements to B that are not in A reduces or preserves the score.
        let a = shared.clone();
        let b_small = shared.clone();
        let b_large: Vec<u64> = shared.iter().copied().chain(extra_b.iter().copied()).collect();
        let score_small = jaccard(&a, &b_small);
        let score_large = jaccard(&a, &b_large);
        prop_assert!(
            score_small >= score_large - 1e-9,
            "adding non-overlapping elements to B must not increase jaccard score: small={} large={}",
            score_small, score_large
        );
    }

    #[test]
    fn find_duplicate_pairs_endpoint_order_stable(
        seed in 1u64..50,
        start_a in 1usize..20,
        start_b in 21usize..40,
    ) {
        // Pairs produced from (f_a, f_b) and (f_b, f_a) must have the same left/right assignment.
        let hashes: Vec<u64> = (seed..seed + 15).collect();
        let f_a = make_fn("alpha.rs", start_a, start_a + 9, hashes.clone());
        let f_b = make_fn("beta.rs", start_b, start_b + 9, hashes.clone());

        let pairs_ab = find_duplicate_pairs(&[f_a.clone(), f_b.clone()], 0.0, 1, 1);
        let pairs_ba = find_duplicate_pairs(&[f_b.clone(), f_a.clone()], 0.0, 1, 1);

        prop_assert_eq!(pairs_ab.len(), pairs_ba.len(), "pair count must be same regardless of input order");
        if !pairs_ab.is_empty() {
            prop_assert_eq!(
                &pairs_ab[0].left.file, &pairs_ba[0].left.file,
                "left endpoint must be consistent regardless of input order"
            );
            prop_assert_eq!(
                &pairs_ab[0].right.file, &pairs_ba[0].right.file,
                "right endpoint must be consistent regardless of input order"
            );
        }
    }
}
