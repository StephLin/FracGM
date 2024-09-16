# :crab: Rust Example for FracGM Solvers

This example shows how to use FracGM solver in Rust.

## :gear: Build

```sh
cd examples/rust
cargo build --release --target-dir ./target
```

## :running: Run

```sh
cd target/release
./example-fracgm-rust-project
```

This main function will read two point clouds from [`data`](../data) folder and
solve the rotation and the registration problems with FracGM.

```
[[ Example for FracGM-based rotation solver ]]

Ground Truth:
[[-0.031867898097342584, 0.8523380280706003, 0.5220194660887449],
 [-0.518338448529327, -0.4606552722113689, 0.7205011956687373],
 [0.8545815875895455, -0.24762190147196284, 0.4564796863654963]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2

FracGM:
[[-0.03224005912227601, 0.8524675415726007, 0.5217850794656356],
 [-0.5202671165035346, -0.46006569758772003, 0.7194870960540032],
 [0.8533948125415871, -0.24827131221365828, 0.45834337505719236]], shape=[3, 3], strides=[1, 3], layout=Ff (0xa), const ndim=2

[[ Example for FracGM-based registration solver ]]

Ground Truth:
[[-0.031867898097342584, 0.8523380280706003, 0.5220194660887449, 0.3],
 [-0.518338448529327, -0.4606552722113689, 0.7205011956687373, 0.2],
 [0.8545815875895455, -0.24762190147196284, 0.4564796863654963, 0.6],
 [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2

FracGM:
[[-0.0320056252466121, 0.8522379965904823, 0.522174335945396, 0.29966727060666926],
 [-0.520380076104837, -0.46025507997663084, 0.7192842537890208, 0.20008234173711115],
 [0.853334762180527, -0.24870797840669806, 0.4582184251327701, 0.6004755586727758],
 [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2

[[ Done ]]
```
