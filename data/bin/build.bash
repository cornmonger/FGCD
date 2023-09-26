#!/bin/bash
##
#  FGCD: Strategy guide & data collection for fighting games
#  Copyright (C) 2023 else.club, a division of Asmov LLC
#
#  This program is free software: you can redistribute it and/or modify
#  it under the terms of the GNU General Public License as published by
#  the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#
#  This program is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU General Public License for more details.
# 
#  You should have received a copy of the GNU General Public License
#  along with this program.  If not, see <https://www.gnu.org/licenses/>.
##
set -aepu -o pipefail ; shopt -s extglob

SCRIPT_NAME="$(basename "$0")"
PROJECT_DIR="$(realpath $(dirname "$0")/..)"
MONO_PROJECT_DIR="$(realpath $(dirname "$0")/../..)"

LIBREOFFICE="$(which libreoffice)"

DATA_DIR="$PROJECT_DIR/data"
TARGET_DIR="$PROJECT_DIR/target"

GAME_DATA_DIR="$DATA_DIR/games"
DATA_TARGET_DIR="$TARGET_DIR/data"

GAME_DATA_TARGET_DIR="$DATA_TARGET_DIR/games"

MORTAL_KOMBAT_1_DATA_DIR="$GAME_DATA_DIR/mortal_kombat_1"
MORTAL_KOMBAT_1_DATA_TARGET_DIR="$GAME_DATA_TARGET_DIR/mortal_kombat_1"

compile() {
    $LIBREOFFICE --headless --convert-to ods "$MORTAL_KOMBAT_1_DATA_DIR/*.fods" --outdir "$MORTAL_KOMBAT_1_DATA_TARGET_DIR"
}

compile

