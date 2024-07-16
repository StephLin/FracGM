# Rust Example for FracGM Solvers

This example shows how to use FracGM solver in Rust.

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
