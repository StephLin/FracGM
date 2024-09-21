# Copyright 2024 the FracGM authors. All rights reserved.
# Use of this source code is governed by a BSD-style
# license that can be found in the LICENSE file.

import subprocess as sp
import typing as T
from functools import lru_cache
from pathlib import Path

try:
    from rich.console import Console

    console = Console()
except ImportError:
    import re

    class Console:
        def __init__(self):
            print("Notice: Rich is not installed. Fallback to plain console.")

        def print(self, *args, **kwargs):
            if "justify" in kwargs and kwargs["justify"] == "right":
                kwargs.pop("justify")
                args = tuple([" " * 80] + list(args))

            args = tuple(re.sub(r"\[[A-Za-z_ ]+\]", "", str(arg)) for arg in args)

            print(*args, **kwargs)

    console = Console()


def get_console() -> Console:
    return console


@lru_cache()
def get_project_root() -> Path:
    return Path(__file__).parent.parent


def run_command(command: str, cwd: T.Optional[Path] = None):
    if cwd is None:
        cwd = get_project_root()

    console.print("[bright_black]$ {}".format(str(command)))
    console.print(
        "[bright_black]{}".format(str(cwd.relative_to(get_project_root()))),
        justify="right",
    )

    sp.run(command, shell=True, cwd=cwd, check=True)
