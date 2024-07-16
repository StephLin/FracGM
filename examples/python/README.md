# Python Example for FracGM Solvers

This example shows how to use FracGM solver in Python 3.

## :gear: Install the FracGM Python Wrapper Library

```sh
# Build the FracGM Python wrapper library
python3 -m pip install maturin[patchelf]
python3 ../../scripts/build_fracgm_python_package.py

# Install the FracGM Python wrapper library
# (Notice: You should checkout the real path to the wheel file)
python3 -m pip install ../../target/wheels/fracgm-0.1.0-cp3xx-cp3xx-manylinux_x_xx_x86_64.whl
```

## :running: Run

```sh
python3 ./example_fracgm_py_project.py
```

This main function will read two point clouds from [`data`](../data) folder and
solve the rotation and the registration problems with FracGM.
