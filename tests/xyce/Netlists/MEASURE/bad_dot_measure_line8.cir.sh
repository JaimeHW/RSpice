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
# these strings should be in the output of this failed Xyce run
@searchstrings = ("Netlist error: Too many dependent variables for AVG measure, \"AVG\"",
      "Netlist error: Too many dependent variables for DUTY measure, \"DUTY\"",
      "Netlist error: Too many dependent variables for EQN measure, \"EQN\"",
      "Netlist error: Too many dependent variables for ERROR measure, \"ERROR\"",
      "Netlist error: Too many dependent variables for FOUR measure, \"FOUR\"",
      "Netlist error: Too many dependent variables for FREQ measure, \"FREQ\"",
      "Netlist error: Too many dependent variables for INTEG measure, \"INTEG\"",
      "Netlist error: Too many dependent variables for MAX measure, \"MAX\"",
      "Netlist error: Too many dependent variables for MIN measure, \"MIN\"",
      "Netlist error: Too many dependent variables for OFF_TIME measure, \"OFF_TIME\"",
      "Netlist error: Too many dependent variables for ON_TIME measure, \"ON_TIME\"",
      "Netlist error: Too many dependent variables for PP measure, \"PP\"",
      "Netlist error: Too many dependent variables for RMS measure, \"RMS\"",
      "Netlist error: Too many dependent variables for MAX measure, \"M1\"",
      "Netlist error: Too many dependent variables for MAX measure, \"M2\""
);

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval;
