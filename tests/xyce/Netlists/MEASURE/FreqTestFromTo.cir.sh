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
# these are used to store the On and Off values for the duty measure
my @OnVals;
my @OffVals;
my @fromVals;
my @toVals;
while( $line=<NETLIST> )
{
  if( $line =~ /^\.measure/i)
  {  
    $numMeasures++;
    # try to parse out ON= and OFF= values 
    @words = split( /\s+/, $line);
    foreach $word (@words)
    {
      if( $word =~ "=" )
      {
        ($keyword, $value) = split( /=/, $word);
        if( $keyword eq "ON" )
        {
          push @OnVals, $value;
        }
        if( $keyword eq "OFF" )
        {
          push @OffVals, $value;
        }
        if( $keyword eq "FROM" )
        {
          push @fromVals, $value;
        }
        if( $keyword eq "TO" )
        {
          push @toVals, $value;
        }
      }
    }
  }
}
close(NETLIST);

#
# Now look for the measure output file and count how many measurements are in it
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
  print "Numer of requested measures did not match number of printed measures.\n";
  print "Exit code = 2\n"; exit 2; 
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
    for ($varNum=0; $varNum < @lineOfDataFromXyce; $varNum++)
    {
      #print "setting dataFromXyce[$numPts][$varNum] = $lineOfDataFromXyce[$varNum] \n";
      $dataFromXyce[$numPts][$varNum] = $lineOfDataFromXyce[$varNum];
    }
    $numPts++;
  }
}
close(XYCE_RESULTS);


# now calculate the values for these measures.  This is the part of 
# the test script that will be different for each one. 
# 
my @calMeasure = (0.0,0.0);
my @measureIsOn = (0,0);
my @measureIsOff = (1,1);  # signals start off "off" so don't count that as an off cycle.
my @numberOnCycles = (0,0);
my @numberOffCycles = (0,0);

for $i (1 .. $#dataFromXyce )
{  
  
  for $j (0 .. 1)
  {
    if( ($dataFromXyce[$i][0] >= $fromVals[$j]) && ($dataFromXyce[$i][0] <= $toVals[$j]) )
    {
	  # count On and Of cycles for better averaging 
	  if($dataFromXyce[$i][$j + 1] > $OnVals[$j] ) 
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
	
	  if($dataFromXyce[$i][$j + 1] < $OffVals[$j] ) 
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
    }
  }
}

my @absMeasureError;
my @relMeasureError;

# calculate the derivative
for $i (0 .. 1)
{
  $calMeasure[$i] = 0.5*($numberOnCycles[$i] + $numberOffCycles[$i]) / ($toVals[$i] - $fromVals[$i]);
  $absMeasureError[$i] = abs( $measureVals[$i] - $calMeasure[$i] );
  if ( $calMeasure[$i] != 0 )
  {
    $relMeasureError[$i] = abs( $absMeasureError[$i] / $calMeasure[$i]);
  }
}

$retval = 0;
foreach $i (0 .. 1)
{
  print "For measure $measureNames[$i] calculated value = $calMeasure[$i]: abserror = $absMeasureError[$i] relerror=$relMeasureError[$i]\n";
  if( ($absMeasureError[$i] > $absTol) || (($measureVals[$i] > $zeroTol) && ($relMeasureError[$i] > $relTol) ))
  {
    print "Out of tolerance with abstol = $absTol and reltol = $relTol \n";
    $retval = 2;
  }
}

# Also test remeasure if the basic measure function works
if ($retval == 0) { $retval = MeasureCommon::checkRemeasure($XYCE,$CIRFILE,$absTol,$relTol,$zeroTol); }

if ( $retval != 0 )
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

print "Exit code = $retval\n"; 
exit $retval;


