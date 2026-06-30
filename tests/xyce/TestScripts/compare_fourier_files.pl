#!/usr/bin/env perl

# This perl script compares two output files from .FOUR liness, and is aware of
# the difference between strings and numbers.  It should ignore whitespace
# differences. For strings the comparison is case sensitive.
# Numbers are compared with an absTol and relTol.  There is also a zeroTol.
# There is also a separate phaseAbsTol for comparing the phase numbers.

# Usage:
# file_compare.pl testfile goodfile absTol phaseAbsTol relTol zeroTol

use strict;

use Scalar::Util qw(looks_like_number);
use Getopt::Long;

my ($debug, $help);
my ($absTol, $phaseAbsTol, $relTol, $phaseRelTol, $zeroTol);

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
  print "Usage: $0 [options] testfile goodfile absTol relTol phaseAbsTol phaseRelTol zeroTol\n";
  print "options:\n";
  print "--debug\n";
  print "--help\n";
  exit 0;
}

# Argument processing.  Don't assume any defaults for absTol, relTol, phaseAbsTol, phaseRelTol and zeroTol.
# User must explicitly pass them into the script.
if ($#ARGV != 6)
{
    print STDERR "Invalid number of arguments on command line.\n";
    print STDERR "Usage: $0 [options] testfile goodfile absTol relTol phaseAbsTol phaseRelTol zeroTol\n";
    exit 1;
}

my $testFileName=$ARGV[0];
my $goldFileName=$ARGV[1];
my $absTol=$ARGV[2];
my $relTol=$ARGV[3];
my $phaseAbsTol=$ARGV[4];
my $phaseRelTol=$ARGV[5];
my $zeroTol=$ARGV[6];

# verify input, during debug mode
debugPrint "testFile = $testFileName\n";
debugPrint "goldFile = $goldFileName\n";
debugPrint "(absTol relTol phaseAbsTol phaseRelTol zeroTol) = ($absTol $relTol $phaseAbsTol $phaseRelTol $zeroTol)\n\n";

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
{
  # If the line counts match, then compare in detail.
  # Re-open the files and compare them line by line.
  debugPrint "Line counts match, beginning detailed comparison\n";
  open(TESTFILE,"$testFileName");
  open(GSFILE,"$goldFileName");

  my $lineCount=0;
  my ($lineGS, $lineTestFile, @gsData, @testFileData);
  # Compare all data, including all header information
  while( ($lineGS=<GSFILE>) && ($lineTestFile=<TESTFILE>) )
  {
    $lineCount++;
    debugPrint "Processing line $lineCount\n";
    my $headerline=0;

    # process a gold file line into text and values.
    chop $lineGS;
    # Remove leading spaces on line, otherwise the spaces become
    # element 0 of "@gsData" instead of the first column of data.
    $lineGS =~ s/^\s*//;
    @gsData = (split(/[\s,]+/, $lineGS));

    # process a test file line into text and values.
    chop $lineTestFile;
    # Remove leading spaces on line, otherwise the spaces become 
    # element 0 of "testFileData" instead of the first column of data.
    $lineTestFile =~ s/^\s*//;
    @testFileData = (split(/[\s,]+/, $lineTestFile));

    if ($#testFileData != $#gsData)
    {
      my $testLineLen=$#testFileData+1;
      my $gsLineLen=$#gsData+1;
      print STDERR "File $testFileName and Gold Standard have a different # of elements on line $lineCount:\n";
      print STDERR "File $testFileName had $testLineLen elements\n";
      print STDERR "Gold Standard had $gsLineLen elements\n";
      $exitCode=2;
    }
    elsif ($#gsData > 0)
    {
      # integer fields on first two lines will get special handling.
      if (!looks_like_number($gsData[0]))
      {
        $headerline = 1;
        debugPrint "Header line found at line $lineCount\n";
      }

      # The two files have the same number of items on a line.
      # Compare individual values as scalars.
      my $prevWord="";
      for( my $i=0; $i<=$#testFileData; $i++ )
      {
        if ( looks_like_number($testFileData[$i]) && looks_like_number($gsData[$i]) )
        {
          if ($testFileData[$i] == $gsData[$i])
          {
            # exact match
            debugPrint "Numeric comparison for column $i on line $lineCount: \n";
            debugPrint "File $testFileName had \"$testFileData[$i]\" and Gold Standard had \"$gsData[$i]\" \n";
            debugPrint "Exact match\n";
          }
          elsif( ( abs( $testFileData[$i] ) <= $zeroTol ) && ( abs( $gsData[$i] ) <= $zeroTol ) )
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
            if ( ($headerline==0) && (($i==3) || ($i==5)) )
            {
              # Phase needs different handling, but all lines with phase values only have numbers
              # on that line.  So they are not a "header line".
              if ( ($absDiff < $phaseAbsTol) && ($relDiff < $phaseRelTol) )
              {
                debugPrint "Phase comparison passed $testFileData[$i] , $gsData[$i]\n";
                debugPrint "Calculated absDiff and relDiff = ($absDiff,$relDiff)\n";
              }
              else
              {
                print STDERR "Phase fields on line failed compare on line $lineCount:\n";
                print STDERR "File $testFileName had \"$testFileData[$i]\" while Gold Standard had \"$gsData[$i]\" \n";
                print STDERR "Calculated absDiff and relDiff = ($absDiff,$relDiff)\n";
                $exitCode=2;
                last;              
              }
            }
            elsif ( ($headerline==1) && (($prevWord eq "Harmonics:") || ($prevWord eq "Gridsize:")) )
            {
              {
                print STDERR "Integer fields on header line failed compare on line $lineCount:\n";
                print STDERR "File $testFileName had \"$testFileData[$i]\" and Gold Standard had \"$gsData[$i]\" \n";
                $exitCode=2;
                last;
              }
            }
            elsif (($absDiff < $absTol) && ($relDiff < $relTol)  )
            {
               # regular compare
               debugPrint "File $testFileName had \"$testFileData[$i]\" while Gold Standard had \"$gsData[$i]\" \n";
               debugPrint "Calculated absDiff and relDiff = ($absDiff,$relDiff)\n";
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
        elsif ($testFileData[$i] eq $gsData[$i])
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

        $prevWord = $gsData[$i];
      }
    }
  }

  # close files
  close(GSFILE);
  close(TESTFILE);
}

exit $exitCode;
