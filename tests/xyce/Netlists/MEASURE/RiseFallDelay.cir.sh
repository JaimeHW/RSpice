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
# parse the keywords in the .measure statements in the .cir file, 
# now that we know when the simulation ended.  This section of code may 
# may be different for each test
my @fromVals = MeasureCommon::parseKeyWord($CIRFILE,"FROM",$endTime);
my @toVals = MeasureCommon::parseKeyWord($CIRFILE,"TO",$endTime);
my @tdVals = MeasureCommon::parseKeyWord($CIRFILE,"TD",$endTime);
my @defaultVals = MeasureCommon::parseKeyWord($CIRFILE,"DEFAULT_VAL",$endTime);

# if frac_max was not specified then trigVals is the absolute trigger level
# if frac_max was specified then trigVals is the frac_max level.  It will be
# multiplied by $maxOfVars to find the absolute trigger level
my ($trigValsPtr,$trigFracMaxFoundPtr) =  MeasureCommon::parseKeyWord($CIRFILE,"TRIG",$endTime);
my @trigVals = @$trigValsPtr;
my @trigFracMaxFound = @$trigFracMaxFoundPtr;

my ($targValsPtr,$targFracMaxFoundPtr) =  MeasureCommon::parseKeyWord($CIRFILE,"TARG",$endTime);
my @targVals = @$targValsPtr;
my @targFracMaxFound = @$targFracMaxFoundPtr;

my ($minValsPtr,$minValsFoundPtr) =  MeasureCommon::parseKeyWord($CIRFILE,"MINVAL",$endTime);
my @minVals = @$minValsPtr;
my @minValsFound = @$minValsFoundPtr;
foreach $j (0 .. $numMeasures-1){
  if ($minValsFound[$j] < 1) {$minVals[$j] = 1e-12};
}

# find the RISE/FALL/CROSS levels for TRIG and TARG
my ($trigRiseValsPtr,$trigRiseGivenPtr,$targRiseValsPtr,$targRiseGivenPtr) =  
                                         MeasureCommon::parseRFCforTrigTarg($CIRFILE,"RISE");
my @trigRiseVals = @$trigRiseValsPtr;
my @trigRiseGiven = @$trigRiseGivenPtr;
my @targRiseVals = @$targRiseValsPtr;
my @targRiseGiven = @$targRiseGivenPtr;

my ($trigFallValsPtr,$trigFallGivenPtr,$targFallValsPtr,$targFallGivenPtr) =  
                                         MeasureCommon::parseRFCforTrigTarg($CIRFILE,"FALL");
my @trigFallVals = @$trigFallValsPtr;
my @trigFallGiven = @$trigFallGivenPtr;
my @targFallVals = @$targFallValsPtr;
my @targFallGiven = @$targFallGivenPtr;

my ($trigCrossValsPtr,$trigCrossGivenPtr,$targCrossValsPtr,$targCrossGivenPtr) =  
                                         MeasureCommon::parseRFCforTrigTarg($CIRFILE,"CROSS");
my @trigCrossVals = @$trigCrossValsPtr;
my @trigCrossGiven = @$trigCrossGivenPtr;
my @targCrossVals = @$targCrossValsPtr;
my @targCrossGiven = @$targCrossGivenPtr;

#
# now calculate the values for this measures.  This is the part of 
# the test script may also different for each test. 
# 
my @calcVals;
my $colIdx;
my @trigMaxOfVars;
my @targMaxOfVars;
my @timeAtStart = (0.0, 0.0);
my @timeAtEnd = (0.0, 0.0);

# This code section will be specific to each measure

