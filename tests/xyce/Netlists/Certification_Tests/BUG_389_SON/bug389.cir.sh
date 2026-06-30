#!/bin/sh

# The input arguments to this script are:
# $1 = location of Xyce binary
# $2 = location of xyce_verify.pl script
# $3 = location of compare script
# $4 = location of circuit file to test
# $5 = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.
XYCE=$1
XYCE_VERIFY=$2

# comparison tolerances for file_compare.pl
abstol=1e-6;
reltol=1e-4;  #0.01%
zerotol=1e-6;

CIR1=global_param.cir
CIR2=local_param.cir

$XYCE $CIR1 > $CIR1.out 2> $CIR1.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

$XYCE $CIR2 > $CIR2.out 2> $CIR2.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

grep -v "TITLE" $CIR1.HB.FD.dat | grep -v "DATASETAUXDATA" | grep -v "ZONE" > $CIR1.prn2
grep -v "TITLE" $CIR2.HB.FD.dat | grep -v "DATASETAUXDATA" | grep -v "ZONE" > $CIR2.prn2

grep -v "TITLE" $CIR1.HB.TD.dat | grep -v "DATASETAUXDATA" | grep -v "ZONE" > $CIR1.prn3
grep -v "TITLE" $CIR2.HB.TD.dat | grep -v "DATASETAUXDATA" | grep -v "ZONE" > $CIR2.prn3

diff $CIR1.prn2 $CIR2.prn2 > $CIR2.prn.out 2> $CIR2.prn.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 2"
  exit 2
fi

# Use file_compare.pl for time domain comparison, since the .prn3 files
# aren't really in .prn format.
XYCE_VERIFY=`echo $XYCE_VERIFY| sed -e 's/xyce_verify/file_compare/'`

$XYCE_VERIFY $CIR1.prn3 $CIR2.prn3 $abstol $reltol $zerotol > $CIR2.prn.out 2> $CIR2.prn.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 2"
  exit 2
fi

EXITCODE=$?
if [ $EXITCODE -eq  0 ]; then
    echo "Exit code = 0"
    exit 0
else
    echo "Exit code = 2"
    exit 2
fi;

