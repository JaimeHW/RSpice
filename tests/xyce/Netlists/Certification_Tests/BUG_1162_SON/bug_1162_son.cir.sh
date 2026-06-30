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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

# This test runs a baseline circuit with a normal, correct .DC statement
# to run a single point.  It then runs three "defective" netlists with
# bad .DC lines that Xyce used to handle in a manner inconsistent with
# how other spice-like simulators handle them --- that is, the other
# simulators just run the first requested point, and Xyce would do something
# unexpected (either not run anything, or run a sweep in the wrong direction).
#
# Xyce also outputs a warning when it encounters these defects (unlike the
# other codes), so we check for that, too.

%searchstringshash = ( "defective_lin.cir"=>("Linear DC or STEP parameters for sweep over V1"),
                       "defective_dec.cir"=>("Decade DC or STEP parameters for sweep over V1"),
                       "defective_oct.cir"=>("Octave DC or STEP parameters for sweep over V1"),
          );
$XYCE="$ARGV[0]";
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

$BASELINE_CIR="baseline.cir";
@cirlist = ("defective_lin.cir","defective_dec.cir","defective_oct.cir");

#Run the baseline
$retval=$Tools->wrapXyce($XYCE,$BASELINE_CIR);
if ($retval != 0)
{
    print STDERR "Xyce failed on $BASELINE_CIR\n";
    print "Exit code = $retval\n"; exit $retval;
}

#Run all defective sweep netlists
foreach $cirname (@cirlist)
{
    $retval = $Tools->runAndCheckWarning($cirname,$XYCE,$searchstringshash{$cirname});
    if ($retval != 0)
    {
        print STDERR "Test failure on $cirname\n";
        print "Exit code = $retval\n"; exit $retval;
    }
    #compare this to baseline
    $CMD="$XYCE_VERIFY $BASELINE_CIR $BASELINE_CIR.prn $cirname.prn > $cirname.prn.out 2>&1 $cirname.prn.err";
    $retval=system($CMD);
    if ($retval != 0)
    {
        print STDERR "$cirname produced output that does not match $BASELINE_CIR.  See $cirname.prn.out and $cirname.prn.err for details.\n";
        print "Exit code = 2\n";
        exit 2;
    }
}

#If we got here, we're good.
print "Exit code = 0\n";
exit 0;
