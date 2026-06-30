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
$absTol = 1e-10;
$relTol = 1e-8;
$zeroTol = 1e-15;
$pbAbsTol=1e-9;

# check that P() and W() match for the V source
$pVNPcolIdx = 2; # P(VNP) 
$wVNPcolIdx = 3; # W(VNP) 
$pRVNPcolIdx = 4; # W(RVNP) 
$pCVNPcolIdx = 5; # W(CVNP) 
$pLVNPcolIdx = 6; # W(LVNP) 

# Do the comparisons for voltage source.  Pass in a reference to @prnData
# compare source with "normal" polarity and positive amplitude
$retval = PowerCommon::compareTwoColumns(\@prnData,$pVNPcolIdx,$wVNPcolIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# compare source with "normal" polarity and negative amplitude
# Pass in a reference to @prnData
$pVNNcolIdx = 7; # P(VNN) 
$wVNNcolIdx = 8; # W(VNN) 
$retval = PowerCommon::comparePowerColumns(\@prnData,$pVNPcolIdx,$pVNNcolIdx,$wVNNcolIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# compare source with "reversed" polarity and positive amplitude
# Pass in a reference to @prnData
$pVRPcolIdx = 9; # P(VRP) 
$wVRPcolIdx = 10; # W(VRP) 
$retval = PowerCommon::comparePowerColumns(\@prnData,$pVNPcolIdx,$pVRPcolIdx,$wVRPcolIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# compare source with "reversed" polarity and negative amplitude
# Pass in a reference to @prnData
$pVRNcolIdx = 11; # P(VRN) 
$wVRNcolIdx = 12; # W(VRN) 
$retval = PowerCommon::comparePowerColumns(\@prnData,$pVNPcolIdx,$pVRNcolIdx,$wVRNcolIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# Check the overall power balance for the V source
@colList=($pVNPcolIdx,$pRVNPcolIdx,$pCVNPcolIdx,$pLVNPcolIdx);
$retval = PowerCommon::checkPowerBalance(\@prnData,\@colList,$pbAbsTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# check that P() and W() match for the I source
$pINPcolIdx = 13; # P(INP) 
$wINPcolIdx = 14; # W(INP) 
$pRINPcolIdx = 15; # W(RINP) 
$pCINPcolIdx = 16; # W(CINP) 
$pLINPcolIdx = 17; # W(LINP) 

# Do the comparisons for current source.  Pass in a reference to @prnData
# compare source with "normal" polarity and positive amplitude
$retval = PowerCommon::compareTwoColumns(\@prnData,$pINPcolIdx,$wINPcolIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# compare source with "normal" polarity and negative amplitude
# Pass in a reference to @prnData
$pINNcolIdx = 18; # P(INN) 
$wINNcolIdx = 19; # W(INN) 
$retval = PowerCommon::comparePowerColumns(\@prnData,$pINPcolIdx,$pINNcolIdx,$wINNcolIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# compare source with "reversed" polarity and positive amplitude
# Pass in a reference to @prnData
$pIRPcolIdx = 20; # P(IRP) 
$wIRPcolIdx = 21; # W(IRP) 
$retval = PowerCommon::comparePowerColumns(\@prnData,$pINPcolIdx,$pIRPcolIdx,$wIRPcolIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# compare source with "reversed" polarity and negative amplitude
# Pass in a reference to @prnData
$pIRNcolIdx = 22; # P(IRN) 
$wIRNcolIdx = 23; # W(IRN) 
$retval = PowerCommon::comparePowerColumns(\@prnData,$pINPcolIdx,$pIRNcolIdx,$wIRNcolIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# Check the overall power balance for the I source
@colList=($pINPcolIdx,$pRINPcolIdx,$pCINPcolIdx,$pLINPcolIdx);
$retval = PowerCommon::checkPowerBalance(\@prnData,\@colList,$pbAbsTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# final check on Exit code
if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

print "Exit code = 0\n"; 
exit 0;


