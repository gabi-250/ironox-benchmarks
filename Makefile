TOOLCHAIN := ironox
ifeq ($(RUST_BACKEND),ironox)
	IRONOX_SO := librustc_codegen_ironox-ironox.so
	IRONOX_BUILD_DIR := $(RUST_ROOT)/build
	IRONOX_DIR := $(realpath $(IRONOX_BUILD_DIR))
	BACKEND := $(IRONOX_DIR)/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/$(IRONOX_SO)
	CUSTOM_RUSTC := RUSTC=$(RUST_ROOT)/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
	LIBS := $(RUST_ROOT)/build/x86_64-unknown-linux-gnu/stage1/lib
	CARGO := $(RUST_ROOT)/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
else ifeq ($(RUST_BACKEND),cranelift)
	CLIF_SO := librustc_codegen_cranelift.so
	BACKEND := $(realpath $(CLIF_DIR)/target/release/$(CLIF_SO))
	CLIF_FLAGS :=  --sysroot $(realpath $(CLIF_DIR)/build_sysroot/sysroot)
	CARGO_FLAGS := +nightly-2019-03-27
	CARGO := cargo
else ifeq ($(RUST_BACKEND),llvm)
	LLVM_SO := librustc_codegen_llvm-llvm.so
	LLVM_BUILD_DIR := $(RUST_ROOT)/llvm_build
	LLVM_DIR := $(realpath $(LLVM_BUILD_DIR))
	BACKEND := $(LLVM_DIR)/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/$(LLVM_SO)
	CUSTOM_RUSTC := RUSTC=$(LLVM_DIR)/x86_64-unknown-linux-gnu/stage1/bin/rustc
	LIBS := $(LLVM_DIR)/x86_64-unknown-linux-gnu/stage1/lib
	CARGO := $(LLVM_DIR)/x86_64-unknown-linux-gnu/stage0/bin/cargo
endif

STDLIB_SO := $(RUST_ROOT)/llvm_build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/libstd.so
PROJ_DIR := ./benchmarks/$(proj)
TEST_NAME := test_$(proj)
CODEGEN_FLAGS := -Cdebuginfo=0 -Cno-integrated-as -Cpanic=abort
NIGHTLY_FLAGS := -Zcodegen-backend=$(BACKEND) -Zno-landing-pads
RUSTFLAGS := $(NIGHTLY_FLAGS) $(CODEGEN_FLAGS) $(CLIF_FLAGS)
CARGO_BUILD_DIR := $(PROJ_DIR)/target/debug/deps
BUILD_DIR := build
RUST_BUILD_DIR := /target/debug
MANIFEST_PATH := --manifest-path $(PROJ_DIR)/Cargo.toml

.PHONY: bench_build clean

bench_build:
	$(MULTITIME) -n $(num_obs) sh -c "${CARGO} clean ${MANIFEST_PATH} && LD_LIBRARY_PATH=${LIBS} RUSTFLAGS=\"${RUSTFLAGS}\" ${CUSTOM_RUSTC} ${CARGO} ${CARGO_FLAGS} build ${MANIFEST_PATH} > /dev/null 2>&1"

bench_run: clean build
	$(MULTITIME) -n $(num_obs) sh -c "LD_LIBRARY_PATH=${LIBS} RUSTFLAGS=\"${RUSTFLAGS}\" ${CUSTOM_RUSTC} ${CARGO} ${CARGO_FLAGS} run ${MANIFEST_PATH} > /dev/null 2>&1"

build:
	LD_LIBRARY_PATH=${LIBS} RUSTFLAGS="${RUSTFLAGS}" ${CUSTOM_RUSTC} ${CARGO} ${CARGO_FLAGS} build ${MANIFEST_PATH} > /dev/null 2>&1

clean:
	$(CARGO) clean $(MANIFEST_PATH)
