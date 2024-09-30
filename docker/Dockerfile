FROM ubuntu:22.04

# avoid selecting 'Geographic area' during installation
ARG DEBIAN_FRONTEND=noninteractive

# apt install required packages
RUN apt-get update \
    && apt-get install -y curl libopenblas-dev pkg-config libssl-dev cmake \
    git python3-pip python3-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install required packages from requirements.txt
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN . "$HOME/.cargo/env"

RUN python3 -m pip install --upgrade pip
RUN python3 -m pip install numpy "maturin[patchelf]" rich

# Build dependencies for FracGM
RUN git clone --recurse-submodules -j8 https://github.com/StephLin/FracGM.git
WORKDIR /FracGM/fracgm-python
RUN /usr/bin/python3 -m pip install maturin patchelf
RUN . "$HOME/.cargo/env" && maturin build --release --interpreter /usr/bin/python3
WORKDIR /FracGM
RUN python3 -m pip install $(ls ./target/wheels/*.whl) --force-reinstall

WORKDIR /
CMD ["/bin/bash"]
