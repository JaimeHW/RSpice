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

# change ending on gold standard file from ".prn" as passed in to ".mt0" 
# which is the file storing the measure results.
$GOLDPRN =~ s/prn$/mt0/;

# remove files from previous runs
system("rm -f $CIRFILE.mt0 $CIRFILE.prn $CIRFILE.out $CIRFILE.err*");

# phase is output in degrees and for fourier components with very small magnitude. It
# can have several degrees of scatter. Thus it gets its own phaseabstol and phasereltol.
$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/compare_fourier_files/;

$absTol = 5e-3;
$relTol = 0.01;
$phaseAbsTol = 1.0;
$phaseRelTol = 0.01;
$zeroTol = 1.0e-8;

#
# Steps common to all of the measure tests are in the Perl module
# MeasureCommon.pm.  This file assumes the analysis type was .TRAN.
#
MeasureCommon::checkTranFilesExist($XYCE,$CIRFILE);

# The next three blocks of code are used to compare the .MEASURE output
# to stdout to the "gold" stdout in the $GSFILE (FourierTestFromToTDGSfile).

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
  if ($line =~ /Measure Functions/) { $foundStart = 1; }
  if ($foundStart > 0 && $line =~ /Total Simulation/) { $foundEnd = 1; }

  if ($foundStart > 0 && $foundEnd < 1)
  {
    print ERRMSG $line;
  }
}

close(NETLIST);
close(ERRMSG);

# test that the values and strings in the .out file match to the required tolerances
my $GSFILE="FourierTestFromToTDGSfile";

$CMD="$fc $CIRFILE.errmsg $GSFILE $absTol $relTol $phaseAbsTol $phaseRelTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
$retval=system($CMD);
$retval= $retval >> 8;

if ( $retval != 0 )
{
  print STDERR "test failed comparison of stdout vs. GSfile\n";
  print "Exit code = 2\n";
  exit 2;
}

# The next two blocks of code are used to compare the measured .mt0 file
# with the "Gold" .mt0 file, which is in OutputData/MEASURE
# Check that the Gold .mt0 file exists
if (not -s "$GOLDPRN" )
{
  print "GOLD .mt0 file does not exist\n";
  print "Exit code = 17\n";
  exit 17;
}

# compare gold and measured .mt0 files
$CMD="$fc $CIRFILE.mt0 $GOLDPRN $absTol $relTol $phaseAbsTol $phaseRelTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
$retval=system($CMD);
$retval= $retval >> 8;
if ( $retval != 0 )
{
  print STDERR "test failed comparison of Gold and measured .mt0 files\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Passed comparison of .mt0 files\n";
}

# check number format in .mt0 file
$retval = MeasureCommon::checkNumberFormatinFourOutput("$CIRFILE.mt0",7);
if ( $retval != 0 )
{
  print STDERR "Failed number format and precision test in .mt0 file\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Passed number format and precision test for .mt0 file\n";
}

# check number format in stdout
$retval = MeasureCommon::checkNumberFormatinFourOutput("$CIRFILE.errmsg",7);
if ( $retval != 0 )
{
  print STDERR "Failed number format and precision test for stdout\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Passed number format and precision test for stdout\n";
}

# Test re-measure
$retval = MeasureCommon::checkRemeasureFour($XYCE,$XYCE_VERIFY,$CIRFILE,$absTol,$relTol,$phaseAbsTol,$phaseRelTol,$zeroTol,'prn');
if ($retval != 0)
{
  print "Failed remeasure\n";
}

print "Exit code = $retval\n";
exit $retval;


