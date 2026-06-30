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

my $absTol = 0.01;
my $relTol = 0.03;
my $zeroTol = 1.0e-5;

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

#
# Did we make a measure file
#
if (not -s "$CIRFILE.mt0" ) { print "Exit code = 17\n"; exit 17; }

#
# Now we need to count the number of .MEASURE statemets in the netlist
# and make sure we have some output for each one in the output file
#
open(NETLIST, "$CIRFILE");
$numMeasures=0;
while( $line=<NETLIST> )
{
  if( $line =~ /^\.measure/i)
  {  
    $numMeasures++;
  }
}
close(NETLIST);

#
# Now look for the measure output file and count how many measurements are in it
# Also save the name and value calculated by Xyce.
#
open(RESULTS, "$CIRFILE.mt0");
$numMeasuresFound=0;
my @measureNames;
my @measureVals;
while( $line=<RESULTS> )
{
  if( $line =~ /=/)
  {  
    # parse out NAME = Value 
    chop $line;
    ($name,$sep,$value) = split(/ /, $line);
    print "name = $name value=$value\n";
    push @measureNames, $name;
    push @measureVals, $value;
    $numMeasuresFound++;
  }
}
close(RESULTS);

if( $numMeasures != $numMeasuresFound )
{
  print "Number of requested measures did not match number of printed measures.\n";
  print "Exit code = 2\n"; exit 2; 
}

#
# Make sure the measurement was not output multiple times in parallel
# in either the output file or the measure file.
for (my $i=0; $i < $numMeasures; $i++)
{
   $numMeasOut = `grep -ic $measureNames[ $i ] $CIRFILE.out`;
   if ( $numMeasOut != 1 )
   {
     print "Xyce printed out $measureNames[ $i ] measure to standard output multiple times.\n";
     $retval = 2;
   }
   $numMeasMT0 = `grep -ic $measureNames[ $i ] $CIRFILE.mt0`;
   if ( $numMeasMT0 != 1 )
   {
     print "Xyce printed out $measureNames[ $i ] measure to file multiple times.\n";
     $retval = 2;
   }
}

if ( $retval != 0 )
{
  print "Exit code = $retval\n";
  exit $retval;
}


