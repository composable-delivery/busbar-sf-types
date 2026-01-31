#!/usr/bin/env bash
set -euo pipefail

# Regenerate busbar-sf-types from the currently installed @salesforce/types.
#
# Intended usage:
#   1) npm install  -> ensures node_modules/@salesforce/types exists
#   2) ./scripts/generate.sh
#
# This script:
# - verifies prerequisites
# - runs the Rust generator (sf-typegen)
# - runs formatting + a quick compile check
# - highlights likely follow-up work (missing categories / overlays) based on generator output
# - copies machine-readable reports into a dedicated report directory for CI automation
# - records the installed `@salesforce/types` version in the report directory (for per-version rollup issues)
#
# Notes:
# - The generator default output dir is "../crates/sf-types/src", but we pass it explicitly here.
# - CI can run this and then fail if git diff is non-empty (enforcing committed generation).
# - Report files (JSON) are written by the generator to the repo root; we collect them under
#   `.typegen-reports/` so CI can upload or post-process them.
#
# @salesforce/types input path notes:
# - Some versions ship `src/metadata.ts`, others ship only `lib/metadata.d.ts` / `lib/metadata.js`.
# - This script prefers `src/metadata.ts` if present, otherwise falls back to `lib/metadata.d.ts`.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

OUTPUT_DIR="crates/sf-types/src"

# Determine the best available @salesforce/types input file.
if [[ -f "node_modules/@salesforce/types/src/metadata.ts" ]]; then
  INPUT_FILE="node_modules/@salesforce/types/src/metadata.ts"
elif [[ -f "node_modules/@salesforce/types/lib/metadata.d.ts" ]]; then
  INPUT_FILE="node_modules/@salesforce/types/lib/metadata.d.ts"
else
  INPUT_FILE="node_modules/@salesforce/types/src/metadata.ts"
fi

# Where we collect machine-readable reports for CI (artifact + issue automation)
REPORT_DIR=".typegen-reports"

# Marker files written into $REPORT_DIR for CI automation
# NOTE: This filename is consumed by the GitHub issue automation under:
#   TYPEGEN_REPORT_DIR/salesforce_types_version.txt
SALESFORCE_TYPES_VERSION_FILE="$REPORT_DIR/salesforce_types_version.txt"

err() { printf "ERROR: %s\n" "$*" >&2; }
warn() { printf "WARN: %s\n" "$*" >&2; }
info() { printf "%s\n" "$*"; }

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    err "Missing required command: $1"
    return 1
  fi
}

# --- Preconditions ------------------------------------------------------------

require_cmd cargo
require_cmd git
require_cmd node

if [[ ! -d "node_modules/@salesforce/types" ]]; then
  err "Missing npm dependency: node_modules/@salesforce/types"
  err ""
  err "Fix:"
  err "  npm install"
  err ""
  err "This repo uses package.json only so Dependabot can bump @salesforce/types."
  exit 1
fi

if [[ ! -f "$INPUT_FILE" ]]; then
  err "Missing TypeScript input file for @salesforce/types. Tried:"
  err "  - node_modules/@salesforce/types/src/metadata.ts"
  err "  - node_modules/@salesforce/types/lib/metadata.d.ts"
  err ""
  err "Fix:"
  err "  npm install"
  exit 1
fi

if [[ ! -d ".git" ]]; then
  warn "No .git directory found at repo root; diff checks may not work as expected."
fi

# Detect uncommitted changes up-front (not fatal, but helps avoid confusion)
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  if [[ -n "$(git status --porcelain=v1)" ]]; then
    warn "Working tree is not clean. Generation will modify files; review diff carefully after."
  fi
fi

# Ensure report directory exists and is clean for this run
rm -rf "$REPORT_DIR"
mkdir -p "$REPORT_DIR"

# Record installed @salesforce/types version (used by CI to create per-version rollup issues)
# Prefer node_modules package.json as the source of truth.
if [[ -f "node_modules/@salesforce/types/package.json" ]]; then
  node -e "const p=require('./node_modules/@salesforce/types/package.json'); process.stdout.write(p.version || 'unknown');" > "$SALESFORCE_TYPES_VERSION_FILE" || true
else
  echo "unknown" > "$SALESFORCE_TYPES_VERSION_FILE"
fi

# --- Run generation -----------------------------------------------------------

info "==> Generating Rust modules into: $OUTPUT_DIR"
info "==> Using @salesforce/types input: $INPUT_FILE"
info ""

GEN_LOG="$(mktemp -t busbar-sf-types-gen.XXXXXX.log)"
cleanup() { rm -f "$GEN_LOG"; }
trap cleanup EXIT

# Run generator; capture output to log for post-processing and actionable messages.
set +e
cargo run -p sf-typegen --bin generate_from_typescript -- \
  --modular \
  --output-dir "$OUTPUT_DIR" \
  --input-file "$INPUT_FILE" 2>&1 | tee "$GEN_LOG"
GEN_STATUS="${PIPESTATUS[0]}"
set -e

if [[ "$GEN_STATUS" -ne 0 ]]; then
  err ""
  err "Type generation failed (exit code $GEN_STATUS)."
  err "See log above; full log saved at: $GEN_LOG"
  exit "$GEN_STATUS"
fi

# --- Collect machine-readable reports for CI automation -----------------------
#
# Convention:
# - The generator may emit JSON reports in repo root (e.g., missing_overlays.json).
# - We copy any known report files into $REPORT_DIR so CI can:
#   - upload them as artifacts
#   - create/update a rollup GitHub issue based on their contents
#
# If a file doesn't exist, we skip it.
for f in \
  "missing_overlays.json" \
  "missing_categories.json" \
  "missing_uncategorized.json" \
; do
  if [[ -f "$f" ]]; then
    cp "$f" "$REPORT_DIR/$f"
  fi
done

# Always persist a short text log excerpt for debugging issue automation
cp "$GEN_LOG" "$REPORT_DIR/generate.log"

info ""
info "==> Running rustfmt"
cargo fmt --all

info ""
info "==> Running cargo check (workspace, all features)"
cargo check --workspace --all-features

# --- Post checks / hints ------------------------------------------------------

info ""
info "==> Generation complete."
info "==> Reports collected under: $REPORT_DIR"
info ""

# Try to surface likely follow-up work from generator output.
# The generator prints warnings; we grep for keywords commonly emitted by sf-typegen.
if grep -Eqi "missing overlay|overlays?|unmapped|uncategorized|no category|warning" "$GEN_LOG"; then
  warn "Generator reported warnings worth investigating. Highlights:"
  # Print a small set of relevant lines without dumping the whole log.
  grep -Eni "missing overlay|overlays?|unmapped|uncategorized|no category|warning" "$GEN_LOG" | head -n 50 >&2
  warn ""
  warn "Next steps:"
  warn "  - If new types are uncategorized: update sf-typegen categorization rules (categories.rs)."
  warn "  - If overlays are missing: update sf-typegen overlays.toml (or related overlay source)."
  warn "  - If schemars derives fail: update overlay docs/attributes for affected types."
fi

# Helpful summary of changed files (no fail here; CI can enforce cleanliness separately).
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  CHANGES="$(git status --porcelain=v1 | wc -l | tr -d ' ')"
  if [[ "$CHANGES" -gt 0 ]]; then
    info "==> Git status summary (you should commit generated changes):"
    git status --porcelain=v1
    info ""
    info "Tip: In CI, you can enforce regeneration is committed by failing if 'git diff --exit-code' fails."
  else
    info "==> No git changes detected."
  fi
fi

info "Done."
