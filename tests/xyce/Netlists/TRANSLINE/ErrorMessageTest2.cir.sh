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
@searchstrings = (["Netlist error in file ErrorMessageTest2.cir at or near line 19",
     "Device instance TLINE1: Neither time delay \\(TD\\) nor frequency \\(F\\) given."],
     ["Netlist error in file ErrorMessageTest2.cir at or near line 19",
     "Device instance TLINE1: Invalid \\(zero or negative\\) frequency \\(F\\) given."],
     ["Netlist error in file ErrorMessageTest2.cir at or near line 20",
     "Device instance TLINE2: Both time delay \\(TD\\) and frequency \\(F\\) given."]);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
