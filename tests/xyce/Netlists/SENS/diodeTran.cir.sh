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

CIR1=diodeTran.cir
CIR2=diodeTranFD.cir
CIR3=dummyDiodeTran.cir

# first comparison
$XYCE $CIR1 > $CIR1.out 2> $CIR1.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi
if [ ! -f $CIR1.SENS.prn ]; then
  echo "Exit code = 14"
  exit 14
fi


$XYCE $CIR2 > $CIR2.out 2> $CIR2.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi
if [ ! -f $CIR2.prn ]; then
  echo "Exit code = 14"
  exit 14
fi

# modify the headers so that xyce verify won't be confused by the different variable names.
sed '1 s/^.*$/Index  TIME   V(2)   I(V1)  I(V1)_DZR:VJ  I(V1)_DZR:CJO  I(V1)_DZR:EG  I(V1)_DZR:XTI  I(V1)_DZR:M  I(V1)_DZR:IS  I(V1)_DZR:RS      I(V1)_DZR:N/g'  $CIR1.SENS.prn > $CIR1.SENS.prn2

sed '1 s/^.*$/Index  TIME   V(2)   I(V1)  I(V1)_DZR:VJ  I(V1)_DZR:CJO  I(V1)_DZR:EG  I(V1)_DZR:XTI  I(V1)_DZR:M  I(V1)_DZR:IS  I(V1)_DZR:RS      I(V1)_DZR:N/g'  $CIR2.prn > $CIR2.prn2

$XYCE_VERIFY $CIR3 $CIR1.SENS.prn2 $CIR2.prn2  

EXITCODE=$?
if [ $EXITCODE -eq  0 ]; then
    echo "Exit code = 0"
    exit 0
else
    echo "Exit code = 2"
    exit 2
fi;

