#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

use MeasureCommon;
use File::Copy;

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
$GOLDPRN=$ARGV[4];

# check various error cases
# this string should be in the output of this failed Xyce run  
@searchstrings = ("Please comment out the .OP statement to get remeasure to work");

# measure should work.  Just test that it runs and makes the .ms0 file
MeasureCommon::checkDCFilesExist($XYCE,$CIRFILE);

# if checkDCFilesExist didn't exit with an error then we reach here.
# Re-name the files from the measure run, so that the
# data from the re-measure run does not overwrite them
print "Measure was successful\n";
move("$CIRFILE.ms0","$CIRFILE.measure.ms0");
move("$CIRFILE.out","$CIRFILE.measure.out");

print "Testing error measure for remeasure\n";
# Turn this into a re-measure line.  The name of the -prn file 
# does not matterfor this test.
$XYCE = "$XYCE -remeasure bogoFile.prn ";
$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
