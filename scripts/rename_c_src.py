import pathlib
import os
import sys
from typing import List

c_files: List[pathlib.Path] = []
rs_files: List[pathlib.Path] = []

import random, string

def randomword(length):
   letters = string.ascii_lowercase
   return ''.join(random.choice(letters) for i in range(length))


def run():
    cwd = os.getcwd()
    c_path = pathlib.Path("./c_src")
    c_full_path = c_path.absolute()
    exists = c_path.exists()

    rs_path = pathlib.Path("./src")
    rs_full_path = rs_path.absolute()
    exists = rs_path.exists()

    c_src_files = c_path.iterdir()
    for f in c_src_files:
        if f.is_file():
            if f.suffix == "c" or f.suffix == "h":
                c_files.append(f)

    for c_file in c_files:
        new_file = c_file.stem
        if c_file.suffix == "c":
            new_file += "_c.rs"
        if c_file.suffix == "h":
            new_file += "_h.rs"
        
        tgt_path = rs_path.joinpath(new_file)
        while tgt_path.exists() is False:
            base_path = rs_path
            f_name = tgt_path.stem
            sfx = tgt_path.suffix
            f_name = f"{f_name}_{randomword(6)}"
            tgt_path = 

    return 0


if __name__ == "__main__":
    sys.exit(run())
