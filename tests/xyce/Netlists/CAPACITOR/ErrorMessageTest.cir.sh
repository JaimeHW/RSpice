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
# this string should be in the output of this failed Xyce run  .
# Note: must escape ( ) and * with \\ to get the CheckError() function to work.
@searchstrings = (["Netlist error in file ErrorMessageTest.cir at or near line 11",
     "Device instance C1: Could find neither C, Q, nor L parameters in instance."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 17",
     "Device instance C2: Could find neither C, Q, nor L parameters in instance."],
    [ "Netlist error in file ErrorMessageTest.cir at or near line 24",
     "Device instance C3: Could find neither C, Q, nor L parameters in instance.",
     "Netlist error in file ErrorMessageTest.cir at or near line 24",
     "Device instance C3: Age \\(A\\) defined, but no C instance parameter given. Can't",
     "use age with semiconductor capacitor options."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 29",
     "Device instance C4: Could find neither C, Q, nor L parameters in instance."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 35",
     "Device instance C5: Age \\(A\\) defined, but no C instance parameter given. Can't",
     "use age with semiconductor capacitor options."],
    ["Netlist error in file ErrorMessageTest.cir at or near line 41",
     "Device instance C6: Illegal use of lead current specification in expression",
     "'\\{1e-6\\*I\\(R9\\)\\}' in parameter C"],
    ["Netlist error in file ErrorMessageTest.cir at or near line 47",
     "Device instance C7: Dependent expression \\{1e-6\\*ddt\\(V\\(9\\)\\)\\} for parameter C",
     "contains time derivatives"],
    ["Netlist error in file ErrorMessageTest.cir at or near line 53",
     "Device instance C8: Parameter L is not allowed to depend on voltage/current",
     "values"],
    ["Netlist error in file ErrorMessageTest.cir at or near line 62",
     "Device instance C10: Both C and Q have been specified as expression",
     "parameters.  Only one may be specified at a time"],
    ["Netlist error in file ErrorMessageTest.cir at or near line 67",
     "Device instance C11: Q has been specified as an expression parameter and an",
     "IC given.  IC with Q specified is not implemented"]);

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
