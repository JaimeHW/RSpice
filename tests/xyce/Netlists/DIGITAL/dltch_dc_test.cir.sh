#!/usr/bin/env perl

use XyceRegression::Tools;
use Scalar::Util qw(looks_like_number);

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
$absTol = 1.0e-8;
$relTol = 0.001;
$zeroTol = 1e-12;

# generate .prn file to be tested             
$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

# compare the test and gold file
$retcode=0;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
#print "$fc $CIRFILE.prn $GOLDPRN $absTol $relTol $zeroTol\n";
$CMD="$fc $CIRFILE.prn $GOLDPRN $absTol $relTol $zeroTol > $CIRFILE.prn.out 2> $CIRFILE.prn.err";

if (system($CMD) != 0) {
     print STDERR "Verification failed on file $CIRFILE.csd, see $CIRFILE.csd.err\n";
     $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;

