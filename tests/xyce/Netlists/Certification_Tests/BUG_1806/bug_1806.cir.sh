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

CIR1="bug_1806_2a.cir"
CIR2="bug_1806_2.cir"

myname="bug_1806.cir"

rm -f $CIR1.prn $CIR1.err
rm -f $CIR2.prn $CIR2.err
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

# this is a perl fragment, not a shell script fragment.  Commenting out for how.
# If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh", instead of the rest of this .sh file, and then exit
#if ($XYCE_VERIFY =~ m/valgrind_check/)
#{
  #print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
  #if (system("$XYCE_VERIFY $CIR1 junk $CIR1.prn > $CIR1.prn.out 2>&1 $CIR1.prn.err"))
  #{
    #print "Exit code = 2 \n";
    #exit 2;
  #}
  #else
  #{
    #print "Exit code = 0 \n";
    #exit 0;
  #}
#}

$XYCE_VERIFY $CIR1 $CIR2.prn $CIR1.prn  > $myname.prn.out 2> $myname.prn.err
RETVAL=$?
if [ "$RETVAL" -ne "0" ]; then
  echo "Exit code = 2"
else
  echo "Exit code = 0"
fi
exit $RETVAL

