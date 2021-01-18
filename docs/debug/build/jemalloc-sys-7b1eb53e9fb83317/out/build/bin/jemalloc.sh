#!/bin/sh

prefix=/home/runner/work/core/core/./docs/debug/build/jemalloc-sys-7b1eb53e9fb83317/out
exec_prefix=/home/runner/work/core/core/./docs/debug/build/jemalloc-sys-7b1eb53e9fb83317/out
libdir=${exec_prefix}/lib

LD_PRELOAD=${libdir}/libjemalloc.so.2
export LD_PRELOAD
exec "$@"
