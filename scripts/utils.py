import subprocess as sp
from functools import lru_cache
from pathlib import Path

from rich.console import Console

console = Console()


def get_console() -> Console:
    return console


@lru_cache()
def get_project_root() -> Path:
    return Path(__file__).parent.parent


def run_command(command: str, cwd: Path | None = None):
    if cwd is None:
        cwd = get_project_root()

    console.print("[bright_black]$ {}".format(str(command)))
    console.print(
        "[bright_black]{}".format(str(cwd.relative_to(get_project_root()))),
        justify="right",
    )

    sp.run(command, shell=True, cwd=cwd, check=True)
