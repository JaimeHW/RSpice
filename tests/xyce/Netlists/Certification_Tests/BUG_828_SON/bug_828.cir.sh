#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script  (unused here)
# $ARGV[2] = location of compare script         (never used)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file  (unused here)

# If Xyce does not produce a prn file, then we return exit code 10.

$XYCE=$ARGV[0];
$CIRFILE=$ARGV[3];

# This test simply makes sure Xyce doesn't fail on the main netlist, as
# it did in all released versions of Xyce prior to 6.6.
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);

if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

print "Exit code = $retval\n"; 
exit $retval; 

