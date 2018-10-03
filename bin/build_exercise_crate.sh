#!/bin/bash
# Compile the 'exercise' crate and put it in the 'bin/' folder

TRACK_ROOT="$(git rev-parse --show-toplevel)"

EXERCISE_CRATE_PATH="$TRACK_ROOT/exercise"

BIN_DIR_PATH="$TRACK_ROOT/bin"

echo $TRACK_ROOT

echo $EXERCISE_CRATE_PATH

echo $BIN_DIR_PATH

(
	cd "$EXERCISE_CRATE_PATH"

	echo "Building exercise crate"

	cargo build --release

	RELEASE_PATH="$EXERCISE_CRATE_PATH/target/release/exercise"

	if [ -f "$RELEASE_PATH/exercise.exe" ]; then
		RELEASE_PATH="$RELEASE_PATH.exe"
	fi

	echo "Copying exercise crate from $RELEASE_PATH into $BIN_DIR_PATH"

	cp "$RELEASE_PATH" "$BIN_DIR_PATH"
)
