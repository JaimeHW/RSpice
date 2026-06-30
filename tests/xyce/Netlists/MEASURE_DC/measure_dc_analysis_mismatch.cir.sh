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
# These strings should be in the output of this failed Xyce run. 
# Note that ( ) and : characters must be escaped with \\
@searchstrings = ("Netlist error\\: Netlist analysis statement and measure mode \\(TRAN\\) for measure",
   "TRANMAX do not agree",
   "Netlist error: Netlist analysis statement and measure mode \\(TRAN_CONT\\) for",
   "measure TRAN_CONT_WHEN do not agree",
   "Netlist error\\: Netlist analysis statement and measure mode \\(AC\\) for measure",
   "ACERROR do not agree",
   "Netlist error: Netlist analysis statement and measure mode \\(AC_CONT\\) for",
   "measure AC_CONT_FIND do not agree",
   "Netlist error\\: Netlist analysis statement and measure mode \\(NOISE\\) for measure",
   "NOISEMAX do not agree",
   "Netlist error: Netlist analysis statement and measure mode \\(NOISE_CONT\\) for",
   "measure NOISE_CONT_FIND do not agree",
   "Netlist error: Netlist analysis statement and measure mode \\(FFT\\) for measure",
   "THD do not agree"
 );

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
