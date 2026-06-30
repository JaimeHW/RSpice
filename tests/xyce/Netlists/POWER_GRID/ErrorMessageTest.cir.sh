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
@searchstrings = (["Netlist error in file ErrorMessageTest.cir at or near line 16",
     "Device instance YPGBR!BR1: Analysis Type must be IV, PQR or PQP in power grid",
     "device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 17",
     "Device instance YPGBS!BS1: Analysis Type must be IV, PQR or PQP in power grid",
     "device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 18",
     "Device instance YPGGB!GB1: Analysis Type must be IV, PQR or PQP in power grid",
     "device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 19",
     "Device instance YPGTR!TR1: Analysis Type must be IV, PQR or PQP in power grid",
     "device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 22",
     "Device instance YPGGB!GB2: Only PQP Analysis Type is supported for",
     "PowerGridGenBus device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 23",
     "Device instance YPGGB!GB3: Only PQP Analysis Type is supported for",
     "PowerGridGenBus device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 26",
     "Device instance YPGTR!TR2: Transformer Type must be FT, VT or PS in",
     "PowerGridTransformer device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 29",
     "Device instance YPGTR!TR3: Incorrect number of inputs in power grid device.",
     "Found 5, should be 4 for transformer type FT power grid transformer."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 32",
     "Device instance YPGBR!BR2: Either R or X must be specified for", 
     "PowerGridBranch device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 33",
     "Device instance YPGBR!BR3: Either R or X must be non-zero for PowerGridBranch",
     "device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 34",
     "Device instance YPGBR!BR4: Either R or X must be non-zero for PowerGridBranch",
     " device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 37",
     "Device instance YPGBS!BS2: Either G or B must be specified for",
     "PowerGridBusShunt device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 38",
     "Device instance YPGBS!BS3: Either G or B must be non-zero for",
     "PowerGridBusShunt device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 39",
     "Device instance YPGBS!BS4: Either G or B must be non-zero for",
     "PowerGridBusShunt device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 43",
     "Device instance YPGTR!TR4: TR must be non-zero for PowerGridTransformer",
     "device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 44",
     "Device instance YPGTR!TR5: R or X must be specified for PowerGridTransformer",
     "device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 45",
     "Device instance YPGTR!TR6: Either R or X must be non-zero for",
     "PowerGridTransformer device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 46",
     "Device instance YPGTR!TR7: Either R or X must be non-zero for",
     "PowerGridTransformer device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 49",
     "Device instance YPGGB!GB4: VM must be positive for PowerGridGenBus bus",
     "device."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 53",
     "Device instance YAWL!AWL1: Upper limit \\(UL\\) must be greater than lower limit",
     "\\(LL\\)"],
    ["Netlist error in file ErrorMessageTest.cir at or near line 54",
    "Device instance YAWL!AWL2: Time constant \\(T\\) must be positive"]);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
