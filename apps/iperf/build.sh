#!/bin/bash

make clean
mkdir -p $1
if [[ "$1" == *"riscv"* ]]; then
    if [[ "$1" == *"musl"* ]]; then
        ./configure  --disable-openssl --disable-cpu-affinity  --disable-daemon\
        --disable-have-dont-fragment  --disable-have-flowlabel  --disable-sendfile \
        --disable-have-so-bindtodevice --disable-have-so-max-pacing-rate \
        --disable-have-tcp-congestion --disable-have-tcp-user-timeout \
        --disable-have-tcp-snd-wnd \
         --prefix=$1 --host=riscv64-linux-musl CC=riscv64-linux-musl-gcc  --enable-static-bin
    else
        ./configure  --disable-openssl --disable-cpu-affinity  --disable-daemon \
        --disable-have-dont-fragment  --disable-have-flowlabel  --disable-sendfile \
        --disable-have-so-bindtodevice --disable-have-so-max-pacing-rate \
        --disable-have-tcp-congestion --disable-have-tcp-user-timeout \
        --disable-have-tcp-snd-wnd \
        --prefix=$1 --host=riscv64-linux-gnu CC=riscv64-linux-gnu-gcc  --enable-static-bin
    fi
else
    if [[ "$1" == *"musl"* ]]; then
        ./configure  --disable-openssl --disable-cpu-affinity  --disable-daemon \
        --disable-have-dont-fragment  --disable-have-flowlabel  --disable-sendfile \
        --disable-have-so-bindtodevice --disable-have-so-max-pacing-rate \
        --disable-have-tcp-congestion --disable-have-tcp-user-timeout \
        --disable-have-tcp-snd-wnd\
        --prefix=$1 --host=loongarch64-linux-musl CC=loongarch64-linux-musl-gcc  --enable-static-bin
    else
        ./configure  --disable-openssl --disable-cpu-affinity  --disable-daemon \
        --disable-have-dont-fragment  --disable-have-flowlabel --disable-sendfile \
        --disable-have-so-bindtodevice --disable-have-so-max-pacing-rate \
        --disable-have-tcp-congestion --disable-have-tcp-user-timeout \
        --disable-have-tcp-snd-wnd \
        --prefix=$1 --host=loongarch64-linux-gnu CC=loongarch64-linux-gnu-gcc  --enable-static-bin
    fi
fi

make V=1 -j 
#cp src/.libs/iperf3 $1
#cp src/iperf3 $1
#cp src/.libs/libiperf.so $1
