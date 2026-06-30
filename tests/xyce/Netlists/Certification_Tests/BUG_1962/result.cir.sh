#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# remove previous output files
system("rm -f $CIRFILE.out $CIRFILE.err $CIRFILE.prn $CIRFILE.res*");

# run Xyce
$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# was the .res file made?
if (not -s "$CIRFILE.res" ) 
{ 
  print ".res file not found\n";
  print "Exit code = 2\n"; exit 2; 
}

# now compare the gold and test .res files
$GOLDRES=$GOLDPRN;
$GOLDRES =~ s/prn/res/;

$fc = $XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;
$abstol=1e-6;
$reltol=1e-4;
$zerotol=1e-6;

$retval = 0;
$CMD="$fc $CIRFILE.res $GOLDRES $abstol $reltol $zerotol > $CIRFILE.res.out 2> $CIRFILE.res.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIRFILE.res, see $CIRFILE.res.err\n";
    $retval = 2;
}

print "Exit code = $retval\n"; exit $retval;



