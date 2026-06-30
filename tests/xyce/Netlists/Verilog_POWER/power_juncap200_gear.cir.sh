#!/usr/bin/env perl

use lib '../POWER';
use XyceRegression::Tools;
use PowerCommon;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# First test that can parse the W() or P() on the .print line
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

# Special case for when this is a valgrind run
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

$pCalcColIdx = 5; # P(d) calculated
$pXyceColIdx = 6; # P(d) reported by Xyce
$wXyceColIdx = 7; # W(d) reported by Xyce

# Do the comparison.  Pass in a reference to @prnData
$retval = PowerCommon::comparePowerColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$wXyceColIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

$pCalcColIdx = 11; # P(d2) calculated
$pXyceColIdx = 12; # P(d2) reported by Xyce
$wXyceColIdx = 13; # W(d2) reported by Xyce

# Do the comparison.  Pass in a reference to @prnData
$retval = PowerCommon::comparePowerColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$wXyceColIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

print "Exit code = 0\n"; 
exit 0;


