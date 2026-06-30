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

# check the warning message
@searchstrings = ("Netlist warning: RFC_LEVEL will be ignored for measure WHEN_IGNORE_RFC_LEVEL");
if( $Tools->checkError("$CIRFILE.out",@searchstrings) != 0)
{
  print "Failed to find correct warning messages\n";
  print "Exit code = 2\n"; exit 2;
}

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

# MINVAL is an optional keywords
my ($minValsPtr,$minValGivenPtr) = MeasureCommon::parseKeyWord($CIRFILE,"MINVAL",$endTime);
my @minVals = @$minValsPtr;
my @minValGiven = @$minValGivenPtr;

# this has to return a pointer because WHEN is an optional keyword for the DERIV measure
# even though WHEN is a mandatory keyword for this measure
my ($whenValsPtr,$whenGivenPtr) =  MeasureCommon::parseKeyWord($CIRFILE,"WHEN",$endTime);
my @whenVals = @$whenValsPtr;

# get the FIND info and determine which measures are FIND-WHEN
my ($findQuantsPtr,$findGivenPtr) =  MeasureCommon::getFindQuants($CIRFILE);
my @findQuants = @$findQuantsPtr;
my @findGiven = @$findGivenPtr;

# for these keywords, whether they were given matters later in the .sh file
my ($riseValsPtr,$riseGivenPtr) =  MeasureCommon::parseKeyWord($CIRFILE,"RISE",$endTime);
my @riseVals = @$riseValsPtr;
my @riseGiven = @$riseGivenPtr;
my ($fallValsPtr,$fallGivenPtr) =  MeasureCommon::parseKeyWord($CIRFILE,"FALL",$endTime);
my @fallVals = @$fallValsPtr;
my @fallGiven = @$fallGivenPtr;
my ($crossValsPtr,$crossGivenPtr) =  MeasureCommon::parseKeyWord($CIRFILE,"CROSS",$endTime);  
my @crossVals = @$crossValsPtr;
my @crossGiven = @$crossGivenPtr;

#
# now calculate the values for this measures.  This is the part of 
# the test script may also different for each test. 
# 
# Find when the waveform crosses the user specified level (WHEN qualifier) for the possible
# variants of the RISE, FALL and CROSS qualifiers.  
my @calcVals;
my @numberOfRises = (0,0);
my @numberOfFalls = (0,0);
my @numberOfCrosses = (0,0);
my $colIdx;
my $findColIdx;
my $validRFCwindow;
my $firstTimeStepInWindow;
my $isRising;
my $isFalling;
my $riseCount;
my $fallCount;
my $crossCount;
my $previousVal;
my $backDiff;
my $forwardDiff;
my $measureTime;
my $measureLastRFC;

