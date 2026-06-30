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
#$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
#$GOLDPRN=$ARGV[4];

# Check various error cases.
# These strings should be in the output of this failed Xyce run.
# Note: must escape ( ) and * and + with \\ to get the CheckError() function to work.  
@searchstrings = ( 
  ["Netlist error: Wrong number of arguments for user defined function",
   "UDFA\\(MU,SIGMA\\) in expression \\{\\(V\\(2\\)\\+udfA\\(10m,1u,23\\)\\)/50\\}"],
  ["Netlist error: Wrong number of arguments for user defined function",
   "UDFA\\(MU,SIGMA\\) in expression \\{udfA\\(10m,1u,23\\)/50\\}"],
  ["Netlist error: Wrong number of arguments for user defined function",
   "UDFA\\(MU,SIGMA\\) in expression \\{udfA\\(10m,1u,23\\)\\+V\\(2\\)\\}"],
  ["Netlist error: Wrong number of arguments for user defined function",
   "UDFA\\(MU,SIGMA\\) in expression \\{udfA\\(10m,1u,23\\)\\}"],
  ["Netlist error: Wrong number of arguments for user defined function",
   "UDFB\\(MU,SIGMA,T\\) in expression \\{\\(V\\(2\\)\\+udfB\\(10m,1u\\)\\)/50\\}"],
  ["Netlist error: Wrong number of arguments for user defined function",
   "UDFB\\(MU,SIGMA,T\\) in expression \\{\\(udfB\\(10m,1u\\)\\+V\\(2\\)\\)/50\\}"],
  ["Netlist error: Wrong number of arguments for user defined function",
  "UDFB\\(MU,SIGMA,T\\) in expression \\{\\(udfB\\(20m,1u\\)\\)/50\\}"]
   );

$retval = $Tools->runAndCheckGroupedError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
