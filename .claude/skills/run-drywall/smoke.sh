#!/usr/bin/env bash
# Smoke test for drywall CLI — run from repo root
set -u

BINARY=./target/release/drywall
PASS=0
FAIL=0

ok()   { echo "  PASS: $*"; PASS=$((PASS+1)); }
fail() { echo "  FAIL: $*"; FAIL=$((FAIL+1)); }

echo "=== drywall smoke ==="

# 1. Binary exists and help works
if $BINARY --help &>/dev/null; then
  ok "--help exits 0"
else
  fail "--help exited non-zero"
fi

# 2. Clean directory → exit 0
$BINARY src/ >/dev/null 2>&1; code=$?
if [ "$code" = "0" ]; then
  ok "clean src/ exits 0"
else
  fail "clean src/ exited $code (expected 0)"
fi

# 3. Duplicate Rust file → exit 1, DUPLICATE on stdout
tmprs=$(mktemp /tmp/drywall_smoke_XXXXXX); mv "$tmprs" "${tmprs}.rs"; tmprs="${tmprs}.rs"
cat > "$tmprs" << 'RUST'
fn compute_area_rectangle(width: f64, height: f64) -> f64 {
    let perimeter = 2.0 * (width + height);
    let area = width * height;
    let result = area + perimeter * 0.0;
    result
}

fn calculate_rect_area(w: f64, h: f64) -> f64 {
    let perim = 2.0 * (w + h);
    let area = w * h;
    let out = area + perim * 0.0;
    out
}
RUST

$BINARY "$tmprs" >/dev/null 2>&1; code=$?
if [ "$code" = "1" ]; then
  ok "duplicate Rust exits 1"
else
  fail "duplicate Rust exited $code (expected 1)"
fi

stdout=$($BINARY "$tmprs" 2>/dev/null || true)
if echo "$stdout" | grep -q "DUPLICATE"; then
  ok "text output contains DUPLICATE"
else
  fail "text output missing DUPLICATE keyword"
fi

# 4. --format json → valid JSON array on stdout
json=$($BINARY "$tmprs" --format json 2>/dev/null || true)
if echo "$json" | python3 -c "import sys,json; d=json.load(sys.stdin); assert len(d)>0" 2>/dev/null; then
  ok "--format json produces non-empty JSON array"
else
  fail "--format json did not produce valid/non-empty JSON"
fi

# 5. Duplicate Python file → exit 1
tmppy=$(mktemp /tmp/drywall_smoke_XXXXXX); mv "$tmppy" "${tmppy}.py"; tmppy="${tmppy}.py"
cat > "$tmppy" << 'PYTHON'
def compute_area_rectangle(width, height):
    perimeter = 2.0 * (width + height)
    area = width * height
    result = area + perimeter * 0.0
    return result

def calculate_rect_area(w, h):
    perim = 2.0 * (w + h)
    area = w * h
    out = area + perim * 0.0
    return out
PYTHON

$BINARY "$tmppy" >/dev/null 2>&1; code=$?
if [ "$code" = "1" ]; then
  ok "duplicate Python exits 1"
else
  fail "duplicate Python exited $code (expected 1)"
fi

# 6. Bad --lang value → exit 2
$BINARY src/ --lang=cobol 2>/dev/null; code=$?
if [ "$code" = "2" ]; then
  ok "unknown --lang exits 2"
else
  fail "unknown --lang exited $code (expected 2)"
fi

# 7. --format json on clean dir → empty array, exit 0
$BINARY src/ --format json >/dev/null 2>&1; code=$?
if [ "$code" = "0" ]; then
  ok "clean dir with --format json exits 0"
else
  fail "clean dir with --format json exited $code (expected 0)"
fi

# Cleanup
rm -f "$tmprs" "$tmppy"

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
[ "$FAIL" -eq 0 ]
