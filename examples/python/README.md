# :snake: Python Example for FracGM Solvers

This example shows how to use FracGM solver in Python 3.

## :gear: Install the FracGM Python Wrapper Library

```sh
# Build the FracGM Python wrapper library
python3 -m pip install "maturin[patchelf]" numpy rich
python3 ../../scripts/build_fracgm_python_package.py

# Install the FracGM Python wrapper library
# (Notice: You should checkout the real path to the wheel file)
python3 -m pip install $(ls ../../target/wheels/*.whl) --force-reinstall
```

## :running: Run

```sh
python3 ./example_fracgm_py_project.py
```

This main function will read two point clouds from [`data`](../data) folder and
solve the rotation and the registration problems with FracGM.

```
[[ Example for FracGM-based rotation solver ]]

Ground Truth:
 [[-0.0318679   0.85233803  0.52201947]
 [-0.51833845 -0.46065527  0.7205012 ]
 [ 0.85458159 -0.2476219   0.45647969]]

FracGM:
 [[-0.03224006  0.85246754  0.52178508]
 [-0.52026712 -0.4600657   0.7194871 ]
 [ 0.85339481 -0.24827131  0.45834338]]

[[ Example for FracGM-based registration solver ]]

Ground Truth:
 [[-0.0318679   0.85233803  0.52201947  0.3       ]
 [-0.51833845 -0.46065527  0.7205012   0.2       ]
 [ 0.85458159 -0.2476219   0.45647969  0.6       ]
 [ 0.          0.          0.          1.        ]]

FracGM:
 [[-0.03200563  0.852238    0.52217434  0.29966727]
 [-0.52038008 -0.46025508  0.71928425  0.20008234]
 [ 0.85333476 -0.24870798  0.45821843  0.60047556]
 [ 0.          0.          0.          1.        ]]

[[ Done ]]
```
