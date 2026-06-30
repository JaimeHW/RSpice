#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

@CIR;
$CIR[0]="PCE_setAnalysisParams1.cir";
$CIR[1]="PCE_setAnalysisParams2.cir";
$CIR[2]="PCE_setAnalysisParams3.cir";

$exitcode = 0;

print "Testing $CIR[0]\n";
@searchstrings = ("STD_DEVIATIONS values for .PCE must be >= 0");

$retval = $Tools->runAndCheckError($CIR[0],$XYCE,@searchstrings);
if ($retval !=0)
{
  print "test failed for $CIR[0], see $CIR[0].stdout\n";
  $exitcode = $retval;
}
else
{
  print "test passed for $CIR[0]\n";
}

print "Testing $CIR[1]\n";
@searchstrings = ("ALPHA values for .PCE must be > 0");

$retval = $Tools->runAndCheckError($CIR[1],$XYCE,@searchstrings);
if ($retval !=0)
{
  print "test failed for $CIR[1], see $CIR[1].stdout\n";
  $exitcode = $retval;
}
else
{
  print "test passed for $CIR[1]\n";
}

print "Testing $CIR[2]\n";
@searchstrings = ("BETA values for .PCE must be > 0");

$retval = $Tools->runAndCheckError($CIR[2],$XYCE,@searchstrings);
if ($retval !=0)
{
  print "test failed for $CIR[2], see $CIR[2].stdout\n";
  $exitcode = $retval;
}
else
{
  print "test passed for $CIR[2]\n";
}

print "Exit code = $exitcode\n";
exit $exitcode
