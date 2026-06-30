#!/usr/bin/env perl

use XyceRegression::Tools;
use PowerCommon;

$Tools = XyceRegression::Tools->new();

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
$GOLDPRN=$ARGV[4];

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# First test that can parse the W() or P() on the .print line
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

# get the data in the .PRN file, return as an array
@prnData = PowerCommon::parseTranPrnFile($CIRFILE);
if ($#prnData == 0)
{
  print "no data found in .prn file\n";
  print "Exit code = 2\n";
  exit 2;
}

# set the comparison tolerances and the columns to compare for this test
$absTol = 1e-7;
$pbAbsTol = 1e-9;
$relTol = 2e-5;
$zeroTol = 1e-14;

# set return value to 0 (success)
$retval=0;

$pCalcColIdx = 4; # P(TLINE1) calculated, via formula
$pXyceColIdx = 2; # P(TLINE1) reported by Xyce 
$wXyceColIdx = 3; # W(TLINE1) reported by Xyce
# Do the comparison based on the formula.  Pass in a reference to @prnData
$retval1 = PowerCommon::comparePowerColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$wXyceColIdx,$absTol,$relTol,$zeroTol);
if ($retval1 != 0)
{
  print "Failed power test for TLINE1, based on expected calculation result\n";
  $retval=$retval1;
}

# Check the overall power balance just for P, initially
# use pbAbsTol for the this part of the test. 

$pVColIdx=  5;      # P(Voltage Source)
$pRinColIdx = 6;    # P(Input resistor)
$pRloadColIdx = 7;  # P(Load Resistor)
@colList=($pVColIdx,$pRinColIdx,$pRloadColIdx,$pXyceColIdx);
$retval2 = PowerCommon::checkPowerBalance(\@prnData,\@colList,$pbAbsTol);
if ($retval2 != 0)
{
  print "Failed power balance test for overall circuit\n";
  $retval=$retval2;
}

# If $retval is not equal to zero, then the return exit code
# will be from the last test failure.  However, the .sh
# file will print out a list of which tests failed.
print "Exit code = $retval\n"; 
exit $retval;


