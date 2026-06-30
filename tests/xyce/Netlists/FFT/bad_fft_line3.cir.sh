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
# Note that underscore (_) characters must be escaped with \\
@searchstrings = ("Netlist error: Invalid FREQ, FMIN or FMAX value on .FFT line for V\\(1\\)",
    "Netlist error: Invalid FREQ, FMIN or FMAX value on .FFT line for V\\(2\\)",
    "Netlist error: Invalid FREQ, FMIN or FMAX value on .FFT line for V\\(3\\)",
    "Netlist error: Invalid FREQ, FMIN or FMAX value on .FFT line for V\\(4\\)",
    "Netlist error: Invalid FREQ, FMIN or FMAX value on .FFT line for V\\(5\\)",
    "Netlist error: Invalid FREQ, FMIN or FMAX value on .FFT line for V\\(6\\)"
);

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval;
