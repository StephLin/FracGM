# FracGM

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg?style=flat)](https://opensource.org/licenses/BSD-3-Clause)

Rust implementation of "FracGM: A Fast Fractional Programming Technique for
Geman-McClure Robust Estimator." This work is submitted to IEEE Robotics and
Automation Letters (RA-L).

This library is written in **Rust** and we support **C++** and **Python**
wrappers.

**Table of Contents**

- [FracGM](#fracgm)
  - [:gear: Setup](#gear-setup)
  - [:seedling: Example Usage](#seedling-example-usage)

## :gear: Setup

Tested in Ubuntu 22.04

```bash
git clone --recurse-submodules -j8 https://github.com/StephLin/FracGM.git
git submodule update --init

# Rust
curl https://sh.rustup.rs -sSf | sh

# OpenBLAS (for ndarray-linalg)
sudo apt update
sudo apt install -y libopenblas-dev pkg-config libssl-dev cmake

# C++ (for maximum clique inlier selection)
sudo apt install -y g++

# (Optional) Setup the C++ wrapper
sudo apt install -y g++
python3 -m pip install meson ninja

# (Optional) Setup the Python wrapper
# python3 -m pip install virtualenv
# python3 -m virtualenv venv
# source venv/bin/activate
sudo apt install -y python3-pip python3-dev
python3 -m pip install numpy "maturin[patchelf]"
```

## :seedling: Example Usage

- [Rust](examples/rust)
- [C++](examples/cpp)
- [Python](examples/python)
