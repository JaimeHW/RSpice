#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# Notes on what this test does:
#
# This script runs two variantes of test circuit bug269.cir in 4 ways:
#  1. bug269_printFormatRaw.cir with ".print format=raw" in ascii raw mode 
#     The raw file is saved as bug269_printFormatRaw.cir.ascii.raw
#  2. bug269_printFormatRaw.cir with ".print format=raw" in binary raw mode 
#     The raw file is saved as bug269_printFormatRaw.cir.raw
#  3. bug269.cir with the command line option "-a -r" to generate an ascii raw 
#     output file containing all of the simulation data.
#  4. bug269.cir with the command line option "-r" to generate a binary raw 
#     output file containing all of the simulation data.

# the raw output files from 1 and 2 are then compared to ensure they had 
# the same headers and same data.  Lilkewise the raw files from 3 and 4 
# care compared to ensure they have the same headers and data.

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

# set an absTol and relTol for comparing the values from the raw files
# absTol may seem high, but some value are of the order 1e4 and we are limited
# in accuracy by what is printed to the file (i.e. about 8 digits of accuracy)
my $absTol=1.0e-4;
my $relTol=1.0e-3;

#
# Run Xyce on bug269_printFormatRaw.cir WITH command line option "-a" to make an 
# ascii raw file which outputs just what is on the print line.
$retval = -1;
$CIRFILEwithPrintRaw="bug269_printFormatRaw.cir";
# wrapXyce is not fully compatible with AC as it
# signals an error if the output file doesn't end with End Of Xyce(TM) Simulation
# which AC does not output.
#$retval=$Tools->wrapXyce($XYCE,$CIRFILEwithPrintRaw);
$CMD="$XYCE -a $CIRFILEwithPrintRaw > $CIRFILEwithPrintRaw.ascii.out 2> $CIRFILEwithPrintRaw.ascii.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

# check that Xyce exited without error
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a raw file
if (not -s "$CIRFILEwithPrintRaw.raw" ) { print STDERR "Missing -a mode $CIRFILEwithPrintRaw.raw using command $CMD\n"; print "Exit code = 14\n"; exit 14; }
rename( "$CIRFILEwithPrintRaw.raw", "$CIRFILEwithPrintRaw.ascii.raw");

#
# Run Xyce on bug269_printFormatRaw.cir WITHOUT command line option "-a" to make a
# binary raw file which outputs just what is on the print line.
$CMD="$XYCE $CIRFILEwithPrintRaw > $CIRFILEwithPrintRaw.out 2> $CIRFILEwithPrintRaw.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

# check that Xyce exited without error
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a prn file
if (not -s "$CIRFILEwithPrintRaw.raw" ) { print STDERR "Missing $CIRFILEwithPrintRaw.raw\n"; print "Exit code = 14\n"; exit 14; }

#
#
# Run Xyce on bug269.cir WITH command line option "-a -r <raw file name>" to make an 
# ascii raw file which outputs all of the simulation data. 
#
$CMD="$XYCE -a -r $CIRFILE.ascii.raw $CIRFILE > $CIRFILE.ascii.out 2> $CIRFILE.ascii.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a prn file
if (not -s "$CIRFILE.ascii.raw" ) { print "Missing -a -r mode $CIRFILEwithPrintRaw.ascii.raw\n"; print "Exit code = 14\n"; exit 14; }

#
# Run Xyce on bug269.cir WITH command line option "-r <raw file name>" to make a
# binary raw file which outputs all of the simulation data. 
#
#
$CMD="$XYCE -r $CIRFILE.raw $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
# check that Xyce made a prn file
if (not -s "$CIRFILE.raw" ) { print "Missing -r mode $CIRFILEwithPrintRaw.raw\n"; print "Exit code = 14\n"; exit 14; }

#
# now examine the raw files to see that they are valid
# and have the right number of output steps.
#

# These first two blocks of code just check that the 
# headers are the same between the two files and that 
# ascii file has "Values" before the data section 
# and the binary file has "Binary" before the data section.
#

