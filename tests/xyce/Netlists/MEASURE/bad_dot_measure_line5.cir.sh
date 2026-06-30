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
@searchstrings = ( "Netlist error: FOURVP1 has illegal value for AT, NUMFREQ or GRIDSIZE",
                   "Netlist error: FOURVP2 has illegal value for AT, NUMFREQ or GRIDSIZE",
                   "Netlist error: FOURVP3 has illegal value for AT, NUMFREQ or GRIDSIZE" );

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
