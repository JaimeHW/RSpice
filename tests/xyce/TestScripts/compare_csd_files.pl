#!/usr/bin/env perl

# This perl script compares two files, and is aware of the difference
# between strings and numbers.  It should ignore whitespace differences.
# For strings the comparison is case sensitive.
# Numbers are compared with an absTol and relTol.  There is also a zeroTol.

# Usage:
# compare_csd_files file1 file2 absTol relTol zeroTol

use strict;

use Scalar::Util qw(looks_like_number);

# Argument processing.  Don't assume any defaults for absTol, relTol and zeroTol.
# User must explicitly pass them into the script.
while ( $ARGV[0] ne "-" && $ARGV[0] =~ /^-/ )
{
    my $diffopts .= " $ARGV[0]";
    shift @ARGV;
}

if ($#ARGV != 4)
{
    print STDERR "Insufficient number of arguments on command line.\n";
    print STDERR "Usage:  $0 file1 file2 absTol relTol zeroTol\n";
    exit 1;
}

my $testFileName=$ARGV[0];
my $goldFileName=$ARGV[1];
my $absTol=$ARGV[2];
my $relTol=$ARGV[3];
my $zeroTol=$ARGV[4];

open(TESTFILE,"$testFileName");
open(GSFILE,"$goldFileName");

# first test if the files have the same number of lines
my ($testFileCount, $gsCount);
for ($testFileCount=0; <TESTFILE>; $testFileCount++) { }
for ($gsCount=0; <GSFILE>; $gsCount++) { }

close(GSFILE);
close(TESTFILE);

my $exitCode=0;
my $foundDataStart=0;

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
  open(TESTFILE,"$testFileName");
  open(GSFILE,"$goldFileName");
  my $lineCount=0;
  my ($lineGS, $lineTestFile, @gsData, @testFileData);
  while( ($lineGS=<GSFILE>) && ($lineTestFile=<TESTFILE>) )
  {
    $lineCount++;
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

    if ($lineTestFile =~ /#C/)
    {
      $foundDataStart=1;
    }
           
    if ($#gsData != $#testFileData)
    {
      my $testLineLen=$#testFileData+1;
      my $gsLineLen=$#gsData+1;
      print STDERR "File $testFileName and Gold Standard have a different # of elements on line $lineCount:\n";
      print STDERR "File $testFileName had $testLineLen elements\n";
      print STDERR "Gold Standard had $gsLineLen elements\n";
      $exitCode=2;
    } 
    # skip version and date/time lines, since they may change in each
    # test run.  Also need to handle XBEGIN line separately
    elsif ( ($lineTestFile =~ /SOURCE/) || ($lineTestFile =~ /TIME/) )
    {
      #no op
    }
    else
    {
      # the two files have the same number of items on a line.  
      # compare individual values as scalars or strings
      my (@testData, @goldData, $absDiff, $relDiff);   
      for( my $i=0; $i<=$#testFileData; $i++ )
      {
        if ( ($testFileData[$i] =~ /XBEGIN/) || ($testFileData[$i] =~ /XEND/) )
        {
          # XBEGIN line has numbers formatted as XBEGIN='0.00000000e+00' XEND='1.00000000e-02'
          # Need to extract the numbers.
          @testData = split(/[\']/,$testFileData[$i]);
          @goldData = split(/[\']/,$gsData[$i]);
          if ($#gsData != $#testFileData)
          {
            print STDERR "File $testFileName and Gold Standard have mismatched XBEGIN line\n";
            $exitCode=2;
            last;
            
          }
          if( ($testData[0] eq $goldData[0]) ) 
          {
            # two string fields match for XBEGIN and XEND
          }
          else
          {
            print STDERR "String elements failed compare on line $lineCount:\n";
            print STDERR "File $testFileName produced: \"$testFileData[$i]\" \n";
            print STDERR "Gold Standard was: \"$gsData[$i]\" \n";
            $exitCode=2;
            last;
          }
          if ( ( abs($testData[1]) <= $zeroTol ) && ( abs($goldData[1]) <= $zeroTol ) )
	  {
            # no op since both test and gold data are less than the zero tolerance
	  }
          else
          {
	    $absDiff = abs($testData[1] - $goldData[1]);
            $relDiff = $absDiff / abs($goldData[1]);   
            if ( ( $absDiff < $absTol ) && ( $relDiff < $relTol ) )
            {
              # two numeric fields match to within specified absolute tolerance and relative tolerance
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
        else
        {
          # this handles date elements from lines that don't start with XBEGIN.  
          # There are three cases, as noted in the comments below.
	  if ( ($foundDataStart > 0) && ($testFileData[$i] =~ /:/ ) && ($gsData[$i] =~/:/) )
          { 
            # these are fields that contain time points.  They are of the form  0.0001:1
            # where 0.0001 is the value and 1 is the idx.  This next block breaks them
            # apart into two pieces, each of which is tested separately.  These values are
            # found after the #C line
            @testData = split(/:/,$testFileData[$i]);
            @goldData = split(/:/,$gsData[$i]); 
            foreach my $idx (0 .. 1)
            {
              if ( ( abs($testData[$idx]) <= $zeroTol ) && ( abs($goldData[$idx]) <= $zeroTol ) )
	      {
                # no op since both test and gold data are less than the zero tolerance
	      }
              else
              {
		$absDiff = abs($testData[$idx] - $goldData[$idx]);
                if (abs($goldData[$idx]) > 0) 
		{
                  $relDiff = $absDiff / abs($goldData[$idx]);
                }
                else
                {
		  $relDiff = 0;
                }   
                if ( ( $absDiff < $absTol ) && ( $relDiff < $relTol ) )
                {
                  # two numeric fields match to within specified absolute tolerance and relative tolerance
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
              if ($exitCode != 0) { last;}
            } 
          }
          elsif ( looks_like_number($testFileData[$i]) && looks_like_number($gsData[$i]) )
          {  
            # this block is for numbers that didn't match the value:idx format above
            if ( ( abs($testFileData[$i]) <= $zeroTol ) && ( abs($gsData[$i]) <= $zeroTol ) )
	    {
              # no op since both test and gold data are less than the zero tolerance
	    }
            else
            {
	      $absDiff = abs($testFileData[$i] - $gsData[$i]);
              if (abs($gsData[$i]) > 0) 
	      {
                $relDiff = $absDiff / abs($gsData[$i]); 
              } 
              else
              {
                $relDiff = 0;
              } 
  
              if ( ( $absDiff < $absTol ) && ( $relDiff < $relTol ) )
              {
                # two numeric fields match to within specified absolute tolerance and relative tolerance
                # debug info, to verify that the comparison is working okay
                #print STDERR "Numeric comparison for column $i on line $lineCount: \n";
                #print STDERR "File $testFileName had \"$testFileData[$i]\" while Gold Standard had \"$gsData[$i]\" \n";
                #print STDERR "Calculated absDiff and relDiff = ($absDiff,$relDiff)\n"; 
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
            # debug info to verify that the string comparison is working okay
            #print STDERR "$testFileData[$i] same as $gsData[$i]\n";
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
  }

  # close files
  close(GSFILE);
  close(TESTFILE);
}

exit $exitCode;
