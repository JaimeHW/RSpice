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

# do a valgrind run.  This is done only for TRAP for the linear
# mutual inductor device.
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
  print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
  if (system("$XYCE_VERIFY $CIRFILE junk $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
  {
    print "Exit code = 2 \n";
    exit 2;
  }
  else
  {
    print "Exit code = 0 \n";
    exit 0;
  }
}

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

# Check component inductor L1
$pCalcColIdx = 6; # P(L1) calculated
$pXyceColIdx = 4; # P(L1) reported by Xyce
$wXyceColIdx = 5; # W(L1) reported by Xyce

# Do the comparison.  Pass in a reference to @prnData
$retval1 = PowerCommon::comparePowerColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$wXyceColIdx,$absTol,$relTol,$zeroTol);
if ($retval1 != 0) 
{ 
  print "Failed power test for Component Inductor L1, based on calculation\n"; 
  $retval = $retval1; 
}

# Check component inductor L2
$pCalcColIdx = 11; # P(L2) calculated
$pXyceColIdx = 9; # P(L2) reported by Xyce
$wXyceColIdx = 10; # W(L2) reported by Xyce

# Do the comparison.  Pass in a reference to @prnData
$retval2 = PowerCommon::comparePowerColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$wXyceColIdx,$absTol,$relTol,$zeroTol);
if ($retval2 != 0) 
{ 
  print "Failed power test for Component Inductor L2, based on calculation\n"; 
  $retval = $retval2; 
}
 
# Check component inductor L1 in Subcircuit X1
$pCalcColIdx = 16; # P(X1:L1) calculated
$pXyceColIdx = 14; # P(X1:L1) reported by Xyce
$wXyceColIdx = 15; # W(X1:L1) reported by Xyce

# Do the comparison.  Pass in a reference to @prnData
$retval3 = PowerCommon::comparePowerColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$wXyceColIdx,$absTol,$relTol,$zeroTol);
if ($retval3 != 0) 
{ 
  print "Failed power test for Component Inductor X1:L1, based on calculation\n"; 
  $retval = $retval3; 
}

# Check component inductor L2 in Subcircuit X2
$pCalcColIdx = 21; # P(X1:L2) calculated
$pXyceColIdx = 19; # P(X1:L2) reported by Xyce
$wXyceColIdx = 20; # W(X1:L2) reported by Xyce

# Do the comparison.  Pass in a reference to @prnData
$retval4 = PowerCommon::comparePowerColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$wXyceColIdx,$absTol,$relTol,$zeroTol);
if ($retval4 != 0) 
{ 
  print "Failed power test for Component Inductor X1:L2, based on calculation\n"; 
  $retval = $retval4; 
}

print "Exit code = $retval\n"; 
exit $retval;


