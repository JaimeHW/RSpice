#!/usr/bin/env perl

use XyceRegression::Tools;
use Scalar::Util qw(looks_like_number);

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$CIRFILE=$ARGV[3];

# these are the tolerances used in comparing the .output file
# and the Gold Standard
my $absTol = 1.0e-10;
my $absTolFail = 1.0e-19; # must the same as TJABSTOL in .cir file
my $relTol = 1.0e-5; # must be the same as TJRELTOL in .cir file

# generate .prn and .out files           
$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

# make .output file from .out file.  Only include info relevant to testjac feature
system("grep -f $CIRFILE.patfile $CIRFILE.out > $CIRFILE.output");

# open files and check that the files have the same number of lines
if (not -s "$CIRFILE.output" ) { print STDERR "Failed to make output file with jactest info: $CIRFILE.output\nExit code = 2\n"; exit 2; }
if (not -s "$CIRFILE.prn" ) { print STDERR "Missing .prn file: $CIRFILE.prn\nExit code = 14\n"; exit 14; }
if (not -s "$CIRFILE.gold_output" ) { print STDERR "Missing Gold Output file: $CIRFILE.gold_output\nExit code = 2\n"; exit 2; }

# check the .output and Gold output file have the same number of lines
open(GSFILE,"$CIRFILE.gold_output");
open(OUTPUTFILE,"$CIRFILE.output");

for ($outputCount=0; <OUTPUTFILE>; $outputCount++) { }
for ($gsCount=0; <GSFILE>; $gsCount++) { }

close(GSFILE);
close(OUTPUTFILE);

$retval=0;
if ($outputCount != $gsCount) 
{
    print STDERR ".output file doesn't match the .gold_output File\n";
    print STDERR ".output line count= $outputCount\n";
    print STDERR ".gold_output line count= $gsCount\n";
    $retval=2;
}
else
{  # If the line counts match, then compare in detail.
   # Re-open the files and compare them line by line, using absTol
    open(GSFILE,"$CIRFILE.gold_output");
    open(OUTPUTFILE,"$CIRFILE.output");
    $lineCount=0;
    while( ($lineGS=<GSFILE>) && ($lineOUTPUT=<OUTPUTFILE>) )
    {
	 $lineCount++;
         # process a line into text and values.
         chop $lineGS;
         # Remove leading spaces on lineGS, otherwise the spaces become 
         # element 0 of "@gsData" instead of the first column of data.
         $lineGS =~ s/^\s*//;
         @gsData = (split(/[\s,]+/, $lineGS));

         # process a line into text and values.
         chop $lineOUTPUT;
         # Remove leading spaces on line, otherwise the spaces become 
         # element 0 of "outputData" instead of the first column of data.
	 $lineOUTPUT =~ s/^\s*//;
         @outputData = (split(/[\s,]+/, $lineOUTPUT));
             
         if ($outputData[0] ne "FT:")
         {
           # do these tests for header lines (which should match exactly) and QT lines
           # which should be all zero and identical
           if ($#gsData != $#outputData)
           {
             print STDERR ".output file and Gold Standard have a different # of elements on line $lineCount:\n";
             print STDERR ".output file had $#outputData elements\n";
             print STDERR ".gold_output file had $#gsData elements\n";
             $retval=2;
           }
           else
           {
             # the two files have the same number of items on a line.  
             # compare individual values as scalars.
             for( $i=0; $i<=$#outputData; $i++ )
             {
                 if ( looks_like_number($outputData[$i]) && looks_like_number($gsData[$i]) )
                 {
                     if( abs( $outputData[$i] - $gsData[$i] ) < $absTol )
                     {
                         # two numeric fields match to within specified absolute tolerance
                     }
                     else
                     {
                        print STDERR "Out of tolerance in column $i on line $lineCount, with abstol = $absTol and reltol = $relTol\n"; 
                        print STDERR ".output file had \"$outputData[$i]\" while .gold_output had \"$gsData[$i]\" \n";
                        $retval=2;
                        last;
                     }
                 }
                 elsif( ($outputData[$i] eq $gsData[$i]) ) 
                 {
                     # two string fields match
                 }
                 else
                 {
                     print STDERR "String elements failed compare on line $lineCount:\n";
                     print STDERR ".output file produced: \"$outputData[$i]\" \n";
                     print STDERR ".gold_output was: \"$gsData[$i]\" \n";
                     $retval=2;
                     last;
                 }                  
             } 
           }
         }
         else
         {
           # For FT lines now check that a "pass" line should have passed and a "fail" line
           # (in the Status column) should have failed.  This uses the absTolFail metric.
           if ( $outputData[5] eq "fail" )
           {
             if ( $outputData[3] < $absTolFail )
             {
               # check absDiff column in Status colunm with a "fail" entry
               print STDERR "Line $lineCount in .output file has \"fail\" when it should have passed:\n";
               $retval=2;
             }
           }
	   elsif ( $outputData[3] > $absTolFail )
           {
             # a "pass" line will have a blank in the status column. check absDiff column 
             print STDERR "Line $lineCount in .output file should not have passed:\n";
             $retval=2;
           }       
         }
     }

     # close files
     close(GSFILE);
     close(OUTPUTFILE);
}

# print success or failure
if ( $retval != 0 )
{
     print "Exit code = $retval\n"; 
     exit $retval;
}
else            
{
     print "Exit code = $retval\n"; 
     exit $retval;
}
