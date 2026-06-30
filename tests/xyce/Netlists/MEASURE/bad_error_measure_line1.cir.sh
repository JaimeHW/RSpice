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
@searchstrings = ("Netlist error: In measure INDEPVARCOLINVALID, using data from file",
   "ErrorTestRawDataMisOrdered.prn. Requested column for independent variable 1",
   "or dependent variable 4 does not exist in the data file for entry 0",
   "Netlist error: In measure DEPVARCOLINVALID, using data from file",
   "ErrorTestRawDataMisOrdered.prn. Requested column for independent variable 4",
   "or dependent variable 2 does not exist in the data file for entry 0",
   "Netlist error: In measure TIMEMISORDRED, using data from file",
   "ErrorTestRawDataMisOrdered.prn. Independent variables are not sorted in",
   "monotonically increasing order at entry 2",
   "Netlist error: In measure TIMEMISORDRED, using data from file",
   "ErrorTestRawDataMisOrdered.prn. Independent variables are not sorted in",
   "monotonically increasing order at entry 10",
   "Netlist error: In measure INDEPVARCOLNEGATIVE, missing or negative value for",
   "INDEPVARCOL or DEPVARCVOL",
   "Netlist error: In measure DEPVARCOLNEGATIVE, missing or negative value for",
   "INDEPVARCOL or DEPVARCVOL",
   "Netlist error: In measure INDEPVARCOLMISSING, missing or negative value for",
   "INDEPVARCOL or DEPVARCVOL",
   "Netlist error: In measure DEPVARCOLMISSING, missing or negative value for",
   "INDEPVARCOL or DEPVARCVOL",
   "Netlist error: In measure IDENTICALVARCOLVALUES, identical values for",
   "INDEPVARCOL and DEPVARCVOL",
   "Netlist error: In measure BOGOFILE, could not find comparison file",
   "Netlist error: In measure FILEISDIR, could not find comparison file",
   "Netlist error: In measure SHORTFILENAME, comparison filename must end in .PRN,",
   ".CSV or .CSD",
   "Netlist error: For measure UNSUPPORTEDEXT, ERROR measure only supports .PRN",
   ".CSV or .CSD formats"
 );

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
