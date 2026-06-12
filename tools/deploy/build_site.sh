#!/usr/bin/env bash
# Build, verify, and assemble the public site into _site/.
#
# The single definition of a site build — used verbatim by
# .github/workflows/deploy-site.yml and runnable locally as a dry run:
#
#   bash tools/deploy/build_site.sh [--skip-headless] [--out DIR]
#
# Stages:
#   1. toolchain gate    — wasm-bindgen CLI must match Cargo.lock
#   2. build             — rspice-ui (IDE, bin target) + rspice-wasm
#                          (playground, lib) for wasm32, release
#   3. assemble          — site/ verbatim + both pkg/ bundles + build.json
#   4. static gates      — \0asm magic + wasm-bindgen export signature
#   5. headless gate     — serve _site, load play/ in headless Chrome,
#                          require "engine ready" AND a completed solve
#
# Any failed gate exits non-zero; the workflow only publishes on success.
set -euo pipefail

OUT="_site"
SKIP_HEADLESS=0
while [ $# -gt 0 ]; do
  case "$1" in
    --skip-headless) SKIP_HEADLESS=1 ;;
    --out) OUT="$2"; shift ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
  shift
done

cd "$(git rev-parse --show-toplevel)"
TARGET=target/wasm32-unknown-unknown/release

# ── 1 · toolchain gate ──────────────────────────────────────────────────
LOCKED=$(grep -A1 '^name = "wasm-bindgen"$' Cargo.lock | grep '^version' | head -1 | cut -d'"' -f2)
HAVE=$(wasm-bindgen --version | awk '{print $2}')
if [ "$LOCKED" != "$HAVE" ]; then
  echo "FAIL: wasm-bindgen CLI is $HAVE but Cargo.lock pins $LOCKED" >&2
  exit 1
fi
echo "ok: wasm-bindgen CLI $HAVE matches Cargo.lock"

# ── 2 · build ───────────────────────────────────────────────────────────
echo "building rspice-ui (browser IDE, bin target)…"
cargo build -p rspice-ui --bins --target wasm32-unknown-unknown --release
echo "building rspice-wasm (engine playground)…"
cargo build -p rspice-wasm --lib --target wasm32-unknown-unknown --release

# ── 3 · assemble ────────────────────────────────────────────────────────
rm -rf "$OUT"
mkdir -p "$OUT"
cp -r site/. "$OUT"/
rm -f "$OUT/README.md"

wasm-bindgen "$TARGET/rspice-ui.wasm" \
  --out-dir "$OUT/ide/pkg" --out-name rspice-ui --target web --no-typescript
wasm-bindgen "$TARGET/rspice_wasm.wasm" \
  --out-dir "$OUT/play/pkg" --out-name rspice_wasm --target web --no-typescript

SHA=$(git rev-parse HEAD)
printf '{"source_sha":"%s","built_utc":"%s","wasm_bindgen":"%s"}\n' \
  "$SHA" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$HAVE" > "$OUT/build.json"

# ── 4 · static gates ────────────────────────────────────────────────────
for stem in ide/pkg/rspice-ui play/pkg/rspice_wasm; do
  wasm="$OUT/${stem}_bg.wasm"
  js="$OUT/${stem}.js"
  magic=$(head -c 4 "$wasm" | od -An -tx1 | tr -d ' \n')
  if [ "$magic" != "0061736d" ]; then
    echo "FAIL: $wasm is not a wasm module (magic $magic)" >&2
    exit 1
  fi
  if ! grep -q 'export { initSync, __wbg_init as default }' "$js"; then
    echo "FAIL: $js is missing the wasm-bindgen export signature" >&2
    exit 1
  fi
  echo "ok: $stem ($(du -h "$wasm" | cut -f1 | tr -d ' ') wasm)"
done

# ── 5 · headless solve gate ─────────────────────────────────────────────
# play/index.html runs a transient on load; a healthy bundle yields the
# "engine ready" badge and a "solved in" log line in the settled DOM.
if [ "$SKIP_HEADLESS" = "1" ]; then
  echo "skipped: headless solve gate (--skip-headless)"
  exit 0
fi

CHROME_BIN="${CHROME:-}"
if [ -z "$CHROME_BIN" ]; then
  for candidate in google-chrome chromium-browser chromium \
    "/c/Program Files/Google/Chrome/Application/chrome.exe"; do
    if command -v "$candidate" >/dev/null 2>&1; then CHROME_BIN="$candidate"; break; fi
  done
fi
if [ -z "$CHROME_BIN" ]; then
  echo "FAIL: no Chrome found for the headless gate (set CHROME or pass --skip-headless)" >&2
  exit 1
fi

# Windows ships non-functional Store stubs named python/python3 — probe
# that the candidate actually runs, don't trust command -v.
PY=""
for candidate in python3 python py; do
  if "$candidate" --version >/dev/null 2>&1; then PY="$candidate"; break; fi
done
if [ -z "$PY" ]; then
  echo "FAIL: no working Python for the local server (pass --skip-headless)" >&2
  exit 1
fi
PORT=8137
"$PY" -m http.server "$PORT" --directory "$OUT" >/dev/null 2>&1 &
SERVER=$!
trap 'kill $SERVER 2>/dev/null || true' EXIT
sleep 1

DOM=$(mktemp)
timeout 90 "$CHROME_BIN" --headless=new --disable-gpu --no-sandbox \
  --virtual-time-budget=20000 --dump-dom \
  "http://127.0.0.1:$PORT/play/" > "$DOM" 2>/dev/null

if ! grep -q "engine ready" "$DOM"; then
  echo "FAIL: playground never reached 'engine ready'" >&2
  grep -o "module failed\|init failed[^<]*" "$DOM" >&2 || true
  exit 1
fi
if ! grep -q "solved in" "$DOM"; then
  echo "FAIL: playground loaded but the on-load transient did not solve" >&2
  grep -o "tran error[^<]*" "$DOM" >&2 || true
  exit 1
fi
rm -f "$DOM"
echo "ok: headless solve — engine ready, transient completed"
echo "site assembled at $OUT (source $SHA)"
