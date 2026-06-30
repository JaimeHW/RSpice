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

# First test that can parse the I(Lname) syntax 
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
$absTol = 1e-8;
$relTol = 1e-8;
$zeroTol = 1e-15;
$pCalcColIdx = 3; # I(L1)       
$pXyceColIdx = 4; # N(YMIN!K1_L1_BRANCH)

# Do the comparison.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

$pCalcColIdx = 5; # I(L2)       
$pXyceColIdx = 6; # N(YMIN!K1_L2_BRANCH)

# Do the comparison.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

$pCalcColIdx = 7; # I(XTXS:L1S)   
$pXyceColIdx = 8; # N(Y:XTXS:MIN!K1S_L1S_BRANCH) 

# Do the comparison.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

$pCalcColIdx = 9; # I(XTXS:L2S)
$pXyceColIdx = 10; # N(Y:XTXS:MIN!K1S_L2S_BRANCH)

# Do the comparison.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}



print "Exit code = 0\n"; 
exit 0;


