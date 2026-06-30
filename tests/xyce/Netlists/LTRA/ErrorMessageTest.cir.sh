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
@searchstrings = (["Netlist error in file ErrorMessageTest.cir at or near line 21",
   "Device model NOPARAMS: Invalid specification. Specify at least two of R, L,",
   "G, or C with nonzero values. Modes supported: RC, RG, LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 24",
   "Device model ONLYR: Invalid specification. Specify at least two of R, L, G,",
   "or C with nonzero values. Modes supported: RC, RG, LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 27",
   "Device model ONLYL: Invalid specification. Specify at least two of R, L, G,",
   "or C with nonzero values. Modes supported: RC, RG, LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 30",
   "Device model ONLYC: Invalid specification. Specify at least two of R, L, G,",
   "or C with nonzero values. Modes supported: RC, RG, LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 33",
   "Device model ONLYG: Invalid specification. Specify at least two of R, L, G,",
   "or C with nonzero values. Modes supported: RC, RG, LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 36",
   "Device model ALL: RLCG line not supported. Modes supported: RC, RG, LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 39",
   "Device model RL: RL transmission line not supported. Modes supported: RC, RG,",
   "LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 42",
   "Device model GC: Nonzero G \\(except RG\\) transmission line not supported. Modes",
   "supported: RC, RG, LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 45",
   "Device model GL: Nonzero G \\(except RG\\) transmission line not supported. Modes",
   "supported: RC, RG, LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 48",
   "Device model GLC: Nonzero G \\(except RG\\) transmission line not supported.",
   "Modes supported: RC, RG, LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 51",
   "Device model RGC: Nonzero G \\(except RG\\) transmission line not supported.",
   "Modes supported: RC, RG, LC, RLC"],
   ["Netlist error in file ErrorMessageTest.cir at or near line 54",
   "Device model RGL: Nonzero G \\(except RG\\) transmission line not supported."]);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
