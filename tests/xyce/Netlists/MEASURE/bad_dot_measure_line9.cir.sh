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
@searchstrings = ( "Netlist error: ONEA has invalid MEASURE line",
                   "Netlist error: ONEB has invalid MEASURE line",
                   "Netlist error: ONEC has invalid MEASURE line",
                   "Netlist error: ONED has invalid MEASURE line",
                   "Netlist error: TWOA has invalid MEASURE line",
                   "Netlist error: TWOB has invalid MEASURE line",
                   "Netlist error: THREE has invalid MEASURE line",
                   "Netlist error: FOUR has invalid MEASURE line",
  "Netlist error:  RISE, FALL or CROSS values < -1 not supported for measure",
  "CROSSMINUS2 for AC, DC, NOISE or TRAN measures",
  "Netlist error:  RISE, FALL or CROSS values < -1 not supported for measure",
  "RISEMINUS2 for AC, DC, NOISE or TRAN measures",
  "Netlist error:  RISE, FALL or CROSS values < -1 not supported for measure",
  "FALLMINUS2 for AC, DC, NOISE or TRAN measures",
  "Netlist error:  RISE, FALL or CROSS values < -1 not supported for measure",
  "CROSSMINUS2A for AC, DC, NOISE or TRAN measures",
  "Netlist error:  RISE, FALL or CROSS values < -1 not supported for measure",
  "RISEMINUS2A for AC, DC, NOISE or TRAN measures",
  "Netlist error:  RISE, FALL or CROSS values < -1 not supported for measure",
  "FALLMINUS2A for AC, DC, NOISE or TRAN measures",
  "Netlist error:  RISE, FALL or CROSS values < 0 not supported for measure",
  "CROSSMINUS1 for AC_CONT, DC_CONT, NOISE_CONT or TRAN_CONT measures",
  "Netlist error:  RISE, FALL or CROSS values < 0 not supported for measure",
  "RISEMINUS1 for AC_CONT, DC_CONT, NOISE_CONT or TRAN_CONT measures",
  "Netlist error:  RISE, FALL or CROSS values < 0 not supported for measure",
  "FALLMINUS1 for AC_CONT, DC_CONT, NOISE_CONT or TRAN_CONT measures",
  "Netlist error:  RISE, FALL or CROSS values < 0 not supported for measure",
  "CROSSMINUS1A for AC_CONT, DC_CONT, NOISE_CONT or TRAN_CONT measures",
  "Netlist error:  RISE, FALL or CROSS values < 0 not supported for measure",
  "RISEMINUS1A for AC_CONT, DC_CONT, NOISE_CONT or TRAN_CONT measures",
  "Netlist error:  RISE, FALL or CROSS values < 0 not supported for measure",
  "FALLMINUS1A for AC_CONT, DC_CONT, NOISE_CONT or TRAN_CONT measures",
);

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