sub readRawFile {
  # 
  # tries to read a raw file and return the following data 
  # $title        -- title of raw file. Only one allowed per file
  # $date         -- date file was generated 
  # @plotname     -- array of plot names from each plot section of the file  
  # @dataSize     -- for each plot section, the dataSize is 1=real (1 double) or 2=complex (2 doubles)
  # @numVars      -- for each plot section, the number of vars per output line 
  # @numPoints    -- for each plot section, the number of output "lines" in this plot section 
  # @data         -- a multidimensional array [plotnum][point number][var number]
  # input parameter
  my $filename = @_[0];
 
  # things I'll return to the caller. 
  my $title;
  my $date;
  my @plotname;
  my @dataSize;
  my @numVars;
  my @varNames;
  my @numPoints;
  my @data;
  my $retval = 0;
  my $numVarsAsciiFile = 0;
  my $numPointsAsciiFile = 0;
  my $dataType = "";
  my $numPlotSections = 0;
  my $titleFound = 0;
  my $dateFound = 0;
  my $plotNameFound = 0;
  open(INPUT_ASCII,$filename) or die "Can't open file $filename.\n";
  while ( my $line_ascii = <INPUT_ASCII> )
  {  
    if( $line_ascii =~ "Title:" )
    {
      $titleFound = 1;
      $title = $line_ascii;
      chomp $title;
      $title =~ s/Title://;
    }
    elsif( $line_ascii =~ "Date:" )
    {
      $dateFound = 1;
      $date = $line_ascii;
      chomp $date;
      $date =~ s/Date://;
    }
    elsif( $line_ascii =~ "Plotname:" )
    {
      $plotNameFound = 1;
      $aPlotName = $line_ascii;
      chomp $aPlotName;
      $aPlotName =~ s/^.*Plotname://;
      push @plotname, $aPlotName;
      $numPlotSections++;
    }
    elsif( $line_ascii =~ "Flags:" )
    {
      @lineVals = split ' ', $line_ascii;
      if( $lineVals[1] =~ "complex" )
      {
        push @dataSize, 2;
      }
      else
      {
        push @dataSize, 1;
      }
    }
    elsif( $line_ascii =~ "No. Variables:" )
    {
      $numVarsFound=1;
      @lineVals = split ' ', $line_ascii;
      push @numVars, $lineVals[2];
    }
    elsif( $line_ascii =~ "No. Points:" )
    {
      $numPointsFound=1;
      @lineVals = split ' ', $line_ascii;
      push @numPoints, $lineVals[2];
    }
    elsif( $line_ascii =~ "Variables:" )
    {
      # save the variable names;
      for( $nv=0; $nv<$numVars[@numVars-1]; $nv++ )
      {
        $line_ascii = <INPUT_ASCII>;
        if( $line_ascii eq undef )
        {
          # variable block was too short!
          print "File $filename did not have numVars variable names;.\n";
          print "  numVars should equal $numVars[@numVars-1] \n";
          print "  but ran out of data at numVars=$nv \n";
          $retVal = 2;
          last;
        }
        @lineVals = split ' ', $line_ascii;
        $varNames[ ($numPlotSections - 1) ][ $nv ] = $lineVals[2];
      }
    }
    elsif( $line_ascii =~ "Values" )
    {
      # hit data section after the header 
      # read the data to ensure that the number of
      # vars and points (and type) is correct.
  
      for( $np=0; $np<$numPoints[@numPoints-1]; $np++ )
      {
        # note we use the upper limit on this loop of $nv<($numVars[@numVars-1] * $dataSize[@dataSize-1]+1)
        # because there is a blank line between each data group. 
        for( $nv=0; $nv<($numVars[@numVars-1] * $dataSize[@dataSize-1]+1); $nv=$nv+$dataSize[@dataSize-1] ) 
        {
          $line_ascii = <INPUT_ASCII>;
          if ($line_ascii eq undef )
          {
            # data block was too short!
            print "File $filename did not have numVars * numPoints data points.\n";
            print "  numVars = $numVars[@numVars-1] \n";
            print "  numPoints = $numPoints[@numPoints-1] \n";
            $retVal = 2;
            last;
          }
          chomp $line_ascii;
          if ($line_ascii ne "" )
          {
            #print "about to process \"$line_ascii\n";
            $offset=0;
            if( $nv== 0 ) 
            {
              # first data line for any timepoint or frequency starts with an
              # index number so we need to look past that to find the data
              $offset=1;
            }
            @lineVals = split /[\s\t,]+/, $line_ascii;
            $data[ ($numPlotSections - 1) ][ $np ][ $nv ] = $lineVals[1];
            if( $dataSize[@dataSize-1] == 2 )
            {
              $data[ ($numPlotSections - 1) ][ $np ][ $nv+1 ] = $lineVals[2];
            }
          }
        }    
      }
    }
    elsif( $line_ascii =~ "Binary" )
    {
      # hit data section after the header 
      # read the data to ensure that the number of
      # vars and points (and type) is correct.
      # for not just do this as block read 
      
      # get the location in the file 
      $binaryDataStart = tell INPUT_ASCII;
      open(INPUT_BINARY,$filename) or die "Can't do second open of file $filename.\n";
      binmode( INPUT_BINARY );
      # $/ is the record separator and is by default "\n" by setting it
      # to a reference to an integer, we specify that reads should only 
      # be than many integer bytes.  So the next line sets the reads to 
      # 8 bytes each.  
      $/ = \8;  # we will read 8 bytes per record 
      
      # move the file pointer to where the binary data section starts. 
      seek INPUT_BINARY, $binaryDataStart, 0;
      # read in a set of data which is [index] [value] ... [value]
      # the number of doubles to read is numVars * dataSize (i.e. real or complex) + 1
      for( $np=0; $np<$numPoints[@numPoints-1]; $np++ )
      {
        # note we use the upper limit on this loop of $nv<($numVars[@numVars-1] * $dataSize[@dataSize-1]+1)
        # because there is an index at the beginning of each data group. 
        for( $nv=0; $nv<($numVars[@numVars-1] * $dataSize[@dataSize-1]); $nv++ ) 
        {
          $dataBuff = <INPUT_BINARY>;
          $aValue = unpack "d", $dataBuff;
          # binary format doesn't have an index variable
          $data[ ($numPlotSections - 1) ][ $np ][ $nv ] = $aValue;
        }
      }
      # relocate the file pointer for text reading. 
      $newFileLoc = tell INPUT_BINARY - $binaryDataStart;
      seek INPUT_ASCII, $newFileLoc, 1;
      # reset the record separator 
      $/ = "\n";
      # close our binary reading 
      close INPUT_BINARY;
    }
  }
  close( INPUT_ASCII );
  if( ($titleFound == 0) || ($dateFound==0) || ($plotNameFound==0) || ($numVarsFound == 0) || ($numPointsFound==0) )
  {
    print "File $filename did not have all raw file components on read.\n";
    $retVal = 2;
  }
  #print "about to return and plotname = @plotname\n";
  #print "about to return and dataSize = @dataSize\n";
  #print "about to return and numVars = @numVars\n";
  #print "about to return and numPoints = @numPoints\n";
  #print "about to return and data = @data\n";
  return ($retVal, $title, $date, \@plotname, \@dataSize, \@numVars, \@varNames, \@numPoints, \@data );

}


