#!/usr/bin/env perl

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This simple test does nothing more than make sure Xyce does not exit with
# error on the input circuit.
#
# Prior to commit 3ba1476, Xyce would exit with a "Directory node not found"
# due to a bug in the expression library.
#

$XYCE=$ARGV[0];
$CIR1=$ARGV[3];

$CMD="$XYCE $CIR1 > $CIR1.out 2> $CIR1.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10 }

print "Exit code = 0\n";
exit 0


