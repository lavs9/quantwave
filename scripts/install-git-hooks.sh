#!/usr/bin/env bash
# Install repo git hooks (pre-push quality gate).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOOKS_DIR="$ROOT/.git/hooks"
SOURCE="$ROOT/scripts/git-hooks/pre-push"

if [[ ! -d "$ROOT/.git" ]]; then
  echo "error: not a git repository ($ROOT)"
  exit 1
fi

mkdir -p "$HOOKS_DIR"
chmod +x "$SOURCE"
ln -sf "../../scripts/git-hooks/pre-push" "$HOOKS_DIR/pre-push"

echo "Installed pre-push hook → $HOOKS_DIR/pre-push"
echo "Runs ./scripts/quantwave_verify.sh before each push."
echo "Bypass once: SKIP_PRE_PUSH_VERIFY=1 git push"
echo "Force rebuild: VERIFY_NO_CACHE=1 git push"
echo "Cache status:  python3 scripts/verify_cache.py status"