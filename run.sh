#!/bin/sh

# Build multitime.
git clone https://github.com/ltratt/multitime.git
cd multitime
 make -f Makefile.bootstrap
./configure
make
cd -

# Build the IronOx backend.
git clone https://github.com/gabi-250/rust.git
cd rust
git checkout ironox-backend-v0.1
./build_ironox.sh
cd -

# Build the Cranelift backend.
git clone https://github.com/bjorn3/rustc_codegen_cranelift.git
cd rustc_codegen_cranelift
git checkout a78029945aaf653225a9cb961c09e486529b9e6a
./prepare.sh
./test.sh
cd -

# Run the benchmarks.
python3.5 run_benchmarks.py --multitime ./multitime/multitime --clif ./rustc_codegen_cranelift/ --num-obs 30 --benchmarks ./benchmarks/ --template benchmark.tex --rust ./rust
