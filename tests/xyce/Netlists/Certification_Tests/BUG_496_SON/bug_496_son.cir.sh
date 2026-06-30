#!/bin/sh

# The input arguments to this script are:
# $1 = location of Xyce binary
# $2 = location of xyce_verify.pl script
# $3 = location of compare script 
# $4 = location of circuit file to test
# $5 = location of gold standard prn file

# This script checks only that Xyce runs without crashing.  No attempt is made
# to examine the output.  The bug was that Xyce segfaulted on the exact input
# file used here.
XYCE=$1
#XYCE_VERIFY=$2
#XYCE_COMPARE=$3
CIRFILE=$4
#GOLDPRN=$5

rm -f $CIRFILE.prn $CIRFILE.err
$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi
echo "Exit code = 0"
exit 0

