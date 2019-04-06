#!/bin/sh
set -e # Exit immediately when any command fails

# Install Rust when necessary.
rustup -V >/dev/null || curl https://sh.rustup.rs -sSf | sh
. $HOME/.cargo/env >/dev/null

# Build multitime.
git clone https://github.com/ltratt/multitime.git || true
cd multitime
 make -f Makefile.bootstrap
 ./configure
 make
cd -

# Build the Cranelift backend.

# Needed by cranelift:
git config --global user.email >/dev/null || git config --global user.email "benchmark-runner@benchmarker.com"
git config --global user.name >/dev/null || git config --global user.name "Benchmark Runner"

git clone https://github.com/bjorn3/rustc_codegen_cranelift.git || true
cd rustc_codegen_cranelift
rustup override set nightly-2019-03-27

git checkout 1fc1fbef93456ac2444479073b36e7d4a9f44b58

./prepare.sh
./test.sh --release
cd -
# Build the IronOx backend.
git clone https://github.com/gabi-250/rust.git || true
cd rust
git checkout ironox-backend-v0.1
sh ./build_ironox.sh
cd -

python3 run_benchmarks.py --multitime ./multitime/multitime --rust ./rust --clif ./rustc_codegen_cranelift --template template.tex --benchmarks ./benchmarks
