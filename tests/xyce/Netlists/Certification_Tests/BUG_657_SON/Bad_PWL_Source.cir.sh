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
#$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
#$GOLDPRN=$ARGV[4];

# check various error cases
# these strings should be in the output of this failed Xyce run
@searchstrings = ( ["Netlist error in file Bad_PWL_Source.cir at or near line 3",
    "PWL device missing source parameters: V1"],
    ["Netlist error in file Bad_PWL_Source.cir at or near line 6",
    "Could not parse time/value pairs for PWL function in device: V2"],
    ["Netlist error in file Bad_PWL_Source.cir at or near line 9",
    "Could not parse time/value pairs for PWL function in device: V3"],
    ["Netlist error in file Bad_PWL_Source.cir at or near line 12",
    "Could not parse time/value pairs for PWL function in device: V4"],
    ["Netlist error in file Bad_PWL_Source.cir at or near line 16",
    "Could not find file missingFile.txt for PWL function in device: V5"], 
    ["Netlist error in file Bad_PWL_Source.cir at or near line 21",
     "Problem reading badTimeVoltageList.csv",
     "File format must be comma, tab or space separated value. There should be no",
     "extra spaces or tabs around the comma if it is used as the separator."],
    ["Netlist error in file Bad_PWL_Source.cir at or near line 25",
     "Could not parse FILE specification for PWL function in device: V7"],
    ["Netlist error in file Bad_PWL_Source.cir at or near line 29",
     "Could not find file . for PWL function in device: V8"] );

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
