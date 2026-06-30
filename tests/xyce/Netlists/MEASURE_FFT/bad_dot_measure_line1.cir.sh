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
# These strings should be in the output of this failed Xyce run.
# Note that ( ) and : characters must be escaped with \\
@searchstrings = ("Netlist error: Too many dependent variables for ENOB measure, \"M9\"",
   "Netlist error: Too many dependent variables for FIND measure, \"M10\"",
   "Netlist error: Too many dependent variables for SFDR measure, \"M11\"",
   "Netlist error: Too many dependent variables for SNDR measure, \"M12\"",
   "Netlist error: Too many dependent variables for SNR measure, \"M13\"",
   "Netlist error: Too many dependent variables for THD measure, \"M14\"",
   "Netlist error: Complex operators such as VR not allowed for output variable",
   "for ENOB measure M1 for FFT measure mode",
   "Netlist error: Complex operators such as II not allowed for output variable",
   "for SFDR measure M2 for FFT measure mode",
   "Netlist error: Complex operators such as VM not allowed for output variable",
   "for SNDR measure M3 for FFT measure mode",
   "Netlist error: Complex operators such as IP not allowed for output variable",
   "for THD measure M4 for FFT measure mode",
   "Netlist error: Complex operators such as VDB not allowed for output variable",
   "for THD measure M5 for FFT measure mode",
   "Netlist error: Expressions not allowed for output variable for FIND measure M6",
   "for FFT measure mode",
   "Netlist error: Only V and I operators allowed for output variable for FIND",
   "measure M7 for FFT measure mode",
   "Netlist error: Only V and I operators allowed for output variable for FIND",
   "measure M8 for FFT measure mode",
   "Netlist error: Only V and I operators allowed for output variable for FIND",
   "measure M15 for FFT measure mode",
   "Netlist error: Multi-terminal lead current designator IB not allowed for",
   "output variable for FIND measure M16 for FFT measure mode",
   "Netlist error: Multi-terminal lead current designator IC not allowed for",
   "output variable for FIND measure M17 for FFT measure mode",
   "Netlist error: Multi-terminal lead current designator IE not allowed for",
   "output variable for FIND measure M18 for FFT measure mode",
   "Netlist error: Multi-terminal lead current designator ID not allowed for",
   "output variable for FIND measure M19 for FFT measure mode",
   "Netlist error: Multi-terminal lead current designator IS not allowed for",
   "output variable for FIND measure M20 for FFT measure mode",
   "Netlist error: Multi-terminal lead current designator IG not allowed for",
   "output variable for FIND measure M21 for FFT measure mode"
);

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval;
