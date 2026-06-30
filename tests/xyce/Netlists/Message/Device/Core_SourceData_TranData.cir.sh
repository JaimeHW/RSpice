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

# these search strings are supposed to occur one right after the other in the
# error output.
@searchstrings = (["Netlist error in file Core_SourceData_TranData.cir at or near line 7",
     "Device instance V1: V0, VA and FREQ are required for the SIN source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 10",
     "Device instance V1A: V0, VA and FREQ are required for the SIN source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 13",
     "Device instance V1B: V0, VA and FREQ are required for the SIN source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 16",
     "Device instance I2: V0, VA and FREQ are required for the SIN source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 20",
     "Device instance V3: V1 and V2 are required for the EXP source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 23",
     "Device instance V3A: V1 and V2 are required for the EXP source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 26",
     "Device instance I4: V1 and V2 are required for the EXP source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 30",
     "Device instance V5: V0 and VA are required for the SFFM source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 33",
     "Device instance V5A: V0 and VA are required for the SFFM source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 36",
     "Device instance I6: V0 and VA are required for the SFFM source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 40",
     "Device instance V7: V1 is required for the PULSE source function"],
    ["Netlist error in file Core_SourceData_TranData.cir at or near line 43",
     "Device instance I8: V1 is required for the PULSE source function"]
);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n";
exit $retval


