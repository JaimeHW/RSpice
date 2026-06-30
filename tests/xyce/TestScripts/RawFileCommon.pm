#!/usr/bin/env perl

package RawFileCommon;
use strict;
use XyceRegression::Tools;
use File::Basename;
use Scalar::Util qw(looks_like_number);

sub makeAndVerifyRawFile
{
  my ($XYCE,$XYCE_VERIFY,$CIRFILE,$GOLDRAW,$absTol,$relTol,$zeroTol)=@_;

  my $retcode = 0;
  my $xyceexit=0;

  $GOLDRAW =~ s/\.prn$//; # remove the .prn at the end.

  system("rm -f $CIRFILE.raw $CIRFILE.err $CIRFILE.raw.gold-info $CIRFILE.raw.info");
  system("rm -f $CIRFILE.prn");

  # run Xyce and check out the output files
  my $CMD="$XYCE -r $CIRFILE.raw -a $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";

  if (system($CMD) != 0) {
    `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
    $xyceexit=1;
  }

  if ($xyceexit!=0) {print "Exit code = 10\n"; return 10;}

  if ( -f "$CIRFILE.prn" ) {
      print STDERR "Extra output file $CIRFILE.prn\n";
      $xyceexit=14;
  }

  if ( !(-f "$CIRFILE.raw")) {
      print STDERR "Missing output file $CIRFILE.raw\n";
      $xyceexit=14;
  }

  if ($xyceexit!=0) {return $xyceexit;}

  $retcode = compareRawFiles($XYCE_VERIFY,$CIRFILE,$GOLDRAW,$absTol,$relTol,$zeroTol);
  return $retcode;
}

sub compareRawFiles
{
  my ($XYCE_VERIFY,$CIRFILE,$GOLDRAW,$absTol,$relTol,$zeroTol)=@_;

  my $retcode = 0;

  my $Tools = XyceRegression::Tools->new();
  my $dirname = dirname($XYCE_VERIFY);
  my $fc = "$dirname/file_compare.pl";
  my $XYCE_RAW_READ = $XYCE_VERIFY;
  $XYCE_RAW_READ =~ s?xyce_verify\.pl?spice_read.py?;

  # Test the .RAW file output. Test that the values and strings in the 
  # match to the required tolerances
  if (system("grep -v 'Date:' $GOLDRAW.raw > $CIRFILE.raw.gold-filtered 2>$CIRFILE.raw.gold-filtered.out") != 0)
  {
      print STDERR "Date line not found in file $CIRFILE.raw.gold-filtered, see $CIRFILE.raw.gold-filtered.out\n";
      $retcode = 2;
  }
  if (system("grep -v 'Date:' $CIRFILE.raw > $CIRFILE.raw.filtered 2>$CIRFILE.raw.filtered.out") != 0) {
      print STDERR "Date line not found in file $CIRFILE.raw, see $CIRFILE.raw.filtered.out\n";
      $retcode = 2;
  }

  if ($retcode != 0) 
  {
    return $retcode;
  }
  else
  {
    # use file_compare.pl, assuming the output was made in serial.  That .pl script is used
    # by many other tests, and should be "correct".
    `$fc $CIRFILE.raw.filtered $CIRFILE.raw.gold-filtered $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err`;
    $retcode=$? >> 8;
  }

  if ( $retcode != 0 )
  {
    print "Test failed comparison of Gold and test .raw files, assuming serial execution\n";
    print "Will retry assuming output was re-ordered in parallel\n";
  }
  else
  {
    print "Passed file comparison of .raw files\n";
    return $retcode;
  }

  # retry to compare files, assuming that the ordering of the variables
  # is different in serial vs. parallel.  This code is specific to this test, and
  # may be "buggier" than file_compare.pl.
  my $testFileName = "$CIRFILE.raw.filtered";
  my $goldFileName = "$CIRFILE.raw.gold-filtered";
  open(TESTFILE,"$testFileName");
  open(GSFILE,"$goldFileName");

  # first test if the files have the same number of lines
  # This is duplicative of the test, assuming serial execution, but is
  # done because we may not know why the file_compare test failed.
  my $testFileCount;
  my $gsCount;
  for ($testFileCount=0; <TESTFILE>; $testFileCount++) { }
  for ($gsCount=0; <GSFILE>; $gsCount++) { }

  close(GSFILE);
  close(TESTFILE);

  $retcode=0;
  if ($testFileCount != $gsCount) 
  {
    print STDERR "file $testFileName doesn't match the Gold Standard\n";
    print STDERR "$testFileName line count= $testFileCount\n";
    print STDERR "Gold Standard line count= $gsCount\n";
    $retcode=2;
  }
  else
  {
    # If the line counts match, then compare in detail. Re-open the files 
    # and compare the three sections (header, variable info and data).
    open(TESTFILE,"$testFileName");
    open(GSFILE,"$goldFileName");
    my $lineCount=0;
    my $numPtsInFile=0;
    my $numVarsInFile=0;
    my $lineGS;
    my $lineTestFile;
    my @gsData;
    my @testFileData;

    # First six lines should be:
    #   Title
    #   Plotname
    #   Flags
    #   No. Variables
    #   No. Points
    #   Variables
    while( ($lineGS=<GSFILE>) && ($lineTestFile=<TESTFILE>) && ($lineCount < 5) )
    {
      $lineCount++;
      @gsData=splitLine($lineGS);
      @testFileData=splitLine($lineTestFile);   
      if ($#gsData != $#testFileData)
      {
        print STDERR "File $testFileName and Gold Standard have a different # of elements on line $lineCount:\n";
        print STDERR "File $testFileName had $#testFileData elements\n";
        print STDERR "Gold Standard had $#gsData elements\n";
        return 2;
      }
      else
      {
	$retcode = compareLineData(\@testFileData,\@gsData,$absTol,$relTol,$zeroTol,$testFileName,$lineCount);
        if ($retcode !=0) { return $retcode; }
      }
       
      # pick off the number of variables and points in the Gold File
      if ( ($gsData[0] eq "No.") && ($gsData[1] eq "Variables:") ) { $numVarsInFile = @gsData[2];} 
      if ( ($gsData[0] eq "No.") && ($gsData[1] eq "Points:") ) { $numPtsInFile = @gsData[2];} 
    }

    # the next set of lines are the variables names and types.
    # These may be re-ordered between serial and parallel.
    # So, make two sets of arrays, so we can figure out the relative
    # ordering between the Gold and Test Files.
    my @varNameGold;
    my @varNameTestFile;
    my @varTypeGold;
    my @varTypeTestFile;
    my $varCount=0;

    # Note: this while loop will also read in the Variable: line, that precedes that
    # data blocks.
    while( ($lineGS=<GSFILE>) && ($lineTestFile=<TESTFILE>) && ($lineCount < ($numVarsInFile+5) ) )
    {
      $lineCount++;
      @gsData=splitLine($lineGS);
      @testFileData=splitLine($lineTestFile);
      # Perl arrays start at 0.  So, last element should have an index of 2.
      if ( ($#gsData != 2) || ($#testFileData != 2) )
      {
        print STDERR "File $testFileName and Gold Standard have wrong number of entries on line $lineCount:\n";
        print STDERR "File $testFileName had index of $#testFileData for its last element\n";
        print STDERR "Gold Standard had index $#gsData for its last element\n";
        return 2;
      } 
            push @varNameGold, $gsData[1];
      push @varNameTestFile, $testFileData[1];
      push @varTypeGold, $gsData[2];
      push @varTypeTestFile, $testFileData[2];
      $varCount++;
    }
    
    # Error out if the number of variables doesn't match the header info
    if ( $varCount != $numVarsInFile )
    {
      print STDERR "Incorrect number of variables listed in the Variables section.\n";
      print STDERR "Found $varCount, but should be \n";
      print return 2;
    } 

    # Now check that the corresponding rows in the Gold and Test Files match.
    # This will also make an array that maps between the two files, so that
    # data blocks can be checked.
    my $idxGold;
    my $idxTestFile;
    my @goldTestIdxMap;
    my $matchCount=0;
    for( $idxGold=0; $idxGold<$numVarsInFile; $idxGold++ )
    {
      $matchCount=0;
      for( $idxTestFile=0; $idxTestFile<$numVarsInFile; $idxTestFile++ )
      {
        if ( $varNameGold[$idxGold] eq $varNameTestFile[$idxTestFile] )
        {
          if ( $varTypeGold[$idxGold] ne $varTypeTestFile[$idxTestFile] )
          {
            print STDERR "Types did not match for variable $varNameGold[$idxGold].\n";
            print STDERR "Gold File had $varTypeGold[$idxGold].  Test file had $varTypeTestFile[$idxTestFile]\n";
            return 2;
          }
          else
          {
	    $matchCount++;
            push @goldTestIdxMap, $idxTestFile;
          }
        } 
      }
      if ($matchCount != 1)
      {
        print STDERR "found $matchCount matches for variable $varNameGold[$idxGold] in Test File\n";
        return 2;
      }
    }
    print "Test to Gold File mapping = @goldTestIdxMap\n";

    # Next line should be Values:  
    # It should have been read-in previously.  So, check that we're not at end-of-file,
    # based on previous count ($gsCount) of the number of lines in the files.
    if ( $lineCount <= $gsCount)
    {
      @gsData=splitLine($lineGS);
      @testFileData=splitLine($lineTestFile);
      if ( !( ($gsData[0] eq "Values:") && ($testFileData[0] eq "Values:") )  ||
            ($#gsData != 0) || ($#testFileData != 0))
      {
        print STDERR "Comparison failed on Values: line\n";
        return 2;
      }  
    }

    # now compare the data blocks
    my $blockIdx=0;
    my $varIdx=0;
    my @goldDataPts;
    my @testFileDataPts;
    for ($blockIdx=0; $blockIdx < $numPtsInFile; $blockIdx++)
    {
      # read the data block from both files into an array.
      # Note each data block is followed by a blank line.
      for ($varIdx=0; $varIdx<=$numVarsInFile; $varIdx++)
      {
	$lineCount++;
        if ( !( ($lineGS=<GSFILE>) && ($lineTestFile=<TESTFILE>) ) )
	{
          print STDERR "Test File $testFileName appears to have different number of lines than Gold File $goldFileName\n";
          return 2;
	}
        else
	{
          # make arrays to compare
          $lineCount++;
          @gsData=splitLine($lineGS);
          @testFileData=splitLine($lineTestFile);
          if ($varIdx == 0)
          {
            if ( $gsData[0] != $testFileData[0] )
	    {
              print STDERR "Test File $testFileName and Gold File $goldFileName differ at line $lineCount\n";
              return 2;
	    }
            else
            {
	      push @goldDataPts, $gsData[1];
              push @testFileDataPts, $testFileData[1];
            }
          }
          else
          {
            push @goldDataPts, $gsData[0];
            push @testFileDataPts, $testFileData[0];
          }
        }
      }
      # Now iterate back through those two arrays, doing a numeric-comparison
      # based on the mapping @goldTestIdxMap 
      my $absDiff;
      my $relDiff;
      for ($varIdx=0; $varIdx<$numVarsInFile; $varIdx++)
      {
        #This should only be numeric data.
        if ( ( abs($testFileDataPts[$goldTestIdxMap[$varIdx]]) <= $zeroTol ) && ( abs($goldDataPts[$varIdx]) <= $zeroTol ) )
        {
          # no op since both test and gold data are less than the zero tolerance
        }
        else
        {
          $absDiff = abs($testFileDataPts[$goldTestIdxMap[$varIdx]] - $goldDataPts[$varIdx]);
          $relDiff = $absDiff / abs($goldDataPts[$varIdx]);   
          if ( ($absDiff == 0) || (( $absDiff < $absTol ) && ( $relDiff < $relTol )) )
          {
            # two numeric fields match to within specified absolute tolerance and relative tolerance
            # debug info, to verify that the comparison is working okay
            #print STDERR "Numeric comparison of data succeeded for (block,variable) count = ($blockIdx,$varIdx)\n";
            #print STDERR "File $testFileName had \"$testFileDataPts[$goldTestIdxMap[$varIdx]]\" \n";
            #print STDERR "Gold Standard had \"$goldDataPts[$varIdx]\" \n";
            #print STDERR "Calculated absDiff and relDiff = ($absDiff,$relDiff)\n"; 
          }
          else
          {
            print STDERR "Numeric comparison of data failed for (block,variable) count = ($blockIdx,$varIdx)\n";
            print STDERR "File $testFileName had \"$testFileDataPts[$goldTestIdxMap[$varIdx]]\" \n";
            print STDERR "Gold Standard had \"$goldDataPts[$varIdx]\" \n";
            print STDERR "Calculated absDiff and relDiff = ($absDiff,$relDiff)\n"; 
            $retcode=2;
            last;
          }
        }
      }  
      # also check for blank line after last data point
      if ( $goldDataPts[$numVarsInFile] != $testFileData[$numVarsInFile] )
      {
        print STDERR "Test File $testFileName and Gold File $goldFileName differ at line $lineCount\n";
        return 2;
      }
      # clear the comparision arrays
      @goldDataPts=();
      @testFileDataPts=(); 
    }
  }

  return $retcode;
}

sub splitLine()
{
  my ($inputLine)=@_;

  # process a line into text and values.
  chop $inputLine;
  # Remove leading spaces on inputLine, otherwise the spaces become 
  # element 0 of "@gsData" instead of the first column of data.
  $inputLine =~ s/^\s*//;
  my @inputData = (split(/[\s,]+/, $inputLine));

  return @inputData;
}

sub compareLineData()
{
  my ($testFileDataRef,$gsDataRef,$absTol,$relTol,$zeroTol,$testFileName,$lineCount)=@_;
  my @testFileData = @$testFileDataRef;
  my @gsData = @$gsDataRef;

  my $i;
  my $absDiff;
  my $relDiff;
  my $exitCode=0;

  for( $i=0; $i<=$#testFileData; $i++ )
  {
    if ( looks_like_number($testFileData[$i]) && looks_like_number($gsData[$i]) )
    {  
      if ( ( abs($testFileData[$i]) <= $zeroTol ) && ( abs($gsData[$i]) <= $zeroTol ) )
      {
        # no op since both test and gold data are less than the zero tolerance
      }
      else
      {
        $absDiff = abs($testFileData[$i] - $gsData[$i]);
        $relDiff = $absDiff / abs($gsData[$i]);   
        if ( ($absDiff == 0) || (( $absDiff < $absTol ) && ( $relDiff < $relTol )) )
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

  return $exitCode;
} 

#
# We need the 1; at the end because, when Perl loads a module, Perl checks
# to see that the module returns a true value to ensure it loaded OK. 
#
1;

#

