#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <environment>"
    exit 1
fi

ENV=$1

APP_ENV=${ENV} cargo build --release

SRC="/home/similar/abjad-sled/target/release/abjad"
DEST="/home/similar/${ENV}/abjad-${ENV}"

if [ "$SRC" != "$DEST" ]; then
    rm -Rf "$DEST"
    mv "$SRC" "$DEST"
else
    echo "Source and destination are the same. Skipping move operation."
fi
cp -r config.${ENV}.yml /home/similar/${ENV}/
