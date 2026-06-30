#!/bin/sh
BASE_NAME="$1"
XYCE_BASE="$2"
VERILOG_BASE="$3"
XYCE_SRC_BASE="$4"

g++ -c -fpic -I$VERILOG_BASE/include/iverilog \
-I$XYCE_SRC_BASE/utils/XyceCInterface \
-I$XYCE_BASE/include \
$BASE_NAME.c

icpc -I$VERILOG_BASE/include/iverilog/libvpi.a \
-shared -L$XYCE_BASE/lib \
-L$XYCE_BASE/utils/XyceCInterface/.libs -lxycecinterface \
-Wl,-rpath=$XYCE_BASE/lib \
-Wl,-rpath=$XYCE_BASE/utils/XyceCInterface/.libs \
$MKLROOT/lib/intel64/libmkl_rt.so \
-o $BASE_NAME.vpi $BASE_NAME.o

iverilog -o$BASE_NAME.vvp $BASE_NAME.v


