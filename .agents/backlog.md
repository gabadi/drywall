# Curator Backlog

Enforcement-gate proposals (mechanical fixes that beat documentation). Append-only; each entry dated.

---

## 2026-06-17 | e1630652 | Equivalent mutant suppression for `find_duplicate_pairs`

**Source:** hardender retro (harden-rust-dup-detect), action #2  
**Failure-class:** convention-gap  
**Proposal:** Document the known equivalent mutant `j = i+1 → i*1` in `find_duplicate_pairs` in `src/core.rs` with a code comment or mutation suppression annotation (e.g. `// EQUIVALENT MUTANT: j=i+1→i*1; i+1 and i*1 are equal when i>0 in practice; is_same_location guards same-index pairs`). Route to coder/hardender — curator cannot touch code.  
**Why a gate beats docs:** Re-investigation cost each run; the mutant is mathematically equivalent and cannot be killed, so documenting it at the source prevents future wasted cycles.

---

## 2026-06-17 | ab98a5aa | `generate_entrypoint.py` hardcodes scaffold_cli feature path

**Source:** QA retro (qa-rust-dup-detect), action #3  
**Failure-class:** wrong-path  
**Proposal:** `acceptance/scripts/generate_entrypoint.py` hardcodes `feature_path = 'features/scaffold_cli.feature'` at line ~157. When processing multiple features, each run clobbers the metadata from the previous. Fix: accept `feature_path` as a CLI argument derived from the input feature file path rather than hardcoding. Route to coder.  
**Why a gate beats docs:** Silent metadata corruption — tests still pass but the JSON metadata for the first feature is permanently overwritten on each subsequent run. This will cause latent failures as more features are added.
