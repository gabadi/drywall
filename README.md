# drywall

Polyglot AST subtree DRY analyzer. Detects duplicate code across files using Jaccard similarity over normalized AST fingerprints.

## Install

Download the binary for your platform from the [latest release](https://github.com/gabadi/drywall/releases/latest):

| Platform | Binary |
|----------|--------|
| Linux x86_64 | `drywall-linux-x86_64` |
| macOS Apple Silicon | `drywall-macos-aarch64` |
| macOS Intel | `drywall-macos-x86_64` |

```bash
# Linux
curl -sL https://github.com/gabadi/drywall/releases/latest/download/drywall-linux-x86_64 -o drywall
chmod +x ./drywall

# macOS Apple Silicon
curl -sL https://github.com/gabadi/drywall/releases/latest/download/drywall-macos-aarch64 -o drywall
chmod +x ./drywall

# macOS Intel
curl -sL https://github.com/gabadi/drywall/releases/latest/download/drywall-macos-x86_64 -o drywall
chmod +x ./drywall
```

The Linux binary is statically linked (musl) — no runtime dependencies.

## Usage

```bash
drywall [--threshold 0.82] ./src
```

`--threshold` is the Jaccard similarity cutoff (default `0.82`, same as dry4go). Pairs at or above the threshold are reported. Exit codes: `0` = no duplicates, `1` = duplicates found, `2` = bad arguments.

## Use as a CI gate

Add to any project's CI pipeline:

```yaml
- name: Install drywall
  run: |
    curl -sL https://github.com/gabadi/drywall/releases/latest/download/drywall-linux-x86_64 -o drywall
    chmod +x ./drywall

- name: DRY check
  run: ./drywall --threshold 0.82 ./src
```

`--threshold` sets the Jaccard similarity cutoff (default `0.82`). Pairs at or above the threshold are flagged. The step fails (exit 1) if any duplicates are found.

## Build from source

Requires Rust 1.96+:

```bash
cargo build --release
./target/release/drywall ./src
```
