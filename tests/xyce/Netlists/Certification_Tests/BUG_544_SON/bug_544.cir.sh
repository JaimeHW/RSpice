#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This script runs a netlist that uses int, floor, and ceil in an expression
# and checks the output against perl's posix int, floor, and ceil functions.
#

$XYCE=$ARGV[0];
$CIRFILE=$ARGV[3];

system("rm -f $CIRFILE.prn $CIRFILE.mt0");

# We're using wrapXyce for its error handling
use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0)
{
    print "Exit code = $retval\n";
    exit $retval;
}

# Declare success if both of the output files are made.
# Don't bother verifying their contents against a gold standard.
# xyce_verify already checked for the <netlistName>.prn file.
$xyce_exit = 0;

if (not -s "$CIRFILE.mt0" )
{
    print STDERR "Missing output file $CIRFILE.mt0\n";
    $xyce_exit = 2;
}

print "Exit code = $xyce_exit\n";
exit $xyce_exit;
