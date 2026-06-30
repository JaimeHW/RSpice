#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);

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

if (defined($verbose)) { $Tools->setVerbose(1); }

# check various error cases
# this string should be in the output of this failed Xyce run
@searchstrings = (
"Netlist error in file unsupported_cont_measure_types.cir at or near line 24",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 25",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 26",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 27",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 28",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 29",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 30",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 31",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 32",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 33",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 34",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 35",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 36",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 37",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
"Netlist error in file unsupported_cont_measure_types.cir at or near line 38",
  "Only DERIV, FIND, TRIG, TARG and WHEN measure types are supported for",
  "continuous \\(CONT\\) measure modes",
);

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval;
