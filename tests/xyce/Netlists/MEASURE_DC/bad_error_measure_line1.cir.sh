#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

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

# check various error cases.
# these strings should be in the output of this failed Xyce run.  
@searchstrings = ("Netlist error: In measure DEPVARCOLINVALID, using data from file",
   "ErrorTest1DCRawData.prn. Requested column for dependent variable 10 does not",
   "exist in the data file for entry 0",
   "Netlist error: In measure DEPVARCOLMISSING, missing or negative value for",
   "DEPVARCVOL",
   "Netlist error: In measure DEPVARCOLNEGATIVE, missing or negative value for",
   "DEPVARCVOL"
);

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
