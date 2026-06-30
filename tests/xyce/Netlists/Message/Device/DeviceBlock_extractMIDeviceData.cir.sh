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

$CIRFILE =~ s/\.cir$//; # remove the .cir at the end.
$exitcode=0;

# these search strings are supposed to occur in the error output
# for the netlist ParameterBlock_extractMIDeviceData1.cir.
@searchstrings = (["Netlist error: Illegal value found for device YMIN!K1"],
      ["Netlist error: Could not resolve parameter COUPLEDINDUCTANCE value of \\{B\\} for",
       "device YMIN!K1"],
      ["Netlist error: Parameter COUPLEDINDUCTANCE for device YMIN!K1 contains",
       "unrecognized symbol: B"]
);

$retval = $Tools->runAndCheckGroupedError($CIRFILE."1.cir",$XYCE,@searchstrings);
if ($retval != 0)
{ 
  $exitcode = $retval;
}
print "\n\n";

# these search strings are supposed to occur in the error output
# for the netlist ParameterBlock_extractMIDeviceData2.cir.
@searchstrings = ("Netlist error in file DeviceBlock_extractMIDeviceData2\\.cir at or near line 6",
    "Unrecognized parameter 20 for device LLEAD1",
    "Netlist error in file DeviceBlock_extractMIDeviceData2\\.cir at or near line 6",
    "Unrecognized fields for device LLEAD1"
);

$retval = $Tools->runAndCheckError($CIRFILE."2.cir",$XYCE,@searchstrings);
if ($retval != 0)
{ 
  $exitcode = $retval;
}
print "\n\n";

print "Exit code = $exitcode\n";
exit $exitcode;

