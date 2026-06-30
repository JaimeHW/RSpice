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

my $crossLevel=0;

#
# now calculate the values for this measures.  This is the part of 
# the test script may also different for each test. 
# 
# Find the minimum value for each voltage, but ignore data outside
# of the intersection of >=TD and FROM>= and <=TO
my @calcVals;
my $firstValue;
my $colIdx;
my @numberOfRises;
my @numberOfFalls;
my @numberOfCrosses;
my $isRising;
my $isFalling;
my $validRFCWindow;

foreach $j (0 .. $numMeasures-1){
  $firstValue=0;
  if ($tdVals[$j] > $toVals[$j])
  {
    $calcVals[$j] = $defaultVals[$j];
    $validRFCWindow=0;
  }
  else
  {
    # find the correct column in the output data, collected from the .prn file
    $colIdx = MeasureCommon::findColumnInPrnFile($measureQuants[$j],\@headerVarNames);
    $numberOfRises[$j] = 0;
    $numberOfFalls[$j] = 0;
    $numberOfCrosses[$j] = 0;
    $isRising=0;
    $isFalling=0;
    $calcVals[$j] = 0;
    $validRFCWindow=0;

    # process that column.  This code section will be specific to each measure
    for $i (0 .. $#dataFromXyce )
    {  
      if (($dataFromXyce[$i][0]>=$tdVals[$j]) && ($dataFromXyce[$i][0]>=$fromVals[$j]) && 
         ($dataFromXyce[$i][0]<=$toVals[$j]))
      {
        # current approach to finding RISE/FALL count
        if ($dataFromXyce[$i][$colIdx] > $dataFromXyce[$i-1][$colIdx] && $isRising < 1)
        {
          $numberOfRises[$j] += 1;
          $isRising = 1.0;
          $isFalling = 0.0;
        }
        if ($dataFromXyce[$i][$colIdx] < $dataFromXyce[$i-1][$colIdx] && $isFalling < 1)
        {
          $numberOfFalls[$j] += 1;
          $isRising = 0.0;
          $isFalling = 1.0;
        }

        if ( (($dataFromXyce[$i][$colIdx]-$crossLevel < 0) && ($dataFromXyce[$i-1][$colIdx]-$crossLevel > 0)) ||
             (($dataFromXyce[$i][$colIdx]-$crossLevel > 0) && ($dataFromXyce[$i-1][$colIdx]-$crossLevel < 0)) )
        {
	  $numberOfCrosses[$j] += 1;
        } 

        if ( ($riseGiven[$j] < 1 && $fallGiven[$j] < 1 && $crossGiven[$j] < 1) ||
             ( ($riseGiven[$j] > 0) && ( $numberOfRises[$j] == $riseVals[$j]  || $riseVals[$j] == -1) ) ||
             ( ($fallGiven[$j] > 0) && ( $numberOfFalls[$j] == $fallVals[$j]  || $fallVals[$j] == -1) ) ||
             ( ($crossGiven[$j] > 0) && ( $numberOfCrosses[$j] == $crossVals[$j]  || $crossVals[$j] == -1) ) )
        {
	  $validRFCWindow=1;
          if ($firstValue==0)
          {
	    $firstValue=1;
            $calcVals[$j] = $dataFromXyce[$i][$colIdx];
            #print "For measure $j, (time,calcVal)=($dataFromXyce[$i][0],$dataFromXyce[$i][$colIdx])\n";
          }
          elsif ($dataFromXyce[$i][$colIdx] < $calcVals[$j])
          {
            $calcVals[$j] = $dataFromXyce[$i][$colIdx];
            #print "For measure $j, (time,calcVal)=($dataFromXyce[$i][0],$dataFromXyce[$i][$colIdx])\n";
	  }
        }
      }
    }
  }

  # measure failed because RFC window was never found
  if ($validRFCWindow < 1) {$calcVals[$j]=$defaultVals[$j];}
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

