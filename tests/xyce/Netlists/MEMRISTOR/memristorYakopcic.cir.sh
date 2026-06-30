#!/usr/bin/env perl

use lib '.';
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

# First test that a prn file was made. 
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
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
#
# Note, the tolerances are pretty loose but we are comparing a set of 
# nonlinear equations implemented as a device and as a sub-circuit.  
#
$absTol = 5e-4;
$relTol = 5e-4;
$zeroTol = 1e-15;



$Idx1 = 2; # V drop over memristor 1 
$Idx2 = 3; # V drop over memristor 2 
# Do the comparison.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$Idx1,$Idx2,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Memristor voltage drops differ.\nExit code = $retval\n"; 
  exit $retval; 
}

$Idx1 = 4; # current through memristor 1 
$Idx2 = 5; # current through memristor 2 
# Do the comparison.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$Idx1,$Idx2,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "Current differes between two memristors.\nExit code = $retval\n"; 
  exit $retval; 
}

$Idx1 = 6; # state var in memristor 1 
$Idx2 = 7; # state var in memristor 2 
# Do the comparison.  Pass in a reference to @prnData
$retval = PowerCommon::compareTwoColumns(\@prnData,$Idx1,$Idx2,$absTol,$relTol,$zeroTol);

if ($retval != 0) 
{ 
  print "State var differes between two memristors.\nExit code = $retval\n"; 
  exit $retval; 
}


print "Exit code = 0\n"; 
exit 0;


