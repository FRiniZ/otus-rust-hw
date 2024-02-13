#!/bin/bash

cargo build || exit 1
cp ./target/debug/libsmartsocket.so ./c_src/

make -C c_src || exit 1

cd c_src && ./a.out && cd ..