# read ascii print format raw file variables prefixed with apf_
($retVal, $apf_title, $apf_date, $apf_plotname_ref, $apf_dataSize_ref, $apf_numVars_ref, $apf_varNames_ref, $apf_numPoints_ref, $apf_data_ref ) = readRawFile("$CIRFILEwithPrintRaw.ascii.raw" );
if( $retval != 0 )
{
  print "file $CIRFILEwithPrintRaw.ascii.raw failed read.\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# read binary print format raw file variables prefixed with bpf_
($retVal, $bpf_title, $bpf_date, $bpf_plotname_ref, $bpf_dataSize_ref, $bpf_numVars_ref, $bpf_varNames_ref, $bpf_numPoints_ref, $bpf_data_ref ) = readRawFile("$CIRFILEwithPrintRaw.raw" );
if( $retval != 0 )
{
  print "file $CIRFILEwithPrintRaw.raw failed read.\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# compare plots and data from the two print format raw files
if( @$apf_plotname_ref != @$bpf_plotname_ref )
{
  print "files $CIRFILEwithPrintRaw.ascii.raw $CIRFILEwithPrintRaw.raw had different numbers of plot sections.\n";
  $retval = 2;
  print "Exit code = $retval\n";
  exit $retval;
}

for ( $nplot = 0; $nplot < @$apf_plotname_ref; $nplot++ )
{
  #print "Plot \"$$plotname_ref[$nplot]\"\n";
  if( $$apf_numPoints_ref[ $nplot ] != $$bpf_numPoints_ref[ $nplot ] )
  {
    print "files $CIRFILEwithPrintRaw.ascii.raw $CIRFILEwithPrintRaw.raw had different numbers of points in plot section $nplot.\n";
    $retval = 2;
    print "Exit code = $retval\n";
    exit $retval;
  }
  for ( $npoints = 0; $npoints < $$apf_numPoints_ref[ $nplot ]; $npoints++ )
  {
    if( ($$apf_numVars_ref[ $nplot ] * $$apf_dataSize_ref[ $nplot ]) != ($$bpf_numVars_ref[ $nplot ] * $$bpf_dataSize_ref[ $nplot ]))
    {
      print "files $CIRFILEwithPrintRaw.ascii.raw $CIRFILEwithPrintRaw.raw had different numbers of variables in plot section $nplot.\n";
      $retval = 2;
      print "Exit code = $retval\n";
      exit $retval;
    }
    for ( $nvars = 0; $nvars < ($$apf_numVars_ref[ $nplot ] * $$apf_dataSize_ref[ $nplot ]); $nvars++ )
    {
      my $absError = $apf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ] - $bpf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ];
      
      my $sumVal = ($apf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ] - $bpf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ]);
      if ( $sumVal < $absTol )
      {
        $sumVal = 1.0;
      }
      my $relError = $absError / $sumVal;
      if ( (abs($absError) > $absTol) || (abs($relError) > $relTol) )
      {
        print " Values failed compare in comparing print format raw files in ascii and binary format\n";
        print " values were  $nvars : \"$apf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ]\" ?=? \"$bpf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ]\" \n";
        print "abserror = $absError abstol = $absTol\n";
        print "relerror = $relError reltol = $relTol \n";
        $retval=2;
        print "Exit code = $retval\n";
        exit $retval;
      }       
    }
  }
}



