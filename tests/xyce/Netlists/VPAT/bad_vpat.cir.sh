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
@searchstrings = ( ["Netlist error in file bad_vpat.cir at or near line 11",
    "Device instance V1: VHI, VLO, TD, TR, TF, TSAMPLE and DATA parameters are all",
    "required for PAT source",
    "Netlist error in file bad_vpat.cir at or near line 11",
    "Device instance V1: Invalid DATA field for the PAT source function"],
    ["Netlist error in file bad_vpat.cir at or near line 15",
     "Device instance V2: TR, TF and TSAMPLE must all be non-negative for the PAT",
     "source function"],
    ["Netlist error in file bad_vpat.cir at or near line 16",
     "Device instance V3: TR, TF and TSAMPLE must all be non-negative for the PAT",
     "source function"],
    ["Netlist error in file bad_vpat.cir at or near line 17",
     "Device instance V4: TR, TF and TSAMPLE must all be non-negative for the PAT",
     "source function"],
    ["Netlist error in file bad_vpat.cir at or near line 20",
     "Device instance V5: Invalid DATA field for the PAT source function"],
    ["Netlist error in file bad_vpat.cir at or near line 21",
     "Device instance V6: Invalid bit symbol in DATA field for the PAT source"],
    ["Netlist error in file bad_vpat.cir at or near line 24",
     "Device instance V7: Only RB=1 is supported for the PAT source function"]
);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval;
