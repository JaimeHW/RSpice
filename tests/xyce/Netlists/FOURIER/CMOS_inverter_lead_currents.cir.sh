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

# remove the .prn at the end
$GOLDPRN =~ s/\.prn$//;

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

#
# Did we make a Fourier file
#
if (not -s "$CIRFILE.four0" )
{
  print "Missing output file $CIRFILE.four0\n"; 
  print "Exit code = 2\n"; exit 2;
}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl

if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE junk $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
    {
        print "Exit code = 2 \n";
        exit 2;
    }
    else
    {
        print "Exit code = 0 \n";
        exit 0;
    }
}

# verify .FOUR output
$retcode=0;

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/compare_fourier_files/;

# Phase is output in degrees and for Fourier components with very small magnitude. It 
# can have several degrees of scatter.  Thus it gets its own phaseabstol and phasereltol.
$abstol = 2.0e-6;
$reltol = 0.02;
$phaseabstol = 1.0;
$phasereltol=0.02;
$zerotol = 1.0e-8;

$CMD="$fc $CIRFILE.four0 $GOLDPRN.four0 $abstol $reltol $phaseabstol $phasereltol $zerotol > $CIRFILE.four.out 2> $CIRFILE.four.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIRFILE.four0, see $CIRFILE.four0.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; 
exit $retcode;
