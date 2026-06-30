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

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

#
# Steps common to all of the measure tests are in the Perl module
# MeasureCommon.pm.  This file assumes the analysis type was .tran
#
MeasureCommon::checkTranFilesExist($XYCE,$CIRFILE);

# Parse the .cir to get the number of measures. Then deference to get the
# underlying array. This makes the subsequent code more readable since the
# variable are arrays or scalars, and aren't mixed in with array references. 
my ($numMeasures,$measuredQuantsRef) =  MeasureCommon::getNumMeasuresInCirFile($CIRFILE);
my @measureQuants = @$measuredQuantsRef;

# Process .mto file to get the measure names and measured values 
# for each measure statement. Then deference to get the underlying arrays.
# This makes the subsequent code more readable since the variable are 
# arrays or scalars, and aren't mixed in with array references. 
my ($measureNamesRef,$measureValsRef) 
          = MeasureCommon::parseMeasureNamesValues($CIRFILE,$numMeasures);
my @measureNames = @$measureNamesRef;
my @measureVals = @$measureValsRef;

# parse out the data from the .prn file.
my ($headVarNamesRef,$dataFromXyceRef) = MeasureCommon::parseTranPrnFile($CIRFILE);
my @headerVarNames = @$headVarNamesRef;
my @dataFromXyce = @$dataFromXyceRef;
my $endTime = $dataFromXyce[$#dataFromXyce][0];

#
# The next two code blocks may differ for each test
#
my @defaultVals = MeasureCommon::parseKeyWord($CIRFILE,"DEFAULT_VAL",$endTime);

# for these keywords, whether they were given matters later in the .sh file
my ($atValsPtr,$atGivenPtr) =  MeasureCommon::parseKeyWord($CIRFILE,"AT",$endTime);
my @atVals = @$atValsPtr;
my @atGiven = @$atGivenPtr;

#set minVal to default value of 1e-12
$minVal=1e-12;

#
# now calculate the values for this measures.  This is the part of 
# the test script may also different for each test. 
# 
# Find when the waveform crosses the user specified level (WHEN qualifier) for the possible
# variants of the RISE, FALL and CROSS qualifiers.  
my @calcVals;
my $colIdx;
my $backDiff;
my $forwardDiff;

# there is only one measure in this test, and it uses AT
$colIdx = 1;

for $i (1 .. $#dataFromXyce )
{  
  $backDiff = $dataFromXyce[$i-1][0] - $atVals[$j];
  $forwardDiff = $dataFromXyce[$i][0] - $atVals[$j];
  if( (($backDiff <= 0.0) && ($forwardDiff > 0.0)) || (($backDiff > 0.0) && ($forwardDiff <= 0.0)) ||
      (abs($backDiff) < $minVal) || (abs($forwardDiff) < $minVal) )
  {
    $calcVals[$j] = ($dataFromXyce[$i][$colIdx] - $dataFromXyce[$i-1][$colIdx])/
                    ($dataFromXyce[$i][0] - $dataFromXyce[$i-1][0]);
    #print "For $measureNames[$j], current/previous values and times, atVal and back/fwd diffs are:\n";
    #print "$dataFromXyce[$i][$colIdx], $dataFromXyce[$i-1][$colIdx], $dataFromXyce[$i][0], $dataFromXyce[$i-1][0]\n";
    #print "$atVals[$j], $backDiff, $forwardDiff\n";
    last;
  }
}

# these are the tolerances used in comparing the measures calculated by this 
# script to those calculated by Xyce.  Zerotol is needed because some measured
# values should be zero (and reltol won't make sense in that case).
my $absTol = 3.0e-3;
my $relTol = 0.02;
my $zeroTol = 1.0e-5;

#
# Print the test results based on relative and absolute error.  This is common to all
# tests.  This subroutine returns a value rather than exiting from the script. 
# 
my $retval = MeasureCommon::printResults($numMeasures,$absTol,$relTol,$zeroTol,\@measureNames,
                                   \@measureVals,\@calcVals);

# Also test remeasure if the basic measure function works
if ($retval != 0)
{ 
  print "Exit code = $retval\n";
  exit $retval;
}
else
{ 
  # Re-measure test uses the same approach as the FOUR measure
  $retval = MeasureCommon::checkRemeasure($XYCE,$XYCE_VERIFY,$CIRFILE,$absTol,$relTol,$zeroTol);
}

print "Exit code = $retval\n";
exit $retval;

