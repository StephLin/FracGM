# C++ Example for FracGM Solvers

This example shows how to use FracGM solver in C++.

You should prepare [Meson](https://mesonbuild.com/) build tool and
[Eigen](https://eigen.tuxfamily.org/) C++ linear algebra library to compile this
example.

## :gear: Build

```sh
# Build the FracGM C++ wrapper library
python3 ../../scripts/build_fracgm_cxx_library.py

# Build the main program
meson setup builddir
cd builddir
meson compile
```

## :running: Run

```sh
./example-fracgm-cpp-project
```

This main function will read two point clouds from [`data`](../data) folder and
solve the rotation and the registration problems with FracGM.
