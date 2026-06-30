#!/usr/bin/env perl

use XyceRegression::Tools;
use MeasureCommon;

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

use File::Basename;
use File::Copy;

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

#
# Steps common to all of the measure tests are in the Perl module
# MeasureCommon.pm.  This file assumes the analysis type was .ac
#
MeasureCommon::checkACFilesExist($XYCE,$CIRFILE);

# The next three blocks of code are used to compare the .MEASURE output
# to stdout to the "gold" stdout in the $GSFILE (TrigTargTestGSfile).  This
# output contains the information for both steps.

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

# Parse the .out file to find the text related to .MEASURE for all steps.
# The measure output in the stdout text for the first step is bracketed by
# lines that contain "Measure Functions" and "Beginning DC Operating Point
# Calculation".  The measure output in the stdout text for the second step
# is bracketed by lines that contain "Measure Functions" and "Solution Summary".
# The code has not been tested for more than two steps, and may not work in that
# case.
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
  elsif ( ($foundStart > 0 && $line =~ /Solution Summary/) ||
       ($foundStart > 0 && $line =~ /Beginning DC Operating Point Calculation/) )
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

# test that the values and strings in the files $CIRFILE.errmsg and $GSFILE, for all steps,
# match to the required tolerances
$GSFILE="TrigTargTestACGSfile";
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;

my $dirname = dirname($XYCE_VERIFY);
my $fc = "$dirname/file_compare.pl";
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

  # The next two blocks of code are used to compare the measured .maX file
  # with the "Gold" .maX file, which is in OutputData/MEASURE_AC/STEP/TrigTargTest.cir.maX
  # Check that the Gold .maX file exists
  $maSuffix=$stepNum-1;
  $ma0String= "ma$maSuffix";
  $GOLDMA0 = $GOLDPRN;
  $GOLDMA0 =~ s/prn$/$ma0String/;
  #print "GOLDMA0 file = $GOLDMA0\n";
  $maSuffix = $stepNum-1;
  if (not -s "$GOLDMA0" )
  {
    print "GOLD $ma0String file does not exist\n";
    print "Exit code = 17\n";
    exit 17;
  }

  # compare gold and measured .ma files, for this step.
  $MEASUREMA0 = "$CIRFILE.ma$maSuffix";
  #print "$fc $MEASUREMA0 $GOLDMA0 $absTol $relTol $zeroTol\n";
  `$fc $MEASUREMA0 $GOLDMA0 $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err`;
  my $retval=$? >> 8;

  if ( $retval != 0 )
  {
    print "test Failed comparison of Gold and measured .ma$maSuffix files!\n";
    print "Exit code = $retval\n";
    exit $retval;
  }
  else
  {
    print "Passed comparison of .ma$maSuffix files\n";
  }
}

print "Exit code = $retval\n";
exit $retval;
