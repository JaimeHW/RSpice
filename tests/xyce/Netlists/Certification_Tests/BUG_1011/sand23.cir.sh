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

XYCE=$1
CIRFILE=$4
GOLDPRN=$5

GOLDDEVS=`echo "$GOLDPRN" | sed 's/sand23\.cir\.prn/dev_count/g'`
if [ ! -f $GOLDDEVS ];         then echo "Exit code = 1"
 exit 1 ; fi
rm -f devs
$XYCE $CIRFILE > $CIRFILE.out

# don't try to do the diff if Xyce exits abnormally
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

sed -n '/Device Count Summary/,/Total Devices/p' $CIRFILE.out | grep " level " > devs 2> $CIRFILE.err

perl -i -pe 's/\r//g' devs
STAT=`diff devs $GOLDDEVS | wc -l | sed 's/ //g'`
if [ "$STAT" -ne "0" ]; then
  diff devs $GOLDDEVS > diffs
  echo "Exit code = 2"
  exit 2
fi
echo "Exit code = 0"
exit 0

