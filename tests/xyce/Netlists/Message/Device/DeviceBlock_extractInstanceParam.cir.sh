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

# These search strings are supposed to occur once in the error output.
# The ordering may change for parallel execution, so runAndCheckGroupedError
# is used.
@searchstrings = (["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 27",
   "Unexpectedly reached end of line while looking for parameters for device R2"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 28",
    " Unexpectedly reached end of line while looking for parameters for device R3"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 29",
    "Unexpectedly reached end of line while looking for parameters for device R4"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 30",
    " Unrecognized fields for device R5"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 30",
    "Unrecognized parameter 0 for device R5"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 31",
    "Unexpectedly reached end of line while looking for parameters for device R6"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 32",
    "Unexpectedly reached end of line while looking for parameters for device R7"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 37",
    "Composite parameter NODE for device YPDE!BJT1 must be enclosed in braces \\{}"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 45",
    "Invalid syntax \\(likely spurious comma\\) for composite parameter NODE for",
    "device YPDE!BJT2"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 53",
    "Invalid syntax \\(likely spurious comma\\) for composite parameter NODE for",
    "device YPDE!BJT3"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 61",
    "Invalid syntax \\(likely spurious comma\\) for composite parameter NODE for",
    "device YPDE!BJT4"],
   ["Netlist error in file DeviceBlock_extractInstanceParam.cir at or near line 73",
    "Invalid syntax \\(likely spurious comma\\) for composite parameter NODE for",
    "device YPDE!BJT5"]
);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n";
exit $retval