# find the maximum output value for each measure.  Will be used later if
# frac_max was specified on a measure line.  However, if Rise/Fall/Cross
# are specified then it is the maximum with the specified RFC window.
foreach $j (0 .. $numMeasures-1){
  $colIdx = MeasureCommon::findColumnInPrnFile($measureQuants[$j],\@headerVarNames);

  $trigMaxOfVars[$j] = MeasureCommon::findMaxWithinRFCwindow(\@dataFromXyce,$colIdx,
                            $trigRiseGiven[$j],$trigFallGiven[$j],$trigCrossGiven[$j],
                            $trigRiseVals[$j],$trigFallVals[$j],$trigCrossVals[$j],$trigCrossVals[$j]);

  $targMaxOfVars[$j] = MeasureCommon::findMaxWithinRFCwindow(\@dataFromXyce,$colIdx,
                            $targRiseGiven[$j],$targFallGiven[$j],$targCrossGiven[$j],
                            $targRiseVals[$j],$targFallVals[$j],$targCrossVals[$j],$targCrossVals[$j]);
}

# account for measures with frac_max=value syntax.  In that case, the trig and
# targ values are the frac_max values multipled by the maximum signal value
foreach $j (0 .. $numMeasures-1)
{
  if ($trigFracMaxFound[$j] > 0)
  {
    $trigVals[$j] = $trigVals[$j] * $trigMaxOfVars[$j]; 
  }
  if ($targFracMaxFound[$j] > 0)
  {
    $targVals[$j] = $targVals[$j] * $targMaxOfVars[$j]; 
  }
}

print "TRIG values, after any frac_max adjustment = @trigVals\n";
print "TARG values, after any frac_max adjustments = @targVals\n";

# now find point closest to trigger and target 
my $diff1, $diff2, $startTimeFound, $endTimeFound;

