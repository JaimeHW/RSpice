#!/bin/sh

# The input arguments to this script are:
# $1 = location of Xyce binary
# $2 = location of xyce_verify.pl script
# $3 = location of compare script 
# $4 = location of circuit file to test
# $5 = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10
# Otherwise we return the exit code of compare or xyce_verify.pl

XYCE=$1
CIRFILE=$4

rm -f output errors
$XYCE -error-test $CIRFILE > output 2>&1

perl count.pl
if [ "$?" == 2 ]; then
  echo "Exit code = 2"
  exit 2
fi
echo "Exit code = 0"
exit 0
