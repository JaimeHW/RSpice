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

# test that the values and strings in the .mtX files match to the required
# tolerances
my $absTol=1e-5;
my $relTol=1e-3;
my $zeroTol=1e-10;

# remove files from previous runs
system("rm -f $CIRFILE.mt* $CIRFILE.out $CIRFILE.err* $CIRFILE.remeasure*");

#
# Steps common to all of the measure tests are in the Perl module
# MeasureCommon.pm.  This file assumes the analysis type was .TRAN
#
MeasureCommon::checkTranFilesExist($XYCE,$CIRFILE);

################################################### 
# process each step.  Number of steps is hard-coded 
################################################### 
$numSteps=2; 
foreach $stepNum (1 .. $numSteps) 
{   
  print "Processing data for step $stepNum\n";

  # The next two blocks of code are used to compare the measured .mtX file
  # with the "Gold" .mtX file, which is in OutputData/MEASURE/REMEASURE/
  # Check that the Gold .mtX file exists
  $mtSuffix=$stepNum-1;   
  $mtxString= "mt$mtSuffix";
  $GOLDMTX = $GOLDPRN;
  $GOLDMTX =~ s/prn$/$mtxString/;
   if (not -s "$GOLDMTX" ) 
  { 
    print "GOLD $mtxstring file does not exist\n";
    print "Exit code = 17\n"; 
   exit 17; 
  }

  # compare gold and measured .mt0 files
  $MEASUREMTX = "$CIRFILE.mt$mtSuffix";
  `$fc $MEASUREMTX $GOLDMTX $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err`;
  my $retval=$? >> 8;

  if ( $retval != 0 )
  {
    print STDERR "test failed comparison of Gold and measured .mt$mtSuffix file with exit code $retval\n";
    print "Exit code = 2\n";
    exit 2;
  }
  else
  {
    print "Passed comparison of .mt$mtSuffix files\n";
  }
}

# Also test remeasure if the basic measure function works
$retval = MeasureCommon::checkRemeasure($XYCE,$XYCE_VERIFY,$CIRFILE,$absTol,$relTol,$zeroTol,'prn',$numSteps,'mt');

print "Exit code = $retval\n";
exit $retval;

