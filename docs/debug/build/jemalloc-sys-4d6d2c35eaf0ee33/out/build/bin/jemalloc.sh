#!/bin/sh

prefix=/home/runner/work/core/core/./docs/debug/build/jemalloc-sys-4d6d2c35eaf0ee33/out
exec_prefix=/home/runner/work/core/core/./docs/debug/build/jemalloc-sys-4d6d2c35eaf0ee33/out
libdir=${exec_prefix}/lib

LD_PRELOAD=${libdir}/libjemalloc.so.2
export LD_PRELOAD
exec "$@"
