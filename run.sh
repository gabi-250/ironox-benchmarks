#!/bin/sh

# Install Rust.
curl https://sh.rustup.rs -sSf | sh
. $HOME/.cargo/env

# Build multitime.
git clone https://github.com/ltratt/multitime.git
cd multitime
 make -f Makefile.bootstrap
./configure
make
cd -

# Build the Cranelift backend.

# Needed by cranelift:
git config --global user.email || git config --global user.email "benchmark-runner@benchmarker.com"
git config --global user.name || git config --global user.name "Benchmark Runner"

git clone https://github.com/bjorn3/rustc_codegen_cranelift.git
cd rustc_codegen_cranelift
rustup override set nightly-2019-03-27

git checkout 1fc1fbef93456ac2444479073b36e7d4a9f44b58

./prepare.sh
./test.sh
cd -

# Build the IronOx backend.
git clone https://github.com/gabi-250/rust.git
cd rust
git checkout ironox-backend-v0.1
sh ./build_ironox.sh
cd -


python3.5 run_benchmarks.py --multitime ./multitime/multitime --rust ./rust --template template.tex --benchmarks ./benchmarks

