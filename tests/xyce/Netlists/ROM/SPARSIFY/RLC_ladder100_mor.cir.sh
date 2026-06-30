#!/bin/sh

# The input arguments to this script are set up in 
# Xyce_Test/TestScripts/tsc_run_test_suite and are as follows:
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
#XYCE_COMPARE=$3
#CIRFILE=$4
#GOLDPRN=$5

# comparison tolerances for ACComparator.pl
abstol=1e-6;
reltol=1e-4;  #0.01%
zerotol=1e-10;
freqreltol=1e-6;

CIR1=RLC_ladder100_mor.cir
CIR2=RLC_ladder100_mor_sparsify.cir
CIR3=RLC_ladder100.cir
CIR4=RLC_ladder_top30.cir

rm -f $CIR1.prn $CIR1.err $CIR2.prn $CIR2.err $CIR1.prn

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

$XYCE $CIR3 > $CIR3.out 2> $CIR3.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

$XYCE $CIR4 > $CIR4.out 2> $CIR4.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

$XYCE_VERIFY $CIR3 $CIR3.prn $CIR4.prn
if [ "$?" -ne "0" ]; then
  echo "Exit code = 2"
  exit 2
fi

# Changing comparator to AC comparator for frequency domain comparison.
XYCE_VERIFY=`echo $XYCE_VERIFY| sed -e 's/xyce_verify/ACComparator/'`
GSFORMAT='--gsformat=xycecsv'

$XYCE_VERIFY $GSFORMAT $CIR1.Orig.FD.prn  $CIR1.Red.FD.prn $abstol $reltol $zerotol $freqreltol
if [ "$?" -ne "0" ]; then
  echo "Exit code = 2"
  exit 2
fi

$XYCE_VERIFY $GSFORMAT $CIR2.Orig.FD.prn  $CIR2.Red.FD.prn $abstol $reltol $zerotol $freqreltol
if [ "$?" -ne "0" ]; then
  echo "Exit code = 2"
  exit 2
fi

$XYCE_VERIFY $GSFORMAT $CIR1.Red.FD.prn  $CIR2.Red.FD.prn $abstol $reltol $zerotol $freqreltol
if [ "$?" -ne "0" ]; then
  echo "Exit code = 2"
  exit 2
fi

echo "Exit code = 0"
exit 0

