#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This test simply checks that Xyce issues appropriate warnings

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 

# check various error cases
# this string should be in the warning messages of this Xyce run  
@searchstrings = ( ["Netlist warning in file multiOptionWarn.cir at or near line 13",
                    "VOLTLIM:  duplicate DEVICE parameter.  Using the first value found = 0"],
                   ["Netlist warning in file multiOptionWarn.cir at or near line 16",
                    " RELTOL:  duplicate TIMEINT parameter.  Using the first value found = 1.0e-4"],
                 );

print "Checking warning messages for multiple options lines\n";
$retval = $Tools->runAndCheckGroupedWarning($CIRFILE,$XYCE,@searchstrings);

if ($retval != 0)
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

print "Exit code = 0\n";
exit 0;