# Now look at Xyce's output file and read in Xyce's data
open(XYCE_RESULTS, "$CIRFILE.prn");
my @calculatedMeasureVals;
my @headerVarNames;
my @dataFromXyce;
my $numPts = 0;
while( $line=<XYCE_RESULTS> )
{
  if( $line =~ /TIME/)
  {  
    # parse out Header names 
    chop $line;
    @headerVarNames = split(/\s+/, $line);
  }
  elsif($line =~ /End of/)
  {
    # end of Xyce simulation line 
    # do nothing in this case
  }
  else
  {
    # save var values 
    chop $line;
    # Remove leading spaces on line, otherwise the spaces become element 0 of "@lineOfDataFromXyce" instead of the first column of data.
    $line =~ s/^\s*//;
    @lineOfDataFromXyce = (split(/\s+/, $line));
    for ($varNum=0; $varNum <= $#lineOfDataFromXyce; $varNum++)
    {
      $dataFromXyce[$numPts][$varNum] = $lineOfDataFromXyce[$varNum];
    }
    $numPts++;
  }
}
close(XYCE_RESULTS);

# now calculate the values for these measures.  This is the part of 
# the test script that will be different for each one. 
# 

# For each signal we are looking for the time it took to go from 10% of max to 90% of max.
# Thus, first find the max.
# search the history for 10% of max and 90% of max
my @maxOfVars;
my @maxOfVars;
$maxOfVars[0]=$dataFromXyce[0][1];
$maxOfVars[1]=$dataFromXyce[0][2];

my @timeAtStart = (0.0, 0.0);
my @timeAtEnd = (0.0, 0.0);

for $i (1 .. $#dataFromXyce )
{  
  if( $dataFromXyce[$i][1] > $maxOfVars[0] )
  {
    $maxOfVars[0] = $dataFromXyce[$i][1];
  }
  
  if( $dataFromXyce[$i][2] > $maxOfVars[1] )
  {
    $maxOfVars[1] = $dataFromXyce[$i][2];
  }  
}

#print "max 0 $maxOfVars[0] max 1 $maxOfVars[1]\n";

# now find point closest to trigger and target 
# I should parse these from the netlist
my @triggerFracMax = (0.1, 0.1);
my @targetFracMax = (0.9, 0.9);

for $i (1 .. $#dataFromXyce )
{ 
  for $j (0 .. 1 )
  {
    # look for trigger time 
    my $diff1 = ($dataFromXyce[$i-1][$j+1] - $triggerFracMax[ $j ] * $maxOfVars[$j]);
    my $diff2 = ($dataFromXyce[$i][$j+1] - $triggerFracMax[ $j ]* $maxOfVars[$j]);
    #if( $j == 0 )
    #{
    #  print "$dataFromXyce[$i-1][0], $dataFromXyce[$i][0] $i $j $diff1 $diff2 : \n";
    #}
    if( ((($diff1 <= 0) && ($diff2 > 0)) || (($diff1 > 0) && ($diff2 <= 0))) )
    {
      # we bracket the target point so save this time as the trigger start 
      #$timeAtStart[$j] = 0.5*($dataFromXyce[$i-1][0] + $dataFromXyce[$i-1][0]);
      #$timeAtStart[$j] = $dataFromXyce[$i-1][0];
      if( $timeAtStart[$j] == 0.0 )
      {
        if( abs($dataFromXyce[$i][$j+1]-$dataFromXyce[$i-1][$j+1]) > 0 )
        {
          $timeAtStart[$j] = (($triggerFracMax[ $j ] * $maxOfVars[$j] - $dataFromXyce[$i-1][$j+1]) / ($dataFromXyce[$i][$j+1]-$dataFromXyce[$i-1][$j+1])) * ($dataFromXyce[$i][0]-$dataFromXyce[$i-1][0]) + $dataFromXyce[$i-1][0];
          $timeAtStart[$j] = $dataFromXyce[$i-1][0];
        }
        else
        {
          $timeAtStart[$j] = $dataFromXyce[$i-1][0];
        }
        #print "====> $j at line $i with time = $dataFromXyce[$i-1][$j+1]: $j ts $timeAtStart[$j] \n ";
      }
    }
    
    # look for target time
    $diff1 = ($dataFromXyce[$i-1][$j+1] -$targetFracMax[ $j ] *  $maxOfVars[$j]);
    $diff2 = ($dataFromXyce[$i][$j+1] - $targetFracMax[ $j ] * $maxOfVars[$j]);
    #print "$i $j $diff1 $diff2 : \n";
    #print " $diff1 $diff2 \n";
    if( ((($diff1 <= 0) && ($diff2 >= 0)) || (($diff1 >= 0) && ($diff2 <= 0))) && ($timeAtEnd[$j] == 0.0))
    {
      # we bracket the target point so save this time as the trigger start 
      # we bracket the target point so save this time as the trigger start 
      #$timeAtEnd[$j] = 0.5*($dataFromXyce[$i-1][0] + $dataFromXyce[$i-1][0]);
      #$timeAtEnd[$j] = $dataFromXyce[$i-1][0];
      if( $timeAtEnd[$j] == 0.0 )
      {
        if( abs($dataFromXyce[$i][$j+1]-$dataFromXyce[$i-1][$j+1]) > 0 )
        {
          $timeAtEnd[$j] = (($targetFracMax[ $j ] * $maxOfVars[$j] - $dataFromXyce[$i-1][$j+1]) / ($dataFromXyce[$i][$j+1]-$dataFromXyce[$i-1][$j+1])) * ($dataFromXyce[$i][0]-$dataFromXyce[$i-1][0]) + $dataFromXyce[$i-1][0];
        }
        else
        {
          $timeAtEnd[$j] = $dataFromXyce[$i-1][0];
        }
        #print "====> $j  at line $i with time = $dataFromXyce[$i-1][$j+1]: $j te $timeAtEnd[$j] \n ";
      }
    }
  }
}

for $j (0 .. 1)
{
  $calculatedMeasureVals[$j] = $timeAtEnd[$j] - $timeAtStart[$j];
  print "Measure $j => $calculatedMeasureVals[$j] = $timeAtEnd[$j] - $timeAtStart[$j] \n";
}

my @absMeasureError;
my @relMeasureError;

# Form the average = integral/total time period, and absolute vs. relative errors.
for $i (0 .. 1 )
{
  $absMeasureError[$i] = abs( $measureVals[$i] - $calculatedMeasureVals[$i] );
  $relMeasureError[$i] = abs($absMeasureError[$i] / $calculatedMeasureVals[$i]);
}

$retval = 0;
foreach $i (0 .. 1)
{
  print "For measure $measureNames[$i] calculated value = $sumOfVars[$i]: abserror = $absMeasureError[$i] relerror=$relMeasureError[$i]\n";
  if( (abs($absMeasureError[$i]) > $absTol) || ((abs($measureVals[$i]) > $zeroTol) && (abs($relMeasureError[$i]) > $relTol) ))
  {
    print "Out of tolerance with abstol = $absTol and reltol = $reltol \n";
    $retval = 2;
  }
}

if ($retval == 0) 
{
  # Re-measure test uses the same approach as the FOUR measure 
  $retval = MeasureCommon::checkRemeasure($XYCE,$XYCE_VERIFY,$CIRFILE,$absTol,$relTol,$zeroTol);
}

if ( $retval != 0 )
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

print "Exit code = $retval\n"; 
exit $retval;

