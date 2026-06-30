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
#$GOLDPRN=$ARGV[4];

$CIR[0]="issue_221_measfail_zero.cir";
$CIR[1]="issue_221_measfail_one.cir";

# remove files from previous runs
system("rm -f $CIRFILE.mt0 $CIRFILE.out $CIRFILE.err");
foreach $i (0 .. 1)
{
  system("rm -f $CIR[$i].mt0 $CIR[$i].out $CIR[$i].err");
}

#
# Steps common to all of the measure tests are in the Perl module
# MeasureCommon.pm.  This file assumes the analysis type was .tran
#
MeasureCommon::checkTranFilesExist($XYCE,$CIRFILE);
MeasureCommon::checkTranFilesExist($XYCE,$CIR[0]);
MeasureCommon::checkTranFilesExist($XYCE,$CIR[1]);

$exitcode=0;

# each netlist should get the same answers in their .mt0 files
foreach $i (0 .. 1)
{
  $CMD="diff $CIRFILE.mt0 $CIR[$i].mt0 > $CIR[$i].mt0.out 2> $CIR[$i].mt0.err";
  $retval = system($CMD);
  $retval = $retval >> 8;

  # check the return value
  if ( $retval != 0 )
  {
    print "Diff Failed for file $CIR[$i].mt0. See $CIR[$i].mt0.out and $CIR[$i].mt0.err\n";
    $exitcode = 2;
  }
}

print "Exit code = $exitcode\n";
exit $exitcode;
