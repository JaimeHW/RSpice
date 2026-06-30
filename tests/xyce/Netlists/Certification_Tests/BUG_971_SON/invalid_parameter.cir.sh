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

# this string should be in the output of this failed Xyce run.
@searchstrings = ("Netlist error: There were 8 undefined symbols in .PRINT command: C1:BLEEM",
                  "I1:BLEEM, L1:BLEEM, R1:BLEEM, X1:C1:BLEEM, X1:I1:BLEEM, X1:L1:BLEEM",
                  "X1:R1:BLEEM");

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval;
