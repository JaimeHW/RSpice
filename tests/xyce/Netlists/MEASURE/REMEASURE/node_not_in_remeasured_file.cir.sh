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

# Check various error cases
# These strings should be in the output of this failed Xyce run.
# Note that parens are escaped with \\ while _ is escaped with \ 
@searchstrings = ( "In OutputMgr::remeasure",
                   "file to reprocess through measure and/or fft functions: node\_not\_in\_file.prn",
                   "Netlist error: Function or variable V\\(2\\) is not defined",
                    "Netlist error: Function or variable N\\(3\\) is not defined",
                   "Netlist error: Function or variable V\\(4\\) is not defined",
                   "Netlist error: Function or variable V\\(6\\) is not defined");

# turn this into a re-measure line
$XYCE = "$XYCE -remeasure node_not_in_file.prn";
$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
