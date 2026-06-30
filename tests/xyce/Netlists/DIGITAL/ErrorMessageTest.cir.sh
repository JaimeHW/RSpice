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
@searchstrings = (["Netlist error in file ErrorMessageTest.cir at or near line 34",
                  "Device instance UINV!INV1: Incorrect number of nodes in digital device. Found",
                  "3, should be 4"],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 35",
                  "Device instance UBUF!BUF2: Incorrect number of nodes in digital device. Found",
                  "3, should be 4"],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 38",
                  "Device instance UAND!AND3!4: too few I/O nodes on instance line."],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 38",
                  "Device instance UAND!AND3!4: Incorrect number of nodes in digital device.",
                  "Found 6, should be 7"],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 39",
                  "Device instance UNAND!NAND4!4: too few I/O nodes on instance line."],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 39",
                  "Device instance UNAND!NAND4!4: Incorrect number of nodes in digital device.",
                  "Found 6, should be 7"],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 40",
                  "Device instance UOR!OR5!4: too few I/O nodes on instance line."],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 40",
                  "Device instance UOR!OR5!4: Incorrect number of nodes in digital device. Found",
                  "6, should be 7"],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 41",
                  "Device instance UNOR!NOR6!4: too few I/O nodes on instance line."],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 41",
                  "Device instance UNOR!NOR6!4: Incorrect number of nodes in digital device.",
                  "Found 6, should be 7"],
                  ["Netlist warning in file ErrorMessageTest.cir at or near line 44",
                  "Device instance YNOT!NOT7: Y-type digital device is deprecated. Consider",
                  "using U-type digital device instead."],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 47",
                  "Device instance UXOR!XOR8: Incorrect number of nodes in digital device. Found",
                  "4, should be 5"],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 48",
                  "Device instance UNXOR!NXOR9: Incorrect number of nodes in digital device.",
                  "Found 4, should be 5"],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 51",
                  "Device instance UADD!ADD10: Incorrect number of nodes in digital device.",
                  "Found 6, should be 7"],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 54",
                  "Device instance UDLTCH!DLTCH11: Incorrect number of nodes in digital device.",
                  "Found 7, should be 8"],
                  ["Netlist error in file ErrorMessageTest.cir at or near line 55",
                  "Device instance UDFF!DFF12: Incorrect number of nodes in digital device.",
                  "Found 7, should be 8"] );

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
