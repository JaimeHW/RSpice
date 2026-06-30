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

# check that P() and W() match for the Voltage form of the B source
$pTopIdx = 2; # P(B) in top-level
$pSubIdx = 3; # P(B) in subcircuit
$pTopIdx = 4; # W(B) in top-level
$pSubIdx = 5; # W(B) in subcircuit

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

$retval = PowerCommon::compareTwoColumns(\@prnData,$wTopIdx,$wSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# check that P() and W() match for the Current form of the B source
$pTopIdx = 6; # P(B) in top-level
$pSubIdx = 7; # P(B) in subcircuit
$pTopIdx = 8; # W(B) in top-level
$pSubIdx = 9; # W(B) in subcircuit

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

$retval = PowerCommon::compareTwoColumns(\@prnData,$wTopIdx,$wSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# check that P() and W() match for the Linear Form of the E source
$pTopIdx = 10; # P(E) in top-level
$pSubIdx = 11; # P(E) in subcircuit
$pTopIdx = 12; # W(E) in top-level
$pSubIdx = 13; # W(E) in subcircuit

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

$retval = PowerCommon::compareTwoColumns(\@prnData,$wTopIdx,$wSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# check that P() and W() match for the Value Form of the E source
$pTopIdx = 14; # P(E) in top-level
$pSubIdx = 15; # P(E) in subcircuit
$pTopIdx = 16; # W(E) in top-level
$pSubIdx = 17; # W(E) in subcircuit

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

$retval = PowerCommon::compareTwoColumns(\@prnData,$wTopIdx,$wSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# check that P() and W() match for the Linear Form of the F source
$pTopIdx = 18; # P(F) in top-level
$pSubIdx = 19; # P(F) in subcircuit
$pTopIdx = 20; # W(F) in top-level
$pSubIdx = 21; # W(F) in subcircuit

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# check that P() and W() match for the Linear Form of the G source
$pTopIdx = 22; # P(G) in top-level
$pSubIdx = 23; # P(G) in subcircuit
$pTopIdx = 24; # W(G) in top-level
$pSubIdx = 25; # W(G) in subcircuit

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# check that P() and W() match for the Value Form of the G source
$pTopIdx = 26; # P(G) in top-level
$pSubIdx = 27; # P(G) in subcircuit
$pTopIdx = 28; # W(G) in top-level
$pSubIdx = 29; # W(G) in subcircuit

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# check that P() and W() match for the Linear Form of the H source
$pTopIdx = 30; # P(H) in top-level
$pSubIdx = 31; # P(H) in subcircuit
$pTopIdx = 32; # W(H) in top-level
$pSubIdx = 33; # W(H) in subcircuit

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# Do the comparisons.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$pTopIdx,$pSubIdx,$absTol,$relTol,$zeroTol);

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


