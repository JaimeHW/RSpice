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

$baseline="push_pull_xyce_baseline.cir";
$restarted="push_pull_xyce_restart.cir";

`rm -f $baseline.prn $restarted.prn `;
`rm -f $baseline.err $restarted.err `;
`rm -f push_pull0*`;
# --------------------------------
# Run Xyce on the circuits, baseline first, then restarted.

$CMD="$XYCE $baseline > $baseline.out 2> $baseline.err";
#
# lower byte is not relevent here.  xyce's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0) { print "Xyce failed running $baseline.\nExit code = 10\n"; exit 10; }

# These next two conditionals are here because on Losedows with Intel compilers
# the exponent in the file name gets an extra zero.  For simplicity, just
# discard it so the netlists can find the restart file they're looking for.
if (-f "push_pull5e-0004")
{
    system("mv push_pull5e-0004 push_pull0.0005");
}

$CMD="$XYCE $restarted > $restarted.out 2> $restarted.err";
#
# lower byte is not relevent here.  xyce's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0) { print "Xyce failed running $restarted.\nExit code = 10\n"; exit 10; }

if ( not -s "$baseline.prn" ) { print "Exit code = 14\n"; exit 14; }
if ( not -s "$restarted.prn" ) { print "Exit code = 14\n"; exit 14; }
# --------------------------------
# Run xyce_verify on the output

$CMD="$XYCE_VERIFY $baseline $baseline.prn $restarted.prn > $restarted.prn.out 2> $restarted.prn.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0) { print "xyce_verify failed with $baseline.\n"; $failure = 1; }


if ($failure)
{
  print "Exit code = 2\n"; exit 2;
}
else
{
  print "Exit code = 0\n"; exit 0;
}

print "Exit code = 1\n"; exit 1;
