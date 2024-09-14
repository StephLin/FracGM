# :croissant: C++ Example for FracGM Solvers

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

```
[[ Example for FracGM-based rotation solver ]]

Ground Truth:
-0.0318679   0.852338   0.522019
 -0.518338  -0.460655   0.720501
  0.854582  -0.247622    0.45648

FracGM:
-0.0322401   0.852468   0.521785
 -0.520267  -0.460066   0.719487
  0.853395  -0.248271   0.458343

[[ Example for FracGM-based registration solver ]]

Ground Truth:
-0.0318679   0.852338   0.522019        0.3
 -0.518338  -0.460655   0.720501        0.2
  0.854582  -0.247622    0.45648        0.6
         0          0          0          1

FracGM:
-0.0320056   0.852238   0.522174   0.299667
  -0.52038  -0.460255   0.719284   0.200082
  0.853335  -0.248708   0.458218   0.600476
         0          0          0          1

[[ Done ]]
```
