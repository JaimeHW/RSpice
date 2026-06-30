#!/usr/bin/env perl

use XyceRegression::Tools;

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

# The test circuit uses .measure INTEG to integrate one of the 
# voltage signals as a comparison to the SDT() function.  Thus, the 
# first part of this script is to verify that the .measure INTEG is 
# working correctly.

# these are the tolerances used in comparing the measures calculated by this 
# script to those calculated by Xyce.  Zerotol is needed because some measured
# values should be zero (and reltol won't make sense in that case).

my $absTol = 5.0e-4;
my $relTol = 0.02;
my $zeroTol = 1.0e-5;
my $ddtabsTol = 0.15;

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
# these are used to store parameters which can limit or windows the scope of a measure
my @OnVals = (0.0, 0.0);
my @OffVals = (0.0, 0.0);
my @FromVals = (0.0, 0.0);
my @ToVals = (0.0, 0.0);
my @minVals = (0.0, 0.0);
my @riseVals = (1, 1);
my @findVals = (0.0, 0.0);
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
          @OnVals[$numMeasures-1] = $value;
        }
        elsif( $keyword eq "OFF" )
        {
          @OffVals[$numMeasures-1] = $value;
        }
        elsif( $keyword eq "FROM" )
        {
          @FromVals[$numMeasures-1] = $value;
        }
        elsif( $keyword eq "TO" )
        {
          @ToVals[$numMeasures-1] = $value;
        }
        elsif( $keyword eq "MINVAL" )
        {
          @minVals[$numMeasures-1] = $value;
        }
        elsif( $keyword eq "RISE" )
        {
          @riseVals[$numMeasures-1] = $value;
        }
        else
        {
          # not great, but use to catch caes of 
          # when v(1)=some_value
          @findVals[$numMeasures-1] = $value;
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
# the test script that will be different for each one.  In this case
# we only need to integrate column 3 (v(1)) as that is used in the measurement.
# 
my @calMeasure = (0.0);

for $i (1 .. $#dataFromXyce )
{  
  # find the time point Xyce output which 
  # is closest to the desired $x1Vals or $x2Vals
  $calMeasure[0] = $calMeasure[0] + ($dataFromXyce[$i][1] - $dataFromXyce[$i-1][1]) * 0.5 * ($dataFromXyce[$i-1][3] + $dataFromXyce[$i][3]);
}

my @absMeasureError;
my @relMeasureError;

# calculate the measure
for $i (0 )
{
  $absMeasureError[$i] = abs( $measureVals[$i] - $calMeasure[$i] );
  if ( $calMeasure[$i] != 0 )
  {
    $relMeasureError[$i] = abs( $absMeasureError[$i] / $calMeasure[$i]);
  }
}

$retval = 0;
foreach $i (0 )
{
  print "For measure $measureNames[$i] calculated value = $calMeasure[$i]: abserror = $absMeasureError[$i] relerror=$relMeasureError[$i]\n";
  if( ($absMeasureError[$i] > $absTol) || (($measureVals[$i] > $zeroTol) && ($relMeasureError[$i] > $relTol) ))
  {
    print "Out of tolerance with abstol = $absTol and reltol = $relTol \n";
    $retval = 2;
  }
}

# We have checked that .measure INTEG is working so now we can use 
# that knowledge to compare columns 3 and 4.
#
# finally, this netlist is written such that 
# if {TIME}, sdt and ddt are working correctly, then
# column 1 == 2        a test that TIME is equal to {TIME}, i.e. that expressions with explicit time dependence do the right thing.
# column 4 == 5        a test that .measure INTEG and sdt() are equal.
# column 6 == 7 == 8   a test that ddt() is equal to the analytic time derivative.

# EXCEPT on the DC OP where DDT will not have a history to use for calculation.
# Note, column 0=Index and 1=time 
# we will now verify this.  

for $i (0 .. $#dataFromXyce )
{  
  # find the time point Xyce output which 
  # is closest to the desired $x1Vals or $x2Vals
  $calMeasure[0] = $calMeasure[0] + ($dataFromXyce[$i][1] - $dataFromXyce[$i-1][1]) * 0.5 * ($dataFromXyce[$i-1][3] + $dataFromXyce[$i][3]);
  
  $absdiff = abs($dataFromXyce[$i][1] - $dataFromXyce[$i][2]);
  if( $absdiff > $absTol )
  {
    print "Out of tolerance on line $i test (column 2 = column 3): $absdiff \n";
    $retval = 2;
  }
  
  $absdiff = abs($dataFromXyce[$i][4] - $dataFromXyce[$i][5]);
  if( $absdiff > $absTol )
  {
    print "Out of tolerance on line $i test (column 4 = column 5): $absdiff \n";
    $retval = 2;
  }
  
  # Note we are skipping line 0 for column 6 as DDT() the function tested in 
  # column 6 cannot calculate a time derivative during the DC OP.
  $absdiff = abs($dataFromXyce[$i][6] - $dataFromXyce[$i][7]);
  if( ($i > 0) && ($absdiff > $ddtabsTol ) )
  {
    print "Out of tolerance on line $i test (column 6 = column 7): $absdiff \n";
    $retval = 2;
  }
  $absdiff = abs($dataFromXyce[$i][6] - $dataFromXyce[$i][8]);
  if( $absdiff > $ddtabsTol )
  {
    print "Out of tolerance on line $i test (column 6 = column 8): $absdiff \n";
    $retval = 2;
  }
}

if ( $retval != 0 )
{
  print "Exit code = $retval\n"; 
  exit $retval;
}
  
print "Exit code = $retval\n"; 
exit $retval;
