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
# Note that parens are escaped with \\ 
@searchstrings = ( "Netlist error in file bad_dot_four_line1.cir at or near line 21",
                   "Error: .FOUR line needs at least 3 arguments '.FOUR freq ov1 <ov2 ...>'",
                   "Netlist warning in file bad_dot_four_line1.cir at or near line 21",
                   "Unrecognized dot line will be ignored",
                   "Netlist error in file bad_dot_four_line1.cir at or near line 22",
                   "Error: .FOUR line needs at least 3 arguments '.FOUR freq ov1 <ov2 ...>'",
                   "Netlist warning in file bad_dot_four_line1.cir at or near line 22",
                   "Unrecognized dot line will be ignored",
                   "Netlist error in file bad_dot_four_line1.cir at or near line 23",
                   "Could not parse .FOUR variable \\(",
                   "Netlist error in file bad_dot_four_line1.cir at or near line 23",
                   "Could not parse .FOUR variable \\)",
                   "Netlist error in file bad_dot_four_line1.cir at or near line 24",
                   " Error: .FOUR line needs at least 3 arguments '.FOUR freq ov1 <ov2 ...>'",
                   "Netlist warning in file bad_dot_four_line1.cir at or near line 24",
                   "Unrecognized dot line will be ignored",
                   "Netlist error in file bad_dot_four_line1.cir at or near line 25",
                   "Error: .FOUR line needs at least 3 arguments '.FOUR freq ov1 <ov2 ...>'",
                   "Netlist warning in file bad_dot_four_line1.cir at or near line 25",
                   "Unrecognized dot line will be ignored",
                   "Netlist error in file bad_dot_four_line1.cir at or near line 26",
                   "Could not parse .FOUR variable \\{",
                   "Netlist error in file bad_dot_four_line1.cir at or near line 27",
                   "Could not parse .FOUR variable }",
                   "Netlist error in file bad_dot_four_line1.cir at or near line 28",
                   "Could not parse .FOUR variable \\{V\\(2\\)",
                   "Netlist error in file bad_dot_four_line1.cir at or near line 29",
                   "Could not parse .FOUR variable",
                   "Netlist error in file bad_dot_four_line1.cir at or near line 30",
                   "Could not parse .FOUR variable 2" );

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
