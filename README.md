# Geman-McClure Robust Rotation Estimation -- Rust Implementation

## Setup

Tested in Ubuntu 22.04

```bash
# Rust
curl https://sh.rustup.rs -sSf | sh

# OpenBLAS (for ndarray-linalg)
sudo apt update
sudo apt install -y libopenblas-dev pkg-config libssl-dev cmake

# Python
sudo apt install -y python3-pip python3-dev

# (Optional) Setup a Python virtualenv
# python3 -m pip install virtualenv
# python3 -m virtualenv venv
# source venv/bin/activate

python3 -m pip install numpy "maturin[patchelf]"
```

## Build

```bash
# Develop within a virtualenv
cd geman-mcclure-rotation-solver-python
maturin develop

# Build
cd geman-mcclure-rotation-solver-python
maturin build --release
```
