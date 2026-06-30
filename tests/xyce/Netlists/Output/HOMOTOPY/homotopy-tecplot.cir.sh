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

CIR1=homotopy-tecplot.cir
CIR2=homotopy-prn.cir

$XYCE $CIR1 > $CIR1.out 2> $CIR1.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi


#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
# We only check after the first one, because it is the focus of the test
echo $XYCE_VERIFY | grep 'valgrind_check' > /dev/null
if [ $? -eq 0 ]
then
    echo "DOING VALGRIND RUN INSTEAD OF REAL RUN!" >&2
    `$XYCE_VERIFY $CIR1 $CIR1.prn $CIR1.prn > $CIR1.prn.out 2>&1 $CIR1.prn.err`
    if [ $? -ne 0 ]
    then
        echo "Exit code = 2"
        exit 2
    else
        echo "Exit code = 0"
        exit 0
    fi
fi

$XYCE $CIR2 > $CIR2.out 2> $CIR2.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

# test that the prn output and the tecplot output are identical except for the headers:
grep -v "Index" $CIR1.HOMOTOPY.dat | grep -v "End" | grep -v "TIME" | grep -v "I(VB)" | grep -v "I(VIC)" | grep -v "ZONE" | grep -v "TITLE" | grep -v "VARIABLES" > $CIR1.prn2
grep -v "GSTEP" $CIR2.HOMOTOPY.prn | grep -v "End" > $CIR2.prn2
#grep -v "GSTEP" $CIR2.HOMOTOPY.prn > $CIR2.prn2
#grep -v "End" $CIR2.HOMOTOPY.prn > $CIR2.prn2

TEST0=$(diff -w $CIR1.prn2 $CIR2.prn2 | wc -l)
if [ "$TEST0" -ne "0" ]; then
  echo "Diff between prn output and tecplot output failed!" >&2
  echo "Exit code = 2"
  exit 2
else
  echo "Diff between prn output ($CIR2.prn2) and tecplot output ($CIR1.prn2) passed!"
  echo "test result = $TEST0"
fi

# grep for all the features that should be present in the tecplot header:
TEST1=$(grep -ic "VARIABLES = " $CIR1.HOMOTOPY.dat)
TEST2=$(grep -ic "\" GSTEPPING\"" $CIR1.HOMOTOPY.dat)
TEST3=$(grep -ic "\" TIME\"" $CIR1.HOMOTOPY.dat)
TEST4=$(grep -ic "\" I(VB)\"" $CIR1.HOMOTOPY.dat)
TEST5=$(grep -ic "\" I(VIC)\"" $CIR1.HOMOTOPY.dat)
TEST6=$(grep -ic "DATASETAUXDATA TIME=" $CIR1.HOMOTOPY.dat)
TEST7=$(grep -ic "ZONE F=POINT T=\"Xyce data\"" $CIR1.HOMOTOPY.dat)
TEST8=$(grep -ic "End of Xyce(TM) Homotopy Simulation" $CIR1.HOMOTOPY.dat)

EXITCODE=0

if [ "$TEST1" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of \"VARIABLES =\" in tecplot header, got $TEST1" >&2
fi

if [ "$TEST2" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of GSTEPPING variable in tecplot header, got $TEST2" >&2
fi

if [ "$TEST3" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of TIME variable declarations in tecplot header, got $TEST3" >&2
fi
if [ "$TEST4" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of I(VB) variable declarations in tecplot header, got $TEST4" >&2
fi
if [ "$TEST5" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of I(VIC) variable declarations in tecplot header, got $TEST5" >&2
fi
if [ "$TEST6" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of \"DATASETAUXDATA TIME=\" in tecplot header, got $TEST6" >&2
fi
if [ "$TEST7" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of \"ZONE F=POINT T=\"Xyce data\"\" in tecplot header, got $TEST7" >&2
fi
if [ "$TEST8" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of \"End of Xyce(TM) Homotopy Simulation\" in tecplot footer, got $TEST8" >&2
fi

echo "Exit code = $EXITCODE"
exit $EXITCODE

