#!/usr/bin/env perl

# This perl script compares two files, and is aware of the difference
# between strings and numbers.  It should ignore whitespace differences.
# For strings the comparison is case sensitive.
# Numbers are compared with an absTol and relTol.  There is also a zeroTol.

# Usage:
# file_compare.pl testfile goodfile absTol relTol zeroTol

use strict; 

use Scalar::Util qw(looks_like_number);
use Getopt::Long;

my ($debug, $help);
my ($absTol, $relTol, $zeroTol, $freqRelTol);

# used to print additional debugging information, mainly about
# successful comparisons
sub debugPrint
{
  print @_ if ($debug);
}

#Read any options
&GetOptions( "debug!" => \$debug,
             "help!" => \$help );

if ( defined $help )
{
  print "Usage: $0 [options] testfile goodfile absTol relTol zeroTol\n";
  print "options:\n";
  print "--debug\n";
  print "--help\n";
  exit 0;
}

# Argument processing.  Don't assume any defaults for absTol, relTol and zeroTol.
# User must explicitly pass them into the script.
if ($#ARGV != 4)
{
    print STDERR "Invalid number of arguments on command line.\n";
    print STDERR "Usage: $0 [options] testfile goodfile absTol relTol zeroTol\n";
    exit 1;
}

my $testFileName=$ARGV[0];
my $goldFileName=$ARGV[1];
my $absTol=$ARGV[2];
my $relTol=$ARGV[3];
my $zeroTol=$ARGV[4];

# verify input, during debug mode
debugPrint "testFile = $testFileName\n";
debugPrint "goldFile = $goldFileName\n";
debugPrint "(absTol relTol zeroTol) = ($absTol $relTol $zeroTol)\n\n"; 

# open files and check that the files have the same number of lines
if (not -s "$goldFileName" ) 
{ 
  print STDERR "Missing Gold Standard file: $goldFileName\nExit code = 2\n"; 
  exit 2;
}
if (not -s "$testFileName" ) 
{ 
  print STDERR "Missing Test file: $testFileName\nExit code = 14\n";  
  exit 14;
}

open(TESTFILE,"$testFileName");
open(GSFILE,"$goldFileName");

# first test if the files have the same number of lines
my ($testFileCount, $gsCount);
for ($testFileCount=0; <TESTFILE>; $testFileCount++) { }
for ($gsCount=0; <GSFILE>; $gsCount++) { }

close(GSFILE);
close(TESTFILE);

my $exitCode=0;
if ($testFileCount != $gsCount) 
{
  print STDERR "file $testFileName doesn't match the Gold Standard\n";
  print STDERR "$testFileName line count= $testFileCount\n";
  print STDERR "Gold Standard line count= $gsCount\n";
  $exitCode=2;
}
else
{ # If the line counts match, then compare in detail.
  # Re-open the files and compare them line by line.
  debugPrint "Line counts match, beginning detailed comparison\n";
  open(TESTFILE,"$testFileName");
  open(GSFILE,"$goldFileName");
  my $lineCount=0;
  my ($lineGS, $lineTestFile, @gsData, @testFileData);
  while( ($lineGS=<GSFILE>) && ($lineTestFile=<TESTFILE>) )
  {
    $lineCount++;
    debugPrint "Processing line $lineCount\n";
    # process a line into text and values.
    chop $lineGS;
    # Remove leading spaces on lineGS, otherwise the spaces become 
    # element 0 of "@gsData" instead of the first column of data.
    $lineGS =~ s/^\s*//;
    @gsData = (split(/[\s,]+/, $lineGS));

    # process a line into text and values.
    chop $lineTestFile;
    # Remove leading spaces on line, otherwise the spaces become 
    # element 0 of "testFileData" instead of the first column of data.
    $lineTestFile =~ s/^\s*//;
    @testFileData = (split(/[\s,]+/, $lineTestFile));
           
    if ($#gsData != $#testFileData)
    {
      my $testLineLen=$#testFileData+1;
      my $gsLineLen=$#gsData+1;
      print STDERR "File $testFileName and Gold Standard have a different # of elements on line $lineCount:\n";
      print STDERR "File $testFileName had $testLineLen elements\n";
      print STDERR "Gold Standard had $gsLineLen elements\n";
      $exitCode=2;
    }
    else
    {
      # the two files have the same number of items on a line.  
      # compare individual values as scalars  
      for( my $i=0; $i<=$#testFileData; $i++ )
      {
        if ( looks_like_number($testFileData[$i]) && looks_like_number($gsData[$i]) )
        { 
          if ($testFileData[$i] ==  $gsData[$i])
          {
            # exact match
            debugPrint "Numeric comparison for column $i on line $lineCount: \n";
            debugPrint "File $testFileName had \"$testFileData[$i]\" and Gold Standard had \"$gsData[$i]\" \n";
            debugPrint "Exact match\n";
          }
          elsif ( ( abs($testFileData[$i]) <= $zeroTol ) && ( abs($gsData[$i]) <= $zeroTol ) )
          {
            # no op since both test and gold data are less than the zero tolerance
            debugPrint "Numeric comparison for column $i on line $lineCount: \n";
            debugPrint "File $testFileName had \"$testFileData[$i]\" while Gold Standard had \"$gsData[$i]\" \n";
            debugPrint "Both are less than zero tolerance\n";
          }
          else
          {
            my $absDiff = abs($testFileData[$i] - $gsData[$i]);
            my $relDiff = $absDiff / abs($gsData[$i]);   
            if ( ( $absDiff < $absTol ) && ( $relDiff < $relTol ) )
            {
              # two numeric fields match to within specified absolute tolerance and relative tolerance
              # debug info, to verify that the comparison is working okay
              debugPrint "Numeric comparison for column $i on line $lineCount: \n";
              debugPrint "File $testFileName had \"$testFileData[$i]\" while Gold Standard had \"$gsData[$i]\" \n";
              debugPrint "Calculated absDiff and relDiff = ($absDiff,$relDiff)\n"; 
            }
            elsif ( ((abs($gsData[$i]) - 180.0) < $absTol) && ((abs($testFileData[$i]) - 180.0) < $absTol) )
            {
              # corner case where Xyce's FFT output prints the phase in degrees.  Xyce uses the arctan2(y,x)
              # function for this and there is a discontinuity at +/- 180 degrees phase.  Small rounding errors 
              # in an FFT library can push the result from -180 degrees to +180 degrees which essentially are the
              # same thing but such a numeric difference would not pass normal checks. 
              debugPrint "Numeric comparison for column $i on line $lineCount: \n";
              debugPrint "File $testFileName had \"$testFileData[$i]\" and Gold Standard had \"$gsData[$i]\" \n";
              debugPrint "Matches given discontinutiy in phase that occurs at +/- 180 degrees.\n";
            }
            else
            {
              print STDERR "Numeric comparison failed in column $i on line $lineCount: \n";
              print STDERR "File $testFileName had \"$testFileData[$i]\" while Gold Standard had \"$gsData[$i]\" \n";
              print STDERR "Calculated absDiff and relDiff = ($absDiff,$relDiff)\n"; 
              $exitCode=2;
              last;
            }
          }
        }
        elsif( ($testFileData[$i] eq $gsData[$i]) ) 
        {
          # two string fields match
          debugPrint "string match:\n";
          debugPrint "test string= $testFileData[$i]\n"; 
          debugPrint "gold string= $gsData[$i]\n"; 
        }
        else
        {
          print STDERR "String elements failed compare on line $lineCount:\n";
          print STDERR "File $testFileName produced: \"$testFileData[$i]\" \n";
          print STDERR "Gold Standard was: \"$gsData[$i]\" \n";
          $exitCode=2;
          last;
        }                 
      }     
    }
  }

  # close files
  close(GSFILE);
  close(TESTFILE);
}

exit $exitCode;
