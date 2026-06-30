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
# this string should be in the output of this failed Xyce run.
# Notes that searchstrings is now a 2D array.  This allows runAndCheckPairedError to
# make sure that each pair of lines is adjacent in the output.  
@searchstrings = (["Netlist error in file invalid_instance_parameters.cir at or near line 18",
    "Device instance TLINE1: Invalid \\(zero or negative\\) impedance \\(Z0\\) given."], 
    ["Netlist error in file invalid_instance_parameters.cir at or near line 21",
    "Device instance TLINE2: Zero or negative time delay."],
    ["Netlist error in file invalid_instance_parameters.cir at or near line 24",
    "Device instance TLINE3: Invalid \\(zero or negative\\) NL parameter given."],
    ["Netlist error in file invalid_instance_parameters.cir at or near line 25",
    "Device instance TLINE4: Invalid \\(zero or negative\\) frequency \\(F\\) given."],
    ["Netlist error in file invalid_instance_parameters.cir at or near line 26",
    "Device instance TLINE5: Invalid \\(zero or negative\\) frequency \\(F\\) given."]);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
