#!/bin/bash

make clean
mkdir -p $1
if [[ "$1" == *"riscv"* ]]; then
    if [[ "$1" == *"musl"* ]]; then
        ./configure  --disable-openssl --prefix=$1 --host=riscv64-linux-musl CC=riscv64-linux-musl-gcc  --enable-static-bin
    else
        ./configure  --disable-openssl --prefix=$1 --host=riscv64-linux-gnu CC=riscv64-linux-gnu-gcc  --enable-static-bin
    fi
else
    if [[ "$1" == *"musl"* ]]; then
        ./configure  --disable-openssl --prefix=$1 --host=loongarch64-linux-musl CC=loongarch64-linux-musl-gcc  --enable-static-bin
    else
        ./configure  --disable-openssl --prefix=$1 --host=loongarch64-linux-gnu CC=loongarch64-linux-gnu-gcc  --enable-static-bin
    fi
fi

make V=1 -j 
#cp src/.libs/iperf3 $1
#cp src/iperf3 $1
#cp src/.libs/libiperf.so $1
