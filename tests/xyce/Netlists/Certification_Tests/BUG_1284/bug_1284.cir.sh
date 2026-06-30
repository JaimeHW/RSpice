#!/usr/bin/env perl

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
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$baseline="bug_1284_baseline.cir";
$initial="bug_1284_first.cir";
$restarted="bug_1284_restarted.cir";

`rm -f $baseline.prn $initial.prn $restarted.prn`;
`rm -f $baseline.err $initial.err $restarted.err`;
`rm -f trans_test*`;
# --------------------------------
# Run Xyce on the circuits, baseline first, then restarted.

$CMD="$XYCE $baseline > $baseline.out 2> $baseline.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }

$CMD="$XYCE $initial > $initial.out 2> $initial.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }

# These next two conditionals are here because on Losedows with Intel compilers
# the exponent in the file name gets an extra zero.  For simplicity, just
# discard it so the netlists can find the restart file they're looking for.
if (-f "trans_test2e-008")
{
    system("mv trans_test2e-008 trans_test2e-08");
}

$CMD="$XYCE $restarted > $restarted.out 2> $restarted.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }

if ( not -s "$baseline.prn" ) { print "Exit code = 14\n"; exit 14; }
if ( not -s "$initial.prn" ) { print "Exit code = 14\n"; exit 14; }
if ( not -s "$restarted.prn" ) { print "Exit code = 14\n"; exit 14; }
# --------------------------------
# Run xyce_verify on the output

$CMD="$XYCE_VERIFY $baseline $baseline.prn $restarted.prn > $restarted.prn.out 2> $restarted.prn.err";
if (system("$CMD") != 0) { $failure = 1; }

if ($failure)
{
  print "Exit code = 2\n"; exit 2;
}
else
{
  print "Exit code = 0\n"; exit 0;
}

print "Exit code = 1\n"; exit 1;
