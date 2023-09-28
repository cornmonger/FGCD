#!/bin/env python3

import os;
import shutil;
import subprocess;

PROJECT_DIR = os.path.realpath(os.path.dirname(__file__) + '/..')

TARGET_DIR = PROJECT_DIR + '/target'
GAME_DATA_DIR = PROJECT_DIR + '/games'
PLATFORM_DATA_DIR = PROJECT_DIR + '/platforms'

DATA_TARGET_DIR = TARGET_DIR + '/data'

GAME_DATA_TARGET_DIR = DATA_TARGET_DIR + '/games'
PLATFORM_DATA_TARGET_DIR = DATA_TARGET_DIR + '/platforms'

LIBREOFFICE = shutil.which('libreoffice')

def clean():
    if (os.path.exists(DATA_TARGET_DIR)):
        shutil.rmtree(DATA_TARGET_DIR)

def setup():
    os.makedirs(DATA_TARGET_DIR, 0o755, True)

def compile():
    compile_dir(GAME_DATA_DIR, GAME_DATA_TARGET_DIR)
    compile_dir(PLATFORM_DATA_DIR, PLATFORM_DATA_TARGET_DIR)

def compile_dir(data_dir, target_dir):
    for name in os.listdir(data_dir):
        item_data_dir = os.path.join(data_dir, name)
        item_target_dir = os.path.join(target_dir, name)
        os.makedirs(item_target_dir, 0o755)
        os.chdir(item_data_dir)
        print(item_data_dir)
        result = subprocess.run([LIBREOFFICE, '--convert-to', 'ods', '*.fods', '--outdir', item_target_dir], shell=True)
        print(result)


def main():
    clean()
    setup()
    compile()
    
if __name__ == "__main__":
    main()
