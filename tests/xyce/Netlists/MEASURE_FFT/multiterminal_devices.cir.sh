#!/usr/bin/env perl

use MeasureCommon;

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

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;

# remove files from previous runs
system("rm -f $CIRFILE.mt0 $CIRFILE.fft0 $CIRFILE.out $CIRFILE.err*");
system("rm -f $CIRFILE.remeasure* $CIRFILE.measure*");

#
# Steps common to all of the measure tests are in the Perl module
# MeasureCommon.pm.  This file assumes the analysis type was .TRAN
#
MeasureCommon::checkFFTFilesExist($XYCE,$CIRFILE);

# test tolerances
my $absTol=5e-4;
my $relTol=1e-3;
my $zeroTol=1e-10;

# The next two blocks of code are used to compare the measured .mt0 file
# with the "Gold" .mt0 file, which is in OutputData/MEASURE_FFT/
# Check that the Gold .mt0 file exists
$GOLDMT0 = $GOLDPRN;
$GOLDMT0 =~ s/prn$/mt0/;
#print "GOLDMT0 file = $GOLDMT0\n";
if (not -s "$GOLDMT0" )
{
  print "GOLD .mt0 file does not exist\n";
  print "Exit code = 17\n";
  exit 17;
}

# compare gold and measured .mt0 files
$MEASUREMT0 = "$CIRFILE.mt0";
$CMD="$fc $MEASUREMT0 $GOLDMT0 $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
$retval=system($CMD);
$retval = $retval >> 8;

if ( $retval != 0 )
{
  print STDERR "test failed comparison of Gold and measured .mt0 files with exit code $retval\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Passed comparison of .mt0 files\n";
}

# Re-measure test uses the same approach as the FOUR measure.
$retval = MeasureCommon::checkRemeasureWithFFTFiles($XYCE,$XYCE_VERIFY,$CIRFILE,$absTol,$relTol,$zeroTol);

print "Exit code = $retval\n";
exit $retval;
