#!/usr/bin/env perl

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
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$fc = $XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;

# comparison tolerances for file_compare.pl
$abstol=1e-6;
$reltol=1e-3;  #0.1%
$zerotol=1e-8;

`rm -f $CIRFILE.prn $CIRFILE.out $CIRFILE.err`;
`rm -f $CIRFILE.prn.out $CIRFILE.prn.err`;

# We're using wrapXyce for its error handling
use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0)
{
    print "Exit code = $retval\n";
    exit $retval;
}
if (not -s "$CIRFILE.prn" )
{
    print "Exit code = 14\n";
    exit 14;
}

$retcode=0;
$CMD="$fc $GOLDPRN $CIRFILE.prn $abstol $reltol $zerotol > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval!= 0){
  print STDERR "Comparator exited on file $CIRFILE.prn with exit code $retval\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
