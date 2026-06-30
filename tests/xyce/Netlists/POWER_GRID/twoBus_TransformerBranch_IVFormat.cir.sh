#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# these are the tolerances used in comparing the .prn file
# and the Gold Standard
$absTol = 1.0e-5;
$relTol = 1.0e-4;
$zeroTol= 1e-7;

# Use file_compare because the column ordering in the output .prn file
# may not match the ordering assumed by xyce_verify from the .DC
# statement in the netlist.  The columns in the output .PRN file were
# ordered for easy comparison with Power System Toolbox (PST) results.
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;

# clean-up files from previous runs
`rm -f $CIRFILE.prn $CIRFILE.prn.out $CIRFILE.prn.err`;

# generate .prn file to be tested             
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# check that the gold file exists.  wrapXyce checked for the test .prn file
if (not -s "$GOLDPRN" ) { print STDERR "Missing Gold Standard file: $GOLDPRN\nExit code = 2\n"; exit 2; }

# compare test and gold output
$CMD="$fc $CIRFILE.prn $GOLDPRN $absTol $relTol $zeroTol > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
$retval = system($CMD);
$retval = $retval >> 8;
if ( $retval != 0 )
{
     print STDERR "Comparator exited with exit code $retval\n";
     print "Exit code = 2\n"; 
     exit 2;
}
else            
{
     print "Exit code = 0\n"; 
     exit 0;
}
