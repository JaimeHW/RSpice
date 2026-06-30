#!/usr/bin/env perl
# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# run the netlist in top_level, so that the execution directory
# is one level up from the directory of the "top-level netlist".
$CIR1="top_level/$CIRFILE";

# The gold .PRN file is top_level_file_path.cir.prn
$retval=$Tools->wrapXyceAndVerify($XYCE,$XYCE_VERIFY,$GOLDPRN,$CIR1);

print "Exit code = $retval\n";
exit $retval;
