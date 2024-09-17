import sys

import utils


def build_python_package():
    utils.run_command("{} -m pip install maturin patchelf".format(sys.executable))

    utils.run_command(
        "maturin build --release --interpreter {}".format(sys.executable),
        cwd=utils.get_project_root() / "fracgm-python",
    )


if __name__ == "__main__":
    build_python_package()
    utils.get_console().print("[bold green]Done.")
