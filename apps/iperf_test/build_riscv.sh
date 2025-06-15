#!/usr/bin/sh

export CC=riscv64-linux-musl-gcc
export CXX=riscv64-linux-musl-g++
export AR=riscv64-linux-musl-ar
export RANLIB=riscv64-linux-musl-ranlib
export STRIP=riscv64-linux-musl-strip

export PATH=$(HOME)/musl/riscv64-linux-musl-cross/bin:$PATH

export RISCV64_MUSL_CUSTOM_SYSROOT=$(HOME)/musl/riscv64-linux-musl-cross

cd $(HOME)/openssl-3.0.16

./Configure \
    no-shared \
    no-zlib \
    --prefix=${RISCV64_MUSL_CUSTOM_SYSROOT}/riscv64-linux-musl/ \
    --cross-compile-prefix=${RISCV64_MUSL_CUSTOM_SYSROOT}/bin/ \
    linux64-riscv64

make -j$(nproc)
make install
