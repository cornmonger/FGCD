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

TARGET_DIR="$PROJECT_DIR/target"

GAME_DATA_DIR="$PROJECT_DIR/games"
DATA_TARGET_DIR="$TARGET_DIR/data"

GAME_DATA_TARGET_DIR="$DATA_TARGET_DIR/games"

##
# Cleans the data target dir
clean() {
    rm -rf "$DATA_TARGET_DIR"
}

##
# Creates the data target dir structure
setup() {
    mkdir -p "$DATA_TARGET_DIR"
}

##
# Uses libreoffice to convert the flat-file FODS files into zipped ODS files.
compile() {
    cd "$GAME_DATA_DIR"

    for dir in $(ls -d $GAME_DATA_DIR/*/); do
        local game_dir_name game_data_dir game_data_target_dir
        game_dir_name="$(basename "$dir")"
        game_data_dir="$GAME_DATA_DIR/$game_dir_name"
        game_data_target_dir="$GAME_DATA_TARGET_DIR/$game_dir_name"

        cd "$game_data_dir"
        mkdir -p "$game_data_target_dir"
        $LIBREOFFICE --convert-to ods *.fods --outdir "$game_data_target_dir"
    done
}

main() {
    clean
    setup 
    compile
}

main
