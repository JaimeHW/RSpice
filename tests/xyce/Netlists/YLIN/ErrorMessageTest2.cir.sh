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

# these strings should be in the output of this failed Xyce run
@searchstrings = (["Netlist error: No valid \\[Version\\] or Option line in file noOptionLine-ts1.s1p",
     "for model YLIN_MOD1 at line 4"],
    ["Netlist error: Invalid Touchstone 1 format, or possible missing \\[Version\\] line",
     "in Touchstone 2 format, in file noVersionLine.s1p for model YLIN_MOD2 at line"],
    ["Netlist error: Unable to parse network data section in file",
     "noNetworkData-ts1.s1p for model YLIN_MOD3.  Number of frequencies < 1"],
    ["Netlist error: Invalid network data line in file unequalLines1-ts1.s1p for",
     "model YLIN_MOD4 at line 7"],
    ["Netlist error: Invalid network data line in file unequalLines2-ts1.s1p for",
     "model YLIN_MOD5 at line 22"],
    ["Netlist error: Error determining number of ports from file",
     "shortLine-ts1.s1pfor model YLIN_MOD6"]
);
$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);

print "Exit code = $retval\n"; exit $retval;


