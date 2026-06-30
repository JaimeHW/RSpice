#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# these search strings are supposed to occur one right after the other in the
# error output.  
@searchstrings = (
"Netlist error in file improperVectorParams.cir at or near line 20",
".SENS line is mis-formatted. Possibly a missing comma in vector parameter",
"Netlist warning in file improperVectorParams.cir at or near line 20",
" Unrecognized dot line will be ignored",
"Netlist error in file improperVectorParams.cir at or near line 21",
".SENS line is mis-formatted. Possibly a missing comma in vector parameter",
"Netlist warning in file improperVectorParams.cir at or near line 21",
"Unrecognized dot line will be ignored",
"Netlist error in file improperVectorParams.cir at or near line 22",
".SENS line is mis-formatted. Vector parameter begins with a comma",
"Netlist warning in file improperVectorParams.cir at or near line 22",
"Unrecognized dot line will be ignored",
"Netlist error in file improperVectorParams.cir at or near line 23",
".SENS line is mis-formatted. Vector parameter begins with a comma",
"Netlist warning in file improperVectorParams.cir at or near line 23",
"Unrecognized dot line will be ignored",
"Netlist error in file improperVectorParams.cir at or near line 25",
".SENS line is mis-formatted. Possibly a missing comma in vector parameter",
"Netlist warning in file improperVectorParams.cir at or near line 25",
" Unrecognized dot line will be ignored",
"Netlist error in file improperVectorParams.cir at or near line 26",
" .SENS line is mis-formatted. Error parsing vector parameter",
"Netlist warning in file improperVectorParams.cir at or near line 26",
"Unrecognized dot line will be ignored",
"Netlist error in file improperVectorParams.cir at or near line 27",
".SENS line is mis-formatted. Vector parameter begins with a comma",
"Netlist warning in file improperVectorParams.cir at or near line 27",
" Unrecognized dot line will be ignored",
"Netlist error in file improperVectorParams.cir at or near line 28",
" .SENS line is mis-formatted. Error parsing vector parameter",
"Netlist warning in file improperVectorParams.cir at or near line 28",
"Unrecognized dot line will be ignored"
);

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n";
exit $retval

