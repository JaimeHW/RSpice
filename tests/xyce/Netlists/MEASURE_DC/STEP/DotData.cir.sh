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
system("rm -f $CIRFILE.ms0 $CIRFILE.out $CIRFILE.err* $CIRFILE.remeasure*");

#
# Steps common to all of the measure tests are in the Perl module
# MeasureCommon.pm.  This file assumes the analysis type was .DC
#
MeasureCommon::checkDCFilesExist($XYCE,$CIRFILE);

# The next three blocks of code are used to compare the .MEASURE output
# to stdout to the "gold" stdout in the $GSFILE (DotDataGSfile).

# check that .out file exists, and open it if it does
if (not -s "$CIRFILE.out" )
{
  print "Exit code = 17\n";
  exit 17;
}
else
{
  open(NETLIST, "$CIRFILE.out");
  open(ERRMSG,">$CIRFILE.errmsg") or die $!;
}

# parse the .out file to find the text related to .MEASURE
my $foundStart=0;
my $foundEnd=0;
my @outLine;
my $lineCount=0;
while( $line=<NETLIST> )
{
  if ($line =~ /Measure Functions/)
  {
    $foundStart = 1;
    $foundEnd = 0;
  }

  if ($foundStart > 0 && $line =~ /LASTMEASURE/ )
  {
    print ERRMSG $line;
    #print "Found last measure line\n";
    $foundEnd = 1;
    $foundStart = 0;
  }
  elsif ( ($foundStart > 0) && ($line =~ /Solution Summary/) )
  {
    $foundEnd = 1;
  }

  if ($foundStart > 0 && $foundEnd < 1)
  {
    print ERRMSG $line;
  }
}

close(NETLIST);
close(ERRMSG);

# test that the values and strings in the .out file match to the required
# tolerances
my $GSFILE="DotDataGSfile";
my $absTol=1e-5;
my $relTol=1e-3;
my $zeroTol=1e-10;

#print "$fc $CIRFILE.errmsg $GSFILE $absTol $relTol $zeroTol\n";
`$fc $CIRFILE.errmsg $GSFILE $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err`;
my $retval=$? >> 8;

if ( $retval != 0 )
{
  print "test Failed comparison of stdout vs. GSfile!\n";
  print "Exit code = $retval\n";
  exit $retval;
}
else
{
  print "Passed comparison of stdout info\n";
}

###################################################
# process each step.  Number of steps is hard-coded
###################################################
$numSteps=2;
foreach $stepNum (1 .. $numSteps)
{
  print "Processing data for step $stepNum\n";

  # The next two blocks of code are used to compare the measured .msX file
  # with the "Gold" .msX file, which is in OutputData/MEASURE_DC/STEP/
  # Check that the Gold .msX file exists
  $msSuffix=$stepNum-1;
  $msxString= "ms$msSuffix";
  $GOLDMSX = $GOLDPRN;
  $GOLDMSX =~ s/prn$/$msxString/;
  #print "GOLDMSX file = $GOLDMSX\n";
  if (not -s "$GOLDMSX" )
  {
    print "GOLD $msxstring file does not exist\n";
    print "Exit code = 17\n";
   exit 17;
  }

  # compare gold and measured .ms0 files
  $MEASUREMSX = "$CIRFILE.ms$msSuffix";
  #print "$fc $MEASUREMS0 $GOLDMS0 $absTol $relTol $zeroTol\n";
  `$fc $MEASUREMSX $GOLDMSX $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err`;
  my $retval=$? >> 8;

  if ( $retval != 0 )
  {
    print "test Failed comparison of Gold and measured .ms$msSuffix file!\n";
    print "Exit code = $retval\n";
    exit $retval;
  }
  else
  {
    print "Passed comparison of .ms$msSuffix files\n";
  }
}

# Also test remeasure if the basic measure function works
$retval = MeasureCommon::checkRemeasure($XYCE,$XYCE_VERIFY,$CIRFILE,$absTol,$relTol,$zeroTol,'prn',$numSteps,'ms');

print "Exit code = $retval\n";
exit $retval;
