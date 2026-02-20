#!/usr/bin/env bash
set -euo pipefail

WASM_FILE=$(ls dist/*_bg.wasm)
JS_FILE=$(ls dist/*.js)
CSS_FILE=$(ls dist/style-*.css)

WASM_BYTES=$(wc -c < "$WASM_FILE")
JS_BYTES=$(wc -c < "$JS_FILE")
CSS_BYTES=$(wc -c < "$CSS_FILE")

MAX_WASM_BYTES=1400000
MAX_JS_BYTES=50000
MAX_CSS_BYTES=25000

echo "Bundle sizes:"
echo "  wasm: $WASM_BYTES bytes ($WASM_FILE)"
echo "  js:   $JS_BYTES bytes ($JS_FILE)"
echo "  css:  $CSS_BYTES bytes ($CSS_FILE)"

if (( WASM_BYTES > MAX_WASM_BYTES )); then
  echo "ERROR: wasm bundle exceeds budget ($WASM_BYTES > $MAX_WASM_BYTES)"
  exit 1
fi

if (( JS_BYTES > MAX_JS_BYTES )); then
  echo "ERROR: js bundle exceeds budget ($JS_BYTES > $MAX_JS_BYTES)"
  exit 1
fi

if (( CSS_BYTES > MAX_CSS_BYTES )); then
  echo "ERROR: css bundle exceeds budget ($CSS_BYTES > $MAX_CSS_BYTES)"
  exit 1
fi

echo "Bundle size checks passed."
