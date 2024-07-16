# C++ Example for FracGM Solvers

This example shows how to use FracGM solver in C++.

You should prepare [Meson](https://mesonbuild.com/) build tool and
[Eigen](https://eigen.tuxfamily.org/) C++ linear algebra library to compile this
example.

## :gear: Build

```sh
cargo build --release --target-dir ./target
```

## :running: Run

```sh
cd target/release
./example-fracgm-rust-project
```

This main function will read two point clouds from [`data`](../data) folder and
solve the rotation and the registration problems with FracGM.
