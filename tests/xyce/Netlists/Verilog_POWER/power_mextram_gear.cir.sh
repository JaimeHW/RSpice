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
$absTol = 1e-7;
$relTol = 5e-5;
$zeroTol = 5e-12;

$pCalcColIdx = 7; # P(q1) calculated for NPN
$pXyceColIdx = 8; # P(q1) reported by Xyce for NPN
$wXyceColIdx = 9; # W(q1) reported by Xyce for NPN

# Do the comparison for NPN.  Pass in a reference to @prnData
$retval1 = PowerCommon::comparePowerColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$wXyceColIdx,$absTol,$relTol,$zeroTol);
if ($retval1 != 0)
{
  print "Failed power test for NPN BJT\n";
}

$pCalcColIdx = 15; # P(q1) calculated for PNP
$pXyceColIdx = 16; # P(q1) reported by Xyce for PNP
$wXyceColIdx = 17; # W(q1) reported by Xyce for PNP
# Do the comparison for PNP.  Pass in a reference to @prnData
$retval2 = PowerCommon::comparePowerColumns(\@prnData,$pCalcColIdx,$pXyceColIdx,$wXyceColIdx,$absTol,$relTol,$zeroTol);
if ($retval2 != 0)
{
  print "Failed power test for PNP BJT\n";
}

if ($retval1 != 0) 
{ 
  print "Exit code = $retval1\n"; 
  exit $retval1; 
}
elsif ($retval2 != 0)
{ 
  print "Exit code = $retval2\n"; 
  exit $retval2; 
}

print "Exit code = 0\n"; 
exit 0;


