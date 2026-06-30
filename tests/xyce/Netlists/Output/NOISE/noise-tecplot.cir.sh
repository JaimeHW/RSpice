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

CIR1=noise-tecplot.cir
CIR2=noise-prn.cir

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
grep -v "DATASET" $CIR1.NOISE.dat | grep -v "End" | grep -v "Re(V(4))" | grep -v "Im(V(4))" | grep -v "VR(4)" | grep -v "VI(4)" | grep -v "INOISE" | grep -v "ONOISE" | grep -v "ZONE" | grep -v "TITLE" | grep -v "VARIABLES" > $CIR1.prn2
grep -v "FREQ" $CIR2.NOISE.prn | grep -v "End" > $CIR2.prn2

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
TEST1=$(grep -ic "VARIABLES = \" FREQ\"" $CIR1.NOISE.dat)
TEST2=$(grep -ic "\" INOISE\"" $CIR1.NOISE.dat)
TEST3=$(grep -ic "\" ONOISE\"" $CIR1.NOISE.dat)
TEST4=$(grep -ic "DATASETAUXDATA TIME=" $CIR1.NOISE.dat)
TEST5=$(grep -ic "ZONE F=POINT   T=\"Xyce data\"" $CIR1.NOISE.dat)
TEST6=$(grep -ic "End of Xyce(TM) Simulation" $CIR1.NOISE.dat)

EXITCODE=0

if [ "$TEST1" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of \"VARIABLES = \" FREQ\"\"  in tecplot header, got $TEST1" >&2
fi

if [ "$TEST2" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of INOISE variable declarations in tecplot header, got $TEST2" >&2
fi
if [ "$TEST3" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of ONOISE variable declarations in tecplot header, got $TEST3" >&2
fi
if [ "$TEST4" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of \"DATASETAUXDATA TIME=\" in tecplot header, got $TEST4" >&2
fi
if [ "$TEST5" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of \"ZONE F=POINT    T=\"Xyce data\"\" in tecplot header, got $TEST5" >&2
fi
if [ "$TEST6" -ne "1" ]
then
  EXITCODE=2
  echo "Wrong number of \"End of Xyce(TM) Simulation\" in tecplot footer, got $TEST6" >&2
fi

echo "Exit code = $EXITCODE"
exit $EXITCODE

