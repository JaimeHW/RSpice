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
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

@CIR;
$CIR[0]="AC_setAnalysisParams1.cir";
$CIR[1]="AC_setAnalysisParams2.cir";
$CIR[2]="AC_setAnalysisParams3.cir";
$CIR[3]="AC_setAnalysisParams4.cir";

$exitcode = 0;

print "Testing $CIR[0]\n";
@searchstrings = ("Netlist error: Points Value parameter on .AC line must be an integer",
    "Netlist error: Points Value parameter on .AC line must be >= 1");

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

foreach $idx (1 .. 2)
{ 
  print "Testing $CIR[$idx]\n";
  @searchstrings = ("Netlist error: Illegal values for start or end frequencies on .AC line. Both",
                    "values must be > 0");

  $retval = $Tools->runAndCheckError($CIR[$idx],$XYCE,@searchstrings);
  if ($retval !=0)
  {
    print "test failed for $CIR[$idx], see $CIR[$idx].stdout\n";
    $exitcode = $retval;
  }
  else
  {
    print "test passed for $CIR[$idx]\n";
  }
}

print "Testing $CIR[3]\n";
@searchstrings = ("Netlist error: End frequency must not be less than start frequency on .AC line");

$retval = $Tools->runAndCheckError($CIR[3],$XYCE,@searchstrings);
if ($retval !=0)
{
  print "test failed for $CIR[3], see $CIR[3].stdout\n";
  $exitcode = $retval;
}
else
{
  print "test passed for $CIR[3]\n";
}    

print "Exit code = $exitcode\n";
exit $exitcode