# now check the raw files produced with the "-r" command line option 
# this option dumps all of the simulation variables to a file.
# check the file $CIRFILEwithPrintRaw.ascii.raw
# read ascii print format raw file variables prefixed with arf_
($retVal, $arf_title, $arf_date, $arf_plotname_ref, $arf_dataSize_ref, $arf_numVars_ref, $arf_varNames_ref, $arf_numPoints_ref, $arf_data_ref ) = readRawFile("$CIRFILE.ascii.raw" );
if( $retval != 0 )
{
  print "file $CIRFILE.ascii.raw failed read.\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# read binary print format raw file variables prefixed with brf_
($retVal, $brf_title, $brf_date, $brf_plotname_ref, $brf_dataSize_ref, $brf_numVars_ref, $brf_varNames_ref, $brf_numPoints_ref, $brf_data_ref ) = readRawFile("$CIRFILE.raw" );
if( $retval != 0 )
{
  print "file $CIRFILE.raw failed read.\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# compare plots and data from the two print format raw files
if( @$arf_plotname_ref != @$brf_plotname_ref )
{
  print "files $CIRFILE.ascii.raw $CIRFILE.raw had different numbers of plot sections.\n";
  $retval = 2;
  print "Exit code = $retval\n";
  exit $retval;
}

for ( $nplot = 0; $nplot < @$arf_plotname_ref; $nplot++ )
{
  #print "Plot \"$$plotname_ref[$nplot]\"\n";
  if( $$arf_numPoints_ref[ $nplot ] != $$brf_numPoints_ref[ $nplot ] )
  {
    print "files $CIRFILE.ascii.raw $CIRFILE.raw had different numbers of points in plot section $nplot.\n";
    $retval = 2;
    print "Exit code = $retval\n";
    exit $retval;
  }
  for ( $npoints = 0; $npoints < $$arf_numPoints_ref[ $nplot ]; $npoints++ )
  {
    if( ($$arf_numVars_ref[ $nplot ] * $$arf_dataSize_ref[ $nplot ]) != ($$brf_numVars_ref[ $nplot ] * $$brf_dataSize_ref[ $nplot ]))
    {
      print "files $CIRFILE.ascii.raw $CIRFILE.raw had different numbers of variables in plot section $nplot.\n";
      $retval = 2;
      print "Exit code = $retval\n";
      exit $retval;
    }
    for ( $nvars = 0; $nvars < ($$arf_numVars_ref[ $nplot ] * $$arf_dataSize_ref[ $nplot ]); $nvars++ )
    {
      my $absError = $arf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ] - $brf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ];
      
      my $sumVal = ($arf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ] - $brf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ]);
      if ( $sumVal < $absTol )
      {
        $sumVal = 1.0;
      }
      my $relError = $absError / $sumVal;
      if ( (abs($absError) > $absTol) || (abs($relError) > $relTol) )
      {
        print " Values failed compare in comparing command line format raw files in ascii and binary format\n";
        print " values were  $nvars : \"$arf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ]\" ?=? \"$brf_data_ref->[ $nplot ]->[ $npoints ]->[ $nvars ]\"\n";
        print "abserror = $absError abstol = $absTol\n";
        print "relerror = $relError reltol = $relTol \n";
        $retval=2;
        print "Exit code = $retval\n";
        exit $retval;
      }       
    }
  }
}


if ($retval != 0)
{
  print "Exit code = $retval\n";
  exit $retval;
}
print "Exit code = 0\n";
exit 0;