# This code ONLY works if the target value (a) in the WHEN clause
# (e.g., WHEN V(1)=a ) is a FIXED value.  It does NOT work for the
# WHEN V(1)=V(2) case.
foreach $j (0 .. $numMeasures-1){
  if ($tdVals[$j] > $toVals[$j])
  {
    $calcVals[$j] = $defaultVals[$j];
  }
  else
  {
    #reset various flags and counters for RFC calculations
    $isRising = 0;
    $isFalling = 0;
    $riseCount=0;
    $fallCount=0;
    $crossCount=0;
    $resultFound=0;
    $validRFCwindow=0;
    $firstTimeStepInWindow=0;
    $measureLastRFC=0;

    # find the correct column in the output data, collected from the .prn file
    $colIdx = MeasureCommon::findColumnInPrnFile($measureQuants[$j],\@headerVarNames);

    # set the measure to the default value
    $calcVals[$j] = $defaultVals[$j];

    # are we looking for a last RFC window?
    if ($riseVals[$j] < 0 || $fallVals[$j] < 0 || $crossVals[$j] < 0)
    {
      $measureLastRFC = 1;
    }

    if ($findGiven[$j] > 0)
    {
       $findColIdx = MeasureCommon::findColumnInPrnFile($findQuants[$j],\@headerVarNames);
       #print "findColIdx and findQuant for measure $j = $findColIdx and $findQuants[$j]\n";  
    }
 
    # process the WHEN column (colIdx).  This code section will be specific to each measure
    for $i (1 .. $#dataFromXyce )
    {  
      if (($dataFromXyce[$i][0]>=$tdVals[$j]) && ($dataFromXyce[$i][0]>=$fromVals[$j]) && 
         ($dataFromXyce[$i][0]<=$toVals[$j]))
      {
        if ($riseGiven[$j] < 1 && $fallGiven[$j] < 1 && $crossGiven[$j] < 1)
        {
          # MINVAL criteria is tested first in the C++ code.  Note: For the regression tests, a MINVAL
          # criteria might not generate the same answer as the C++ code if RISE/FALL/CROSS are 
          # also specified.
          if ( ($minValGiven[$j] > 0) && ( abs($dataFromXyce[$i][$colIdx] - $whenVals[$j]) < $minVals[$j] ) ) 
          {
            if ( $findGiven[$j] < 1 )
            {
	      $calcVals[$j] = $dataFromXyce[$i][0];
              last;
            }
            else
            {
              $calcVals[$j] =  $dataFromXyce[$i][$findColIdx];
              last;
            }
	  }
          elsif ((($dataFromXyce[$i-1][$colIdx] < $whenVals[$j]) && ($dataFromXyce[$i][$colIdx] > $whenVals[$j])) ||
               (($dataFromXyce[$i-1][$colIdx] > $whenVals[$j]) && ($dataFromXyce[$i][$colIdx] < $whenVals[$j])))
          {
            $measureTime = $dataFromXyce[$i][0] - ( (($dataFromXyce[$i][0] - $dataFromXyce[$i-1][0])/
                               ($dataFromXyce[$i][$colIdx] - $dataFromXyce[$i-1][$colIdx])) *
                               ($dataFromXyce[$i][$colIdx] - $whenVals[$j]));
            if ( $findGiven[$j] < 1 )
            {
	      $calcVals[$j] = $measureTime;
              last;
            }
            else
            {
              $calcVals[$j] = $dataFromXyce[$i][$findColIdx] - ($dataFromXyce[$i][0] - $measureTime)*
                                  ( ($dataFromXyce[$i][$findColIdx]-$dataFromXyce[$i-1][$findColIdx])/
                                    ($dataFromXyce[$i][0] - $dataFromXyce[$i-1][0] ) );
              last;
            }
          }
        }
        else
        { 
          # set prevValue variable at first time step in window
          if ($firstTimeStepInWindow < 1)
          {
	    $firstTimeStepInWindow = 1;
            $previousVal = $dataFromXyce[$i][$colIdx];
          }

          $rfcVal = $whenVals[$j];
          # WHEN is always given for this measure, so the last value in the
          # argument list is set to 1.
          ($riseCount,$fallCount,$crossCount,$isRising,$isFalling,
             $validRFCwindow,$previousVal,$newRFCwindowForLast) = 
                MeasureCommon::withinRFCwindow($dataFromXyce[$i][$colIdx],$previousVal,
		    $isRising,$isFalling,$riseCount,$fallCount,$crossCount,
                    $riseGiven[$j],$fallGiven[$j],$crossGiven[$j],
                    $riseVals[$j],$fallVals[$j],$crossVals[$j],
                    $rfcVal,$rfcLevelGiven[$j],1);
       
          #print ("For time $dataFromXyce[$i][0], validRFCwindow, RFC counts = $validRFCwindow, ($riseCount,$fallCount,$crossCount)\n");
          if ($validRFCwindow > 0)
          {
            if ($newRFCwindowForLast > 0) 
            {
	      $resultFound=0;
              $calcVals[$j]=$defaultVals[$j];
              #print "For measure $measureNames[$j], new RFC window at time $dataFromXyce[$i][0]\n";
              #print "RFC Counts=($riseCount,$fallCount,$crossCount)\n";
            }

            if( $resultFound < 1 )               
            {
              $backDiff = $dataFromXyce[$i-1][$colIdx] - $whenVals[$j];
              $forwardDiff = $dataFromXyce[$i][$colIdx] - $whenVals[$j];
              if( (($backDiff < 0.0) && ($forwardDiff > 0.0)) || (($backDiff > 0.0) && ($forwardDiff < 0.0)) )
              {
                # interpolate from points that bound the WHEN value
                $measureTime = $dataFromXyce[$i][0] - ( (($dataFromXyce[$i][0] - $dataFromXyce[$i-1][0])/
                               ($dataFromXyce[$i][$colIdx] - $dataFromXyce[$i-1][$colIdx])) *
                               ($dataFromXyce[$i][$colIdx] - $whenVals[$j]));
                if ( $findGiven[$j] < 1 )
                {
		  $calcVals[$j] = $measureTime;
                }
                else
                {
                  $calcVals[$j] = $dataFromXyce[$i][$findColIdx] - ($dataFromXyce[$i][0] - $measureTime)*
                                  ( ($dataFromXyce[$i][$findColIdx]-$dataFromXyce[$i-1][$findColIdx])/
                                    ($dataFromXyce[$i][0] - $dataFromXyce[$i-1][0] ) );
                }
                $resultFound=1;
                if ( $measureLastRFC < 1 ) { last; }
              }
            }
          }
        }
      }
    }
  }
}

# these are the tolerances used in comparing the measures calculated by this 
# script to those calculated by Xyce.  Zerotol is needed because some measured
# values should be zero (and reltol won't make sense in that case).
my $absTol = 1.0e-4;
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

