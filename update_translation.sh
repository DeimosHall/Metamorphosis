#!/usr/bin/env bash

BUILD_DIR="translation-build/"
if [ -d "$BUILD_DIR" ]; then
	rm -r translation-build
fi

meson translation-build
meson compile -C translation-build metamorphosis-pot
# meson compile -C translation-build metamorphosis-update-po

rm -r translation-build
