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
#XYCE_COMPARE=$3
#CIRFILE=$4
#GOLDPRN=$5

CIR1=continuation1_mos2.cir
CIR2=continuation2_mos2.cir
CIR3=continuation3_mos2.cir
CIR4=continuation4_mos2.cir

# first comparison
$XYCE $CIR1 > $CIR1.out 2> $CIR1.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi
if [ ! -f $CIR1.HOMOTOPY.prn ]; then
  echo "Exit code = 14"
  exit 14
fi


$XYCE $CIR2 > $CIR2.out 2> $CIR2.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi
if [ ! -f $CIR2.HOMOTOPY.prn ]; then
  echo "Exit code = 14"
  exit 14
fi


grep -v "Index" $CIR1.HOMOTOPY.prn | grep -v "End" > $CIR1.prn2
grep -v "Index" $CIR2.HOMOTOPY.prn | grep -v "End" > $CIR2.prn2

diff $CIR1.prn2 $CIR2.prn2 > $CIR1.prn.out 2> $CIR1.prn.err
if [ "$?" -ne "0" ]; then
  echo "Diff failed comparing $CIR1.prn2 to $CIR2.prn2 --- see $CIR1.prn.out for details"
  echo "Exit code = 2"
  exit 2
fi

# This test isn't really about the non-HOMOTOPY *prn files, but 
# compare them anyway.
$XYCE_VERIFY $CIR1 $CIR2.prn $CIR1.prn  



# 2nd comparison
$XYCE $CIR3 > $CIR3.out 2> $CIR3.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi
if [ ! -f $CIR3.HOMOTOPY.prn ]; then
  echo "Exit code = 14"
  exit 14
fi


$XYCE $CIR4 > $CIR4.out 2> $CIR4.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi
if [ ! -f $CIR4.HOMOTOPY.prn ]; then
  echo "Exit code = 14"
  exit 14
fi


grep -v "Index" $CIR3.HOMOTOPY.prn | grep -v "End" > $CIR3.prn2
grep -v "Index" $CIR4.HOMOTOPY.prn | grep -v "End" > $CIR4.prn2

diff $CIR3.prn2 $CIR4.prn2 > $CIR3.prn.out 2> $CIR3.prn.err
if [ "$?" -ne "0" ]; then
  echo "Diff failed comparing $CIR3.prn2 to $CIR4.prn2.  See $CIR3.prn.out for details."
  echo "Exit code = 2"
  exit 2
fi

# This test isn't really about the non-HOMOTOPY *prn files, but 
# compare them anyway.
$XYCE_VERIFY $CIR3 $CIR4.prn $CIR3.prn  


EXITCODE=$?
if [ $EXITCODE -eq  0 ]; then
    echo "Exit code = 0"
    exit 0
else
    echo "xyce_verify failed comparing $CIR4.prn to $CIR3.prn."
    echo "Exit code = 2"
    exit 2
fi;

