#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
$GOLDPRN=$ARGV[4];

# check various error cases
# this string should be in the output of this failed Xyce run
@searchstrings = (["Netlist warning in file semicolon_fails.cir at or near line 21",
   "Expected value field for device R2, continuing with value of 0"],
  ["Netlist error in file semicolon_fails.cir at or near line 25",
   "Not enough fields on input line for device V2"],
  ["Netlist error in file semicolon_fails.cir at or near line 26",
   "Not enough fields on input line for device V3"],
  ["Netlist error in file semicolon_fails.cir at or near line 27",
   "Not enough fields on input line for device V4"],
  ["Netlist error in file semicolon_fails.cir at or near line 28",
   "Not enough fields on input line for device V"]);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
