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
@searchstrings = ( "Netlist error: ONE has incomplete MEASURE line",
                   "Netlist error: TWOA has incomplete MEASURE line",
                   "Netlist error: TWOB has incomplete MEASURE line",
                   "Netlist error: TWOC has incomplete MEASURE line",
                   "Netlist error: TWOD has incomplete MEASURE line",
                   "Netlist error: THREEA has invalid MEASURE line",
                   "Netlist error: THREEB has invalid MEASURE line",
                   "Netlist error: THREEC has invalid MEASURE line",
                   "AT keyword not allowed in TARG block for measure FOURA",
                   "Netlist error: FIVEA has incomplete MEASURE line",
                   "Netlist error: Invalid value for RISE, FALL or CROSS for measure RISEBOGO",
                   "Netlist error: Invalid value for RISE, FALL or CROSS for measure FALLBOGO",
                   "Netlist error: Invalid value for RISE, FALL or CROSS for measure CROSSBOGO",
                   "Netlist error: Attempt to evaluate expression \\{STARTTIME\\}, which contains",
                   "unknowns",
                   "Netlist error: SIXA has invalid MEASURE line",
                   "Netlist error: SIXB has invalid MEASURE line",
                   "Netlist error: SEVENA has invalid MEASURE line",
                   "Netlist error: SEVENB has invalid MEASURE line");

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
