#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.  

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CIR1="ring51.cir";
$CIR2="ring51_loca.cir";

$retval=$Tools->wrapXyce($XYCE,$CIR1);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

`rm -f $CIR2.HOMOTOPY.prn`;
$retval=$Tools->wrapXyce($XYCE,$CIR2);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

if ( not -s "$CIR1.prn" ) { print "Exit code = 14\n"; exit 14; }
`grep -v "Index" $CIR1.prn | grep -v "End" > $CIR1.prn2`;
if ( not -s "$CIR2.HOMOTOPY.prn" ) { print "Exit code = 14\n"; exit 14; }
`grep -v "Index" $CIR2.HOMOTOPY.prn | grep -v "End" > $CIR2.prn2`;

$retval = system("diff $CIR1.prn2 $CIR2.prn2 > $CIR1.prn.out 2> $CIR1.prn.err");
if ($retval == 0) { print "Exit code = 0\n"; exit 0; } else { print "Exit code = 2\n"; exit 2; }

# There are some manual tests in the README.  However, these tests are
# now covered (directly or indirectly) by other automated homotopy tests.

