#!/usr/bin/env perl

use MeasureCommon;

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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# number of steps in the netlist CIRFILE
$numSteps=2;  

# Verify that measure works with .STEP.
$retval=MeasureCommon::checkTranStepResults($XYCE,$CIRFILE,$numSteps);

# If .MEASURE worked, then test -remeasure.  
if ($retval == 0)
{
  # -remeasure test must use tolerances, since measure and re-measure results may differ slightly 
  #on the same platform.
  $absTol = 3.0e-3;
  $relTol = 0.02;
  $zeroTol = 1.0e-5;
  $retval = MeasureCommon::checkRemeasure($XYCE,$XYCE_VERIFY,$CIRFILE,$absTol,$relTol,$zeroTol,"prn",$numSteps);
}

print "Exit code = $retval\n";
exit $retval;


