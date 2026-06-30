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

# these are the tolerances used in comparing the measures calculated by this 
# script to those calculated by Xyce.  Zerotol is needed because some measured
# values should be zero (and reltol won't make sense in that case).

# abtTol is high here because we are measuring values on the order of 1000
my $absTol = 0.2;
my $relTol = 0.02;
my $zeroTol = 1.0e-5;

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
# parse the keywords in the .measure statements in the .cir file, 
# now that we know when the simulation ended.  This section of code may 
# may be different for each test
my @fromVals = MeasureCommon::parseKeyWord($CIRFILE,"FROM",$endTime);
my @toVals = MeasureCommon::parseKeyWord($CIRFILE,"TO",$endTime);
my @OffVals = MeasureCommon::parseKeyWord($CIRFILE,"OFF",$endTime);
my @OnVals = MeasureCommon::parseKeyWord($CIRFILE,"ON",$endTime);
my @defaultVals = MeasureCommon::parseKeyWord($CIRFILE,"DEFAULT_VAL",$endTime);

# now calculate the values for these measures.  This is the part of 
# the test script that will be different for each one. 
# 
my @measureIsOn;
my @measureIsOff;  
my @numberOnCycles;
my @numberOffCycles;
my @lastTimeValue;
my @totalAveragingWindow;

for $j (0 ..$numMeasures-1) 
{
  if ($tdVals[$j] > $toVals[$j])
  {
    $calcVals[$j] = $defaultVals[$j];
  }
  else
  {
    $measureIsOn[$j] = 0;
    $measureIsOff[$j] = 1;  # signals start off "off" so don't count that as an off cycle.
    $numberOnCycles[$j] = 0;
    $numberOffCycles[$j] = 0;
    $lastTimeValue[$j] = 0;
    $totalAveragingWindow[$j]=0;
    $firstVal = 0;

    # find the correct column in the output data, collected from the .prn file
    $colIdx = MeasureCommon::findColumnInPrnFile($measureQuants[$j],\@headerVarNames);
  
    for $i (1 .. $#dataFromXyce )
    {
      if ( ($dataFromXyce[$i][0]>=$fromVals[$j]) && ($dataFromXyce[$i][0]<=$toVals[$j]) )   
      {
        # count On and Of cycles for better averaging 
        if($dataFromXyce[$i][$colIdx] > $OnVals[$j] ) 
        {
          if( $measureIsOn[$j] == 0 )
          {
            $measureIsOn[$j]=1;
            $numberOnCycles[$j]++;
          }
        } 
        else
        {
          $measureIsOn[$j]=0;
        }
    
        if($dataFromXyce[$i][$colIdx] < $OffVals[$j] ) 
        {
          if( $measureIsOff[$j] == 0 )
          {
            $measureIsOff[$j]=1;
            $numberOffCycles[$j]++;
          }
        } 
        else
        {
          $measureIsOff[$j]=0;
        }
        if ($firstVal > 0) {$totalAveragingWindow[$j] += ( $dataFromXyce[$i][0] - $lastTimeValue[$j]);}
        $firstVal = 1;
        $lastTimeValue[$j] = $dataFromXyce[$i][0]
      }
    }
  }
  if ($totalAveragingWindow[$j] > 0)
  {
    $calcVals[$j] = 0.5*($numberOnCycles[$j] + $numberOffCycles[$j]) / $totalAveragingWindow[$j];
  }
  else
  {
    $calcVals[$j] = $defaultVals[$j];
  }
}

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


