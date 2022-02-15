import pathlib
import os
import sys
from typing import List
import shutil
import random
import string

c_files: List[pathlib.Path] = []
rs_files: List[pathlib.Path] = []


def randomword(length):
    letters = string.ascii_lowercase
    return "".join(random.choice(letters) for i in range(length))


def run():
    cwd = os.getcwd()
    print(f"current working directory: {cwd}")
    c_path = pathlib.Path("./c_src")
    # c_full_path = c_path.absolute()
    # exists = c_path.exists()

    rs_path = pathlib.Path("./src")
    # rs_full_path = rs_path.absolute()
    # exists = rs_path.exists()

    paths_to_check: List[pathlib.Path] = list(c_path.iterdir())
    while len(paths_to_check) > 0:
        f = paths_to_check.pop()
        if f.is_file():
            print(f"found file {f}")
            # if f.suffix == "c" or f.suffix == "h":
            if f.suffix in (".c", ".h"):
                print(f"adding {f} to list of files to copy")
                c_files.append(f)
            else:
                print(f"un-handled/un-wanted suffix {f.suffix}")
        elif f.is_dir():
            paths_to_check.extend(list(f.iterdir()))

    for c_file in c_files:
        new_file = c_file.stem
        if c_file.suffix == ".c":
            new_file += "_c.rs"
        if c_file.suffix == ".h":
            new_file += "_h.rs"

        tgt_path = rs_path.joinpath(new_file)
        print(f"testing if {tgt_path} exists")
        while tgt_path.exists() is True:
            print(f"{tgt_path} exists, generating pseudo-random name")
            base_path = rs_path
            f_name = tgt_path.stem
            sfx = tgt_path.suffix
            f_name = f"{f_name}_{randomword(6)}{sfx}"
            tgt_path = base_path.joinpath(f_name)
            print(f"trying new name {tgt_path}")
        print(f"target name {tgt_path}; copying file")
        shutil.copyfile(str(c_file), str(tgt_path))
        print("file copied")
    print("done")
    return 0


if __name__ == "__main__":
    sys.exit(run())
