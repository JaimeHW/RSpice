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
# Note that parens are escaped with \\ 
@searchstrings = ("Netlist error in file bad_dot_measure_line3.cir at or near line 17",
   "Error in .MEASURE line.  Could not parse measured variable",
   "Netlist error in file bad_dot_measure_line3.cir at or near line 18",
   "Error in .MEASURE line.  Could not parse measured variable",
   "Netlist error in file bad_dot_measure_line3.cir at or near line 19",
   "Error in .MEASURE line.  Could not parse measured variable",
   "Netlist error in file bad_dot_measure_line3.cir at or near line 20",
   "Error in .MEASURE line.  Could not parse measured variable",
   "Netlist error in file bad_dot_measure_line3.cir at or near line 23",
   "Error in .MEASURE line.  Could not parse measured variable",
   "Netlist error in file bad_dot_measure_line3.cir at or near line 24",
   "Error in .MEASURE line.  Could not parse measured variable",
);

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