foreach $j (0 .. $numMeasures-1)
{
  if ( ($minValsFound[$j] < 1) && ( $tdVals[$j] > $toVals[$j] || 
        ( $trigFracMaxFound > 0  && $trigMaxOfVars[$j] < $trigVals[$j] ) ||
        ( $targFracMaxFound > 0  && $targMaxOfVars[$j] < $targVals[$j] ) ) )
  {
    print "For measure $measureNames[$j], setting calculated value to default value = $defaultVals[$j]\n"; 
    $calcVals[$j] = $defaultVals[$j];
  }
  else
  {
    # find the correct column in the output data, collected from the .prn file.  This code assumes
    # that trig and targ are using the same signal.  This may be enhanced in the future since
    # trig and targ do no have to use the same signal.
    $colIdx = MeasureCommon::findColumnInPrnFile($measureQuants[$j],\@headerVarNames);
    $startTimeFound = 0;
    $endTimeFound = 0;
    $isRising=0;
    $isFalling=0;
    $riseCount=0;
    $fallCount=0;
    $crossCount=0;
    $firstStepInWindow=0;
    $validTrigRFCWindow=0;
 
    # use absolute rise/fall sensing if FRAC_MAX is specified for this measure
    if ($trigFracMaxFound[$j] > 0) 
    { 
      $useTrigRFClevel = 0;
      $trigCrossLevel = 0;
    }
    else
    {
      $useTrigRFClevel = 1;
      $trigCrossLevel = $trigVals[$j];
    }
    # enable LAST keyword for TRIG
    if ( ($trigRiseVals[$j] < 0) || ($trigFallVals[$j] < 0) || ($trigCrossVals[$j] < 0) )
    {
      $measureRFC = 1;
    }
    else
    {
      $measureRFC = 0;
    }

    # look for trigger time 
    for $i (1 .. $#dataFromXyce )
    { 
     if (($dataFromXyce[$i][0]>=$tdVals[$j]) && ($dataFromXyce[$i][0]>=$fromVals[$j]) && 
         ($dataFromXyce[$i][0]<=$toVals[$j]))
     {
      # set prevValue variable at first time step in window
      if ($firstTimeStepInWindow < 1)
      {
        $firstTimeStepInWindow = 1;
        $previousVal = $dataFromXyce[$i][$colIdx];
      }
      
      # determine if we're within the RFC window    
      ($riseCount,$fallCount,$crossCount,$isRising,$isFalling,
         $validTrigRFCWindow,$previousVal,$newRFCwindowForLast) = 
            MeasureCommon::withinRFCwindow($dataFromXyce[$i][$colIdx],$previousVal,
	        $isRising,$isFalling,$riseCount,$fallCount,$crossCount,
                $trigRiseGiven[$j],$trigFallGiven[$j],$trigCrossGiven[$j],
                $trigRiseVals[$j],$trigFallVals[$j],$trigCrossVals[$j],
                $trigCrossLevel,$useTrigRFClevel);
        
      #print ("For index $i, validTrigRFCwindow = $validTrigRFCwindow\n");
      
      if( $validTrigRFCWindow > 0 ) 
      {
        if ($newRFCwindowForLast > 0) 
        {
	  $timeAtstart[$j] = 0;
	  $startTimeFound = 0;
        }
        # look for trigger time 
        if ($startTimeFound < 1)
        {
          $diff1 = ($dataFromXyce[$i-1][$colIdx] - $trigVals[$j]);
          $diff2 = ($dataFromXyce[$i][$colIdx] - $trigVals[$j]);
          #  print "$dataFromXyce[$i-1][0], $dataFromXyce[$i][0] $i $j $diff1 $diff2 : \n";
          if( ((($diff1 <= 0) && ($diff2 > 0)) || (($diff1 > 0) && ($diff2 <= 0))) && ($validTrigRFCWindow > 0) )
          {
            # we bracket the target point so save this time as the trigger start 
            if( abs($dataFromXyce[$i][$colIdx]-$dataFromXyce[$i-1][$colIdx]) > 0 )
            {
              $timeAtStart[$j] = ($dataFromXyce[$i][0] - $dataFromXyce[$i-1][0]) * 
                                 ( ($trigVals[$j] - $dataFromXyce[$i-1][$colIdx]) / 
                                  ($dataFromXyce[$i][$colIdx] - $dataFromXyce[$i-1][$colIdx]) ) 
                                 + $dataFromXyce[$i-1][$0];
              $startTimeFound = 1;
              if ($measureRFC < 1) {last;}
            }
            else
            {
              $timeAtStart[$j] = $dataFromXyce[$i-1][0];
              $startTimeFound = 1;
              if ($measureRFC < 1) {last;}
            }
            #print "====> at line $i with time = $dataFromXyce[$i-1][$colIdx]: $timeAtStart[$j] \n ";
          }
          elsif ( (abs($diff1) < $minVals[$j]) && (abs($diff2) >= $minVals[$j]) )
          {
            $timeAtStart[$j] = $dataFromXyce[$i-1][0];
            $startTimeFound = 1;
            #print "====> at line $i with time = $dataFromXyce[$i-1][$colIdx]: $timeAtStart[$j] \n ";
            if ($measureRFC < 1) {last;}
          }
        }
      }
     }
    }

    # look for target time.  This code assumes that trig and targ are using the same signal.
    # This may be enhanced in the future since trig and targ do no have to use the same signal.
    $isRising=0;
    $isFalling=0;
    $riseCount=0;
    $fallCount=0;
    $crossCount=0;
    $validTargRFCWindow=0;
    $firstTimeStepInWindow=0;

    # use absolute rise/fall sensing if FRAC_MAX is specified for this measure
    if ($targFracMaxFound[$j] > 0) 
    { 
      $useTargRFClevel = 0;
      $targCrossLevel = 0;
    }
    else
    {
      $useTargRFClevel = 1;
      $targCrossLevel = $targVals[$j];
    }
    # enable LAST keyword for TARG
    if ( ($targRiseVals[$j] < 0) || ($targFallVals[$j] < 0) || ($targCrossVals[$j] < 0) )
    {
      $measureRFC = 1;
    }
    else
    {
      $measureRFC = 0;
    }

    for $i (1 .. $#dataFromXyce )
    {
     if (($dataFromXyce[$i][0]>=$tdVals[$j]) && ($dataFromXyce[$i][0]>=$fromVals[$j]) && 
         ($dataFromXyce[$i][0]<=$toVals[$j]))
     {
      # set prevValue variable at first time step in window
      if ($firstTimeStepInWindow < 1)
      {
        $firstTimeStepInWindow = 1;
        $previousVal = $dataFromXyce[$i][$colIdx];
      }

      # determine if we're within the RFC window    
      ($riseCount,$fallCount,$crossCount,$isRising,$isFalling,
         $validTargRFCWindow,$previousVal,$newRFCwindowForLast) = 
            MeasureCommon::withinRFCwindow($dataFromXyce[$i][$colIdx],$previousVal,
	       $isRising,$isFalling,$riseCount,$fallCount,$crossCount,
               $targRiseGiven[$j],$targFallGiven[$j],$targCrossGiven[$j],
               $targRiseVals[$j],$targFallVals[$j],$targCrossVals[$j],
               $targCrossLevel,$useTargRFClevel);
        
      #print ("For index $i, validTrigRFCwindow = $validTrigRFCwindow\n");
    
      if( $validTargRFCWindow > 0 ) 
      {
        if ($newRFCwindowForLast > 0) 
        {
	  $timeAtEnd[$j] = 0;
	  $endTimeFound = 0;
        }
        if ($endTimeFound < 1)
        {
          $diff1 = ($dataFromXyce[$i-1][$colIdx] -$targVals[$j]);
          $diff2 = ($dataFromXyce[$i][$colIdx] - $targVals[$j]);
          #print "at time $dataFromXyce[$i][0]: i,j,diff1,diff2 = $i $j $diff1 $diff2 \n";
          # targ time must bracket the targ value.  It must also be later than the trig time
          if( ((($diff1 <= 0) && ($diff2 >= 0)) || (($diff1 >= 0) && ($diff2 <= 0))) && 
              ($dataFromXyce[$i][0] > $timeAtStart[$j]) )
          {
            # we bracket the target point so save this time as the target start 
            if( abs($dataFromXyce[$i][$j+1]-$dataFromXyce[$i-1][$colIdx]) > 0 )
            {
              $timeAtEnd[$j] = ($dataFromXyce[$i][0] - $dataFromXyce[$i-1][0]) * 
                               ( ($targVals[$j] - $dataFromXyce[$i-1][$colIdx]) / 
                                ($dataFromXyce[$i][$colIdx] - $dataFromXyce[$i-1][$colIdx]) ) 
                               + $dataFromXyce[$i-1][$0];
              $endTimeFound = 1;
              if ($measureRFC < 1) {last};
            }
            else
            {
              $timeAtEnd[$j] = $dataFromXyce[$i-1][0];
              $endTimeFound = 1;
              if ($measureRFC < 1) {last;}
            }
            #print "====> at line $i with time = $dataFromXyce[$i-1][$colIdx]: $j te $timeAtEnd[$j] \n ";
          }
          elsif ( (abs($diff1) < $minVals[$j]) && (abs($diff2) >= $minVals[$j]) 
                    && ($dataFromXyce[$i][0] > $timeAtStart[$j]) )
          {
            $timeAtEnd[$j] = $dataFromXyce[$i-1][0];
            $endTimeFound = 1;
            #print "====> at line $i with time = $dataFromXyce[$i-1][$colIdx]: $timeAtEnd[$j] \n ";
            if ($measureRFC < 1) {last;}
          }
        }
      }
     }
    }
    if ($startTimeFound > 0 && $endTimeFound > 0 && ($timeAtEnd[$j] >= $timeAtStart[$j]) )
    {
      print "For measure $measureNames[$j], (timeAtEnd, timeAtStart) = ($timeAtEnd[$j], $timeAtStart[$j])\n";
      $calcVals[$j] = $timeAtEnd[$j] - $timeAtStart[$j];
    }
    else
    {
      print "For measure $measureNames[$j], the start or end time was not found.\n";
      print "                (timeAtEnd, timeAtStart) = ($timeAtEnd[$j], $timeAtStart[$j])\n";
      $calcVals[$j] = $defaultVals[$j];
    }
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

