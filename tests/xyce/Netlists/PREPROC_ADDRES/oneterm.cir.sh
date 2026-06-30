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

CIR1=oneterm.cir
CIR2=oneterm.cir_xyce.cir
CIR2TEMP=$CIR2.temp
GOLDSCRIPT=./oneterm.cir_xyce.cir.gs.pl
REMOVESLASHR=./oneterm.cir.pl

rm -f $CIR1.prn $CIR1.err $GOLDSCRIPT.err $CIR1.out
rm -f $CIR2TEMP.prn $CIR2TEMP.err $CIR2TEMP.prn.gs $CIR2TEMP.out
rm -f $REMOVESLASHR.err

$XYCE $CIR1 > $CIR1.out 2> $CIR1.err
if [ "$?" -ne "0" ]; then
  echo "Xyce exited on $CIR1!"
  echo "Exit code = 10"
  exit 10
fi
# Now we have $CIR2

#Have to add this stuff in case there are random \r characters in the file
#oneterm.cir_xyce.cir
$REMOVESLASHR > /dev/null 2> $REMOVESLASHR.err
if [ "$?" -ne "0" ]; then
    echo "Script $REMOVESLASHR failed!"
    echo "Exit code = 10"
    exit 10
fi
# Now we've fixed up $CIR2 into $CIR2TEMP

rm -f $CIR2

$XYCE $CIR2TEMP > $CIR2TEMP.out 2> $CIR2TEMP.err
if [ "$?" -ne "0" ]; then
    echo "Xyce exited on $CIR2TEMP!"
  echo "Exit code = 10"
  exit 10
fi

$GOLDSCRIPT $CIR2TEMP.prn > /dev/null 2> $GOLDSCRIPT.err
if [ "$?" -ne "0" ]; then
  echo "Script $GOLDSCRIPT failed!"
  echo "Exit code = 10"
  exit 10
fi

$XYCE_VERIFY $CIR2TEMP $CIR2TEMP.prn.gs $CIR2TEMP.prn

EXITCODE=$?
if [ $EXITCODE -eq  0 ]; then
    echo "Exit code = 0"
    exit 0
else
    echo "Xyce Verify failed!"
    echo "Exit code = 2"
    exit 2
fi;

