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

$baseline="voltDepCap_baseline.cir";
$testcir="voltDepCap_test.cir";
$testcir1="voltDepCharge_test.cir";

`rm -f $baseline.prn $testcir.prn`;
`rm -f $baseline.err $testcir.prn`;

$CMD="$XYCE $baseline > $baseline.out 2> $baseline.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }

$CMD="$XYCE $testcir > $testcir.out 2> $testcir.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }
$CMD="$XYCE $testcir1 > $testcir1.out 2> $testcir1.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }

$CMD="$XYCE_VERIFY --goodres=$baseline.res $baseline $baseline.prn $testcir.prn > $testcir.prn.out 2> $testcir.prn.err";
if (system("$CMD") != 0) { $failure = 1; }

$CMD="$XYCE_VERIFY --goodres=$baseline.res $baseline $baseline.prn $testcir1.prn > $testcir1.prn.out 2> $testcir1.prn.err";
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

