#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# these search strings are supposed to occur in the error output
# Note: double escaping dot (\\.) matches a single dot rather 
# than any single character.  
@searchstrings = (["Netlist error in file DeviceBlock_extractSwitchDeviceData\\.cir at or near line 7",
    "No model found for switch device S1"],
    ["Netlist error in file DeviceBlock_extractSwitchDeviceData\\.cir at or near line 10",
     "Wrong number of nodes for switch device S2"],
    ["Netlist error in file DeviceBlock_extractSwitchDeviceData\\.cir at or near line 14",
     "Wrong number of nodes for switch device SW1"],
    ["Netlist error in file DeviceBlock_extractSwitchDeviceData\\.cir at or near line 18",
     "Wrong number of nodes for switch device W1"],
    ["Netlist error in file DeviceBlock_extractSwitchDeviceData\\.cir at or near line 21",
     "CONTROL param found for W switch device W2"]
);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);

print "Exit code = $retval\n";
exit $retval;

