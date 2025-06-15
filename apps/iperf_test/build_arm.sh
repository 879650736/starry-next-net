#!/usr/bin/sh

export CC=aarch64-linux-musl-gcc
export CXX=aarch64-linux-musl-g++
export AR=aarch64-linux-musl-ar
export RANLIB=aarch64-linux-musl-ranlib
export STRIP=aarch64-linux-musl-strip

export PATH=$(HOME)/musl/aarch64-linux-musl-cross/bin:$PATH

export AARCH64_MUSL_CUSTOM_SYSROOT=$(HOME)/musl/aarch64-linux-musl-cross


cd $(HOME)/openssl-3.0.16

./Configure \
    no-shared \
    no-zlib \
    --prefix=${AARCH64_MUSL_CUSTOM_SYSROOT}/aarch64-linux-musl/ \
    --cross-compile-prefix=${AARCH64_MUSL_CUSTOM_SYSROOT}/bin/ \
    linux-aarch64

make -j$(nproc)
make install
