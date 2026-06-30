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
# this string should be in the output of this failed Xyce run  .
# Note: must escape ( ) with \\ to get the CheckError() function to work.
@searchstrings = (["Netlist error in file ErrorMessageTest.cir at or near line 14",
   "Device instance R1A: Multiplicity Factor \\(M\\) must be non-negative"],
  ["Netlist error in file ErrorMessageTest.cir at or near line 13",
   "Device instance R1: Multiplicity Factor \\(M\\) must be non-negative"],
  ["Netlist error in file ErrorMessageTest.cir at or near line 20",
   "Device instance R2: Multiplicity Factor \\(M\\) must be non-negative"],
  ["Netlist error in file ErrorMessageTest.cir at or near line 26",
   "Device instance L3: Multiplicity Factor \\(M\\) must be non-negative"],
  ["Netlist error in file ErrorMessageTest.cir at or near line 31",
   "Device instance C4: Multiplicity Factor \\(M\\) must be non-negative"]);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
