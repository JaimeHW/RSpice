#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce the appropriate output files, then we return exit code 14.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.raw.out and the STDERR output from
# comparison to go into $CIRFILE.raw.err.

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# this string should be in the output of this failed Xyce run
@searchstrings = (["Netlist error: Number of ports in file invalidNumFreqLine.s2p for model",
     "YLIN_MOD6 must be an integer > 0",
     "Netlist error: Invalid \\[Two-Port Data Order\\] line in file",
     "invalidNumFreqLine.s2p for model YLIN_MOD6 at line 6",
     "Netlist error: Invalid \\[Number of Frequencies\\] line in file",
     "invalidNumFreqLine.s2p for model YLIN_MOD6 at line 7",
     "Netlist error: Invalid \\[Reference\\] line in file invalidNumFreqLine.s2p for",
     "model YLIN_MOD6 at line 8"],
    ["Netlist error: Invalid string 2_21 in file invalidLines.s2pfor \\[Two-Port Data",
     "Order\\] for model YLIN_MOD7",
     "Netlist error: Number of frequencies in file invalidLines.s2p for model",
     "YLIN_MOD7 must be an integer > 0",
     "Netlist error: \\[Reference\\] line in file invalidLines.s2p for model YLIN_MOD7",
     "lacks enough entries",
     "Netlist error: Unable to parse \\[Network Data Section\\] in file invalidLines.s2p",
     "for model YLIN_MOD7.  Number of ports or frequencies < 1"],
    ["Netlist error: Number of lines in \\[Network Data\\] does not match \\[Number of",
     "Frequencies\\] in file tooFewNetworkDataLines.s2p for model YLIN_MOD8",
     "Netlist error: Incorrect number of entries in \\[Network Data\\] block found in",
     "file tooFewNetworkDataLines.s2p for model YLIN_MOD8. Found 90. Expected 99"],
     ["Netlist error: Number of lines in \\[Network Data\\] does not match \\[Number of",
     "Frequencies\\] in file tooManyNetworkDataLines.s2p for model YLIN_MOD9",
     "Netlist error: Incorrect number of entries in \\[Network Data\\] block found in",
     "file tooManyNetworkDataLines.s2p for model YLIN_MOD9. Found 99. Expected 90"],
    ["Netlist error: No valid \\[Network Data\\] block of lines found in file",
     "shortNetworkDataLine.s1p for model YLIN_MOD10",
     "Netlist error: Incorrect number of entries in \\[Network Data\\] block found in",
     "file shortNetworkDataLine.s1p for model YLIN_MOD10. Found 32. Expected 33"]
);
$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);

print "Exit code = $retval\n"; exit $retval;


