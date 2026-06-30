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
my @defaultVals = MeasureCommon::parseKeyWord($CIRFILE,"DEFAULT_VAL",$endTime);

my ($minValsPtr,$minValsFoundPtr) =  MeasureCommon::parseKeyWord($CIRFILE,"MINVAL",$endTime);
my @minVals = @$minValsPtr;
my @minValsFound = @$minValsFoundPtr;
foreach $j (0 .. $numMeasures-1){
  if ($minValsFound[$j] < 1) {$minVals[$j] = 1e-12};
}

#
# now calculate the values for this measures.  This is the part of 
# the test script may also different for each test. 
# 
my @calcVals;
my $trigColIdx;
my $targColIdx;
my $trigValColIdx;
my $targValColIdx;
my @timeAtStart = (0.0, 0.0);
my @timeAtEnd = (0.0, 0.0);
my $trigVal;
my $targVal;
my $atGiven;
my $atVal;

# This code section will be specific to each measure

# now find point closest to trigger and target 
my $diff1, $diff2, $startTimeFound, $endTimeFound;

foreach $j (0 .. $numMeasures-1)
{
    $startTimeFound = 0;
    $endTimeFound = 0;

    # hard code the column numbers for this test
    if ($j==0)
    {
      # TRIG v(1)=<val> TARG v(2)=<val>
      $trigColIdx = 1;
      $targColIdx = 2;
      $atGiven=0;
    }
    elsif ($j==1)
    {
      #TRIG v(1)=v(3) TARG v(2)=<val>
      $trigColIdx = 1;
      $targColIdx = 2;
      $trigValColIdx = 3;
      $atGiven=0;
    }
    elsif ($j==2)
    {
      #TRIG v(1)=<val> TARG v(2)=v(3)
      $trigColIdx = 1;
      $targColIdx = 2;
      $targValColIdx = 3;
      $atGiven=0;
    }
    elsif ($j==3)
    {
      #TRIG v(1)=v(3) TARG v(2)=v(3)
      $trigColIdx = 1;
      $targColIdx = 2;
      $trigValColIdx = 3;
      $targValColIdx = 3;
      $atGiven=0;
    } 
    elsif ($j==4)
    {
      #TRIG AT=<time> TARG v(2)=<val>)
      $targColIdx = 2;
      $atGiven=1;
    }    
    elsif ($j==5)
    {
      #TRIG AT=<time> TARG v(2)=v(3)
      $targColIdx = 2;
      $targValColIdx = 3;
      $atGiven=1;
    }
    else
    {
      print ("Invalid measure number in test\n"); $retval=2;
      print "Exit code = $retval\n";
      exit $retval;
    }

    # look for trigger time 
    for $i (1 .. $#dataFromXyce )
    { 
      # hand code trig levels
      if ($j==0)
      {
        $trigVal = 0.5;
      }
      elsif ($j==1)
      {
        $trigVal= $dataFromXyce[$i][$trigValColIdx];
      }
      elsif ($j==2)
      {
        $trigVal = 0.5;
      }
      elsif ($j==3)
      {
        $trigVal = $dataFromXyce[$i][$trigValColIdx];
      } 
      elsif ($j==4 || $j==5)
      {
	$atVal = 1e-3;
      }

      # look for trigger time 
      $diff1 = ($dataFromXyce[$i-1][$trigColIdx] - $trigVal);
      $diff2 = ($dataFromXyce[$i][$trigColIdx] - $trigVal);
      #  print "$dataFromXyce[$i-1][0], $dataFromXyce[$i][0] $i $j $diff1 $diff2 : \n";
      if ($atGiven > 0)
      {
        if ($dataFromXyce[$i][0] >= $atVal)
	{
	  $timeAtStart[$j] = $dataFromXyce[$i][0];
          $startTimeFound = 1;
          last;
	}
      }
      elsif( ((($diff1 <= 0) && ($diff2 > 0)) || (($diff1 > 0) && ($diff2 <= 0))) )
      {
        # we bracket the target point so save this time as the trigger start 
        if( abs($dataFromXyce[$i][$trigColIdx]-$dataFromXyce[$i-1][$trigColIdx]) > 0 )
        {
          $timeAtStart[$j] = ($dataFromXyce[$i][0] - $dataFromXyce[$i-1][0]) * 
                               ( ($trigVal - $dataFromXyce[$i-1][$trigColIdx]) / 
                               ($dataFromXyce[$i][$trigColIdx] - $dataFromXyce[$i-1][$trigColIdx]) ) 
                               + $dataFromXyce[$i-1][$0];
          $startTimeFound = 1;
          last;
        }
        else
        {
          $timeAtStart[$j] = $dataFromXyce[$i-1][0];
          $startTimeFound = 1;
          last;
        }
        #print "====> at line $i with time = $dataFromXyce[$i-1][$trigColIdx]: $timeAtStart[$j] \n ";
      } 
      elsif ( (abs($diff1) < $minVals[$j]) && (abs($diff2) >= $minVals[$j]) )
      {
        $timeAtStart[$j] = $dataFromXyce[$i-1][0];
        $startTimeFound = 1;
        #print "====> at line $i with time = $dataFromXyce[$i-1][$colIdx]: $timeAtStart[$j] \n ";
        last;
      }
    }

    # look for target time.  This code assumes that trig and targ are using the same signal.
    # This may be enhanced in the future since trig and targ do no have to use the same signal.
    for $i (1 .. $#dataFromXyce )
    {
      # hand code targ levels
      if ($j==0)
      {
        $targVal = 0.5;
      }
      elsif ($j==1)
      {
        $targVal = 0.5;
      }
      elsif ($j==2)
      {
        $targVal = $dataFromXyce[$i][$targValColIdx];
      }
      elsif ($j==3)
      {
        $targVal = $dataFromXyce[$i][$targValColIdx];
      } 
      elsif ($j==4)
      {
        $targVal = 0.5;
      }
      elsif ($j==5)
      {
        $targVal = $dataFromXyce[$i][$targValColIdx];
      } 

      $diff1 = ($dataFromXyce[$i-1][$targColIdx] -$targVal);
      $diff2 = ($dataFromXyce[$i][$targColIdx] - $targVal);
      #print "at time $dataFromXyce[$i][0]: i,j,diff1,diff2 = $i $j $diff1 $diff2 \n";
      # targ time must bracket the targ value.  It must also be later than the trig time
      if( ((($diff1 <= 0) && ($diff2 >= 0)) || (($diff1 >= 0) && ($diff2 <= 0))) && 
          ($dataFromXyce[$i][0] > $timeAtStart[$j]) )
      {
        # we bracket the target point so save this time as the target start 
        if( abs($dataFromXyce[$i][$j+1]-$dataFromXyce[$i-1][$targColIdx]) > 0 )
        {
          $timeAtEnd[$j] = ($dataFromXyce[$i][0] - $dataFromXyce[$i-1][0]) * 
                             ( ($targVal - $dataFromXyce[$i-1][$targColIdx]) / 
                             ($dataFromXyce[$i][$targColIdx] - $dataFromXyce[$i-1][$targColIdx]) ) 
                             + $dataFromXyce[$i-1][$0];
          $endTimeFound = 1;
          last;
        }
        else
        {
          $timeAtEnd[$j] = $dataFromXyce[$i-1][0];
          $endTimeFound = 1;
          last;
        }
        #print "====> at line $i with time = $dataFromXyce[$i-1][$colIdx]: $j te $timeAtEnd[$j] \n ";
      }
      elsif ( (abs($diff1) < $minVals[$j]) && (abs($diff2) >= $minVals[$j]) 
                && ($dataFromXyce[$i][0] > $timeAtStart[$j]) )
      {
        $timeAtEnd[$j] = $dataFromXyce[$i-1][0];
        $endTimeFound = 1;
        #print "====> at line $i with time = $dataFromXyce[$i-1][$colIdx]: $timeAtEnd[$j] \n ";
        last;
      }
    }
    if ($startTimeFound > 0 && $endTimeFound > 0)
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

