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
PLATFORM_DATA_DIR="$PROJECT_DIR/platforms"

DATA_TARGET_DIR="$TARGET_DIR/data"

GAME_DATA_TARGET_DIR="$DATA_TARGET_DIR/games"
PLATFORM_DATA_TARGET_DIR="$DATA_TARGET_DIR/platforms"


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
# Compiles data files
compile() {
    compile_dir "$GAME_DATA_DIR" "$GAME_DATA_TARGET_DIR"
    compile_dir "$PLATFORM_DATA_DIR" "$PLATFORM_DATA_TARGET_DIR"
}

##
# Uses libreoffice to convert the flat-file FODS files into zipped ODS files.
compile_dir() {
    local data_dir target_dir
    data_dir="$1"
    target_dir="$2"
    
    dirs="$(ls -d $data_dir/*/)"
    for dir in "${dirs[@]}"; do
        local dir_name data_item_dir target_item_dir
    echo "$dir"
        dir_name="$(basename "$dir")"
    echo "dir: ${dir_name}"
        data_item_dir="$data_dir/$dir_name"
        target_item_dir="$target_dir/$dir_name"

        cd "$data_item_dir"
        mkdir -p "$target_item_dir"
        $LIBREOFFICE --convert-to ods *.fods --outdir "$target_item_dir"
    done
}

main() {
    clean
    setup 
    compile
}

main
