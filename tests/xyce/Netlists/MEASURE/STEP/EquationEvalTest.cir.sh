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
$GOLDPRN=$ARGV[4];  # Gold .prn file used to check EQN measure output in that file

# number of steps in the netlist CIRFILE
$numSteps=2;  

# Verify that measure works with .STEP.
$retval=MeasureCommon::checkTranStepResults($XYCE,$CIRFILE,$numSteps);

# If .MEASURE worked, then test -remeasure.  That test must use tolerances, since 
# measure and re-measure results may differ slightly on the same platform.  These
# tolerances are used for the check of the stdout.
$absTol = 1.0e-4;
$relTol = 0.02;
$zeroTol = 1.0e-5;
if ($retval == 0)
{
  $retval = MeasureCommon::checkRemeasure($XYCE,$XYCE_VERIFY,$CIRFILE,$absTol,$relTol,$zeroTol,"prn",$numSteps);
  if ($retval != 0)
  { 
    print "Re-measure failed\n";
  }
}

if ($retval != 0)
{
  print "Exit code = $retval\n";
  exit $retval;
}

# Also test that the EQN measures are correct in the output .prn file
# use file_compare since xyce verify doesn't like NOINDEX format.  This
# check currently uses a gold standard .prn file.
$fc=$XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$CMD="$fc $CIRFILE.prn $GOLDPRN $absTol $relTol $zeroTol >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
$retval = system($CMD);
$retval= $retval >> 8;
if ($retval !=0 )
{
  print STDERR "Failed comparison of EQN Measure values in output .prn file with $retval\n";
  print "Exit code = 2\n";
  exit 2;
}

print "Exit code = $retval\n";
exit $retval;

