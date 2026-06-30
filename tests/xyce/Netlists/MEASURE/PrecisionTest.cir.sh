#!/usr/bin/env perl

use XyceRegression::Tools;
use MeasureCommon;
use Scalar::Util qw(looks_like_number);

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
#$GOLDPRN=$ARGV[4];

#
# Steps common to all of the measure tests are in the Perl module
# MeasureCommon.pm.  This file assumes the analysis type was .tran
#
MeasureCommon::checkTranFilesExist($XYCE,$CIRFILE);

# hard code this
my $numMeasures = 13;

# Process .mt0 file to get the measure names and measured values
# for each measure statement. Then deference to get the underlying arrays.
# This makes the subsequent code more readable since the variable are
# arrays or scalars, and aren't mixed in with array references.
my ($measureNamesRef,$measureValsRef)
          = MeasureCommon::parseMeasureNamesValues($CIRFILE,$numMeasures);
my @measureNames = @$measureNamesRef;
my @measureVals = @$measureValsRef;

# hard code this value rather than parsing it from the .cir file
my $requestedPrecision=10;

# check the precision in the .mt0 file.
my $retval=0;
my $foundDot;
my $idx;
my $token;

foreach $j (0 .. $numMeasures-1)
{
  $digitCount = MeasureCommon::getPrecision($measureVals[$j]);
  if ($digitCount != $requestedPrecision)
  {
    # using the value set above in the $requestedPrecision variable.
    print "Failed requested precision test for measure $measureNames[$j]\n";
    print "Requested and measured precision = ($requestedPrecision,$digitCount)\n";
    print "Exit code = 2\n";
    exit 2;
  }
  else
  {
    print "Passed precision test for measure $measureNames[$j]\n";
    print "MEASDGT and measured precision = ($requestedPrecision,$digitCount)\n";
  }
}

# Now check the precision in the stdout.
# Check that .out file exists, and open it if it does.
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

# parse the .out file to find the text related to .MEASURE.
my $foundStart=0;
my $foundEnd=0;
my @outLine;
my $lineCount=0;
while( $line=<NETLIST> )
{
  if ($line =~ /Measure Functions/) { $foundStart = 1; }
  if ($foundStart > 0 && $line =~ /Total Simulation/) { $foundEnd = 1; }

  if ( ($foundStart > 0 && $foundEnd < 1) ||
       ($foundStart < 1 && ($line =~ /Netlist warning/)) )
  {
    print ERRMSG $line;
  }
}

close(NETLIST);
close(ERRMSG);

# check that the numbers in the .out file have the correct precision
if (-s "$CIRFILE.errmsg" )
{
  open(TESTFILE,"$CIRFILE.errmsg");
  $lineCount=0;

  while( $lineTestFile=<TESTFILE> )
  {
    $lineCount++;
    # process a line into text and values.
    chop $lineTestFile;

    # Remove leading spaces on line, otherwise the spaces become
    # element 0 of "testFileData" instead of the first column of data.
    $lineTestFile =~ s/^\s*//;
    @testFileData = (split(/[\s,]+/, $lineTestFile));
    for( $i=0; $i<=$#testFileData; $i++ )
    {
      if ( looks_like_number($testFileData[$i]) )
      {
        $digitCount = MeasureCommon::getPrecision($testFileData[$i]);
        if ($digitCount != $requestedPrecision)
        {
          # using requested value, which was set above in the $requestedPrecision variable.
          print "Failed requested precision test in stdout for value $testFileData[$i]\n";
          print "Requested and measured precision = ($requestedPrecision,$digitCount)\n";
          print "Exit code = 2\n";
          exit 2;
        }
      }
    }
  }
}

print "Exit code = $retval\n";
exit $retval;
