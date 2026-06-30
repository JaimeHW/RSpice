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

# check various error cases
# this string should be in the output of this failed Xyce run
@searchstrings = ( "Netlist error in file bad_dot_dc_line.cir at or near line 17",
                   "Extraneous values on .DC line",
                   "Netlist error in file bad_dot_dc_line.cir at or near line 20",
                   "Extraneous values on .DC line",
                   "Netlist error in file bad_dot_dc_line.cir at or near line 24",
                   ".DC line not formatted correctly, found unexpected number of fields",
                   "Netlist error in file bad_dot_dc_line.cir at or near line 25",
                    "Unrecognized dot line will be ignored",
                   "Netlist error in file bad_dot_dc_line.cir at or near line 26",
                   ".DC line has an unexpected number of fields",
                   "Netlist warning in file bad_dot_dc_line.cir at or near line 26",
                   "Unrecognized dot line will be ignored",
                   "Netlist error in file bad_dot_dc_line.cir at or near line 27",
                   ".DC line has an unexpected number of fields",
                   "Netlist warning in file bad_dot_dc_line.cir at or near line 27",
                   "Unrecognized dot line will be ignored",
                   "Netlist error in file bad_dot_dc_line.cir at or near line 28",
                   ".DC line has an unexpected number of fields",
                   "Netlist warning in file bad_dot_dc_line.cir at or near line 28",
                   "Unrecognized dot line will be ignored",
                   "Netlist error in file bad_dot_dc_line.cir at or near line 29",
                   ".DC line has an unexpected number of fields",
                   "Netlist warning in file bad_dot_dc_line.cir at or near line 29",
                   "Unrecognized dot line will be ignored"
);

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
