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

use File::Basename;

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

# check that the values in the .mt0 file are formatted in 
# scientific notation with the correct default precision (6).
my $defaultPrecision=6;

###################################################
# process each step.  Number of steps is hard-coded
###################################################
my $numSteps=2;
foreach $stepNum (1 .. $numSteps)
{
  print "Processing data for step $stepNum\n";
  # Process .mt file to get the measure names and measured values 
  # for each measure statement. Then deference to get the underlying arrays.
  # This makes the subsequent code more readable since the variable are 
  # arrays or scalars, and aren't mixed in with array references.
  $mtSuffix = $stepNum-1; 
  if (not -s "$CIRFILE.mt$mtSuffix" ) 
  { 
    print ".mt$mtSuffix file not found for step $stepNum.\n"; 
    print "Exit code = 17\n"; exit 17; 
  }
  my ($measureNamesRef,$measureValsRef) 
          = MeasureCommon::parseMeasureNamesValues($CIRFILE,$numMeasures,$stepNum-1,$numSteps);
  my @measureNames = @$measureNamesRef;
  my @measureVals = @$measureValsRef;
  
  foreach $j (0 .. $numMeasures-1)
  {
    if ($measureNames[$j] eq "FOURFAIL" || $measureNames[$j] eq "FOUR1PTFAILT" || $measureNames[$j] eq "FOURATFAIL")
    {
      #print "Skipping checking number format for measure $measureNames[$j] in mt0 file\n";
    } 
    else
    {
      #print "Checking number format for measure $measureNames[$j] in mt0 file\n";
      # precision is known.  So, second parameter is 1 in the function calls to
      # checkNumberFormat
      if ($precGiven[$j] > 0)
      {
        $retval = MeasureCommon::checkNumberFormat($measureVals[$j],1,$precVals[$j]);
      }
      else
      {
        $retval = MeasureCommon::checkNumberFormat($measureVals[$j],1,$defaultPrecision);
      }
    }
    if ( $retval != 0 )
    {
      print "test Failed!\n";
      print "Exit code = $retval\n";
      exit $retval;
    }					       
  }
}

# check that .out file exists, and open it if it does
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

# parse the .out file to find the text related to .MEASURE
my $foundStart=0;
my $foundEnd=0;
my @outLine;
my $lineCount=0;
while( $line=<NETLIST> )
{
  if ($line =~ /Measure Functions/) 
  { 
    $foundStart = 1;
    $foundEnd = 0; 
  }

  if ($foundStart > 0 && $line =~ /LASTMEASURE/ )
  {
    print ERRMSG $line;
    #print "Found last measure line\n";
    $foundEnd = 1; 
    $foundStart = 0;
  }
  elsif ( $foundStart > 0 && ( $line =~ /Solution Summary/ || $line =~ /Beginning DC/ ) )
  { 
    #print "Stopping on Solution Summary or Beginning DC line\n";
    $foundEnd = 1; 
    $foundStart = 0;
  }  
   
  if ($foundStart > 0 && $foundEnd < 1)
  {
    print ERRMSG $line;
  }
}


close(ERRMSG);

my $GSFILE="ErrorMessageTestGSfile";
my $absTol=1e-5;
my $relTol=1e-3;
my $zeroTol=1e-10;

my $dirname = dirname($XYCE_VERIFY);
my $fc = "$dirname/file_compare.pl";
#print "$fc $CIRFILE.errmsg $GSFILE $absTol $relTol $zeroTol\n";
`$fc $CIRFILE.errmsg $GSFILE $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err`;
my $retval=$? >> 8;

if ( $retval != 0 )
{
  print "test Failed!\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# check that the numbers in the .out file are formatted correctly in 
# scientific notation
if (-s "$CIRFILE.errmsg" )
{
  open(TESTFILE,"$testFileName");
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
        # precision is unknown. So, it won't be checked
        $retval = MeasureCommon::checkNumberFormat($testFileData[$i],0,$defaultPrecision);
        if ( $retval != 0 )
        {
          print "test Failed!\n";
          print "Exit code = $retval\n";
          exit $retval;
        }
      }
    }
  }
}

# do final checks on return value 
if ( $retval != 0 )
{
  print "test Failed!\n";
  print "Exit code = $retval\n";
  exit $retval;
}
else
{
  print "test passed!\n";
  print "Exit code = $retval\n";
  exit $retval;
}

