#!/bin/zsh
# One-command build of the burp mod and the client it runs in: run the mod
# tests, apply the mixins to the pinned Horizon source, build the client
# (fast profile) and install it next to the stock one as
# horizon-client-burp. Installing writes a fresh file (temp + mv), so a
# running stock client or other variants are never touched.
#
# Usage: tools/build.sh [--skip-tests]
set -e
cd "$(dirname "$0")/.."

RUST_BIN=/home/hakergeniusz/.local/share/horizon-launcher/rust/1.99.1-x86_64-unknown-linux-gnu/bin
CARGO_HOME=/home/hakergeniusz/.local/share/horizon-launcher/cargo
TARGET_DIR=/home/hakergeniusz/.cache/horizon-burp-build
TMPDIR=/home/hakergeniusz/.cache/tmp-build
WORKSPACE=/home/hakergeniusz/.local/share/horizon-launcher/native/source/af55a734865da4f1b11836164a979ff06a1f0449
MIXINS=/home/hakergeniusz/.local/share/horizon-launcher/native/tools/3f99a5c60dfa/bin/horizon-mixins
STAGE=/home/hakergeniusz/.cache/burp-stage
INSTALL=/home/hakergeniusz/Downloads/horizon-linux-x86_64/horizon-client-burp

export PATH="$RUST_BIN:$PATH" CARGO_HOME TMPDIR CARGO_TARGET_DIR="$TARGET_DIR"

if pgrep -f horizon-client-burp >/dev/null; then
  echo "build: horizon-client-burp is running — quit it first (the binary is swapped in place)"
  exit 1
fi

if [ "$1" != "--skip-tests" ]; then
  echo "== mod tests"
  [ -f Cargo.lock ] || cargo generate-lockfile
  cargo test --locked --lib
fi

echo "== apply mixins"
rm -rf "$STAGE"
OUT=$("$MIXINS" apply --workspace "$WORKSPACE" --mods "$PWD/mixins" --out "$STAGE")
echo "$OUT"
echo "$OUT" | grep -q 'stage_changed' || { echo "build: mixins apply reported no stage change"; exit 1; }

echo "== build client (fast profile)"
(cd "$STAGE" && cargo build --profile fast --locked -p horizon-client)

echo "== install"
T="$INSTALL.new.$$"
cp "$TARGET_DIR/fast/horizon-client" "$T"
mv "$T" "$INSTALL"
echo "build: installed $INSTALL"
