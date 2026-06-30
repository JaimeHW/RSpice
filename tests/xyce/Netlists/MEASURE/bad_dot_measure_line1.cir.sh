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
# Note that underscore (_) characters must be escaped with \\
@searchstrings = ( "Netlist error in file bad_dot_measure_line1.cir at or near line 21",
      "Illegal name in .MEASURE line.  Cannot be AC, DC, FFT, NOISE, TRAN/TR,",
      "AC_CONT, DC_CONT, NOISE_CONT or TRAN_CONT",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 22",
      "Unknown mode in .MEASURE line.  Should be TRAN/TR, DC, AC, FFT, NOISE,",
      "TRAN_CONT, DC_CONT, AC_CONT or NOISE_CONT",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 23",
      "Too few items on .MEASURE line.  Need at least .MEASURE <mode> <name> <type>",
      "Netlist warning in file bad_dot_measure_line1.cir at or near line 23",
      "Unrecognized dot line will be ignored",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 24",
      "Too few items on .MEASURE line.  Need at least .MEASURE <mode> <name> <type>",
      "Netlist warning in file bad_dot_measure_line1.cir at or near line 24",
      "Unrecognized dot line will be ignored",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 25",
      "Too few items on .MEASURE line.  Need at least .MEASURE <mode> <name> <type>",
      "Netlist warning in file bad_dot_measure_line1.cir at or near line 25",
      "Unrecognized dot line will be ignored",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 26",
      "Invalid VAL= syntax in TRIG or TARG block on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 27",
      "Invalid VAL= syntax in TRIG or TARG block on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 28",
      "Unknown mode in .MEASURE line.  Should be TRAN/TR",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 31",
      "Illegal type in .MEASURE line for TRAN mode.  Must be one of: AVG,",
      "DERIV/DERIVATIVE, DUTY, EQN/PARAM, ERR, ERR1, ERR2, ERROR, FIND, WHEN, FOUR",
      "INTEG/INTEGRAL, MIN, MAX, OFF\\_TIME, ON\\_TIME, PP, RMS, TRIG, TARG",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 34",
      "Unmatched expression delimiter on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 35",
      "Unmatched expression delimiter on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 38",
      "Invalid PAR\\(\\) expression syntax on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 39",
      "Invalid PAR\\(\\) expression syntax on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 40",
      "Invalid PAR\\(\\) expression syntax on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 41",
      "Invalid PAR\\(\\) expression syntax on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 44",
      "Invalid \\(\\) expression syntax on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 45",
      "Invalid \\(\\) expression syntax on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 46",
      "Invalid \\(\\) expression syntax on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 47",
      "Invalid \\(\\) expression syntax on .MEASURE line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 50",
      "Incomplete .MEASURE line.  TD, GOAL, WEIGHT, MINVAL, AT, TO, FROM, ON, OFF,",
      "IGNORE, YMIN, YMAX must be followed by a value",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 51",
      "Incomplete .MEASURE line.  TD, GOAL, WEIGHT, MINVAL, AT, TO, FROM, ON, OFF,",
      "IGNORE, YMIN, YMAX must be followed by a value",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 52",
      "Incomplete .MEASURE line.  TD, GOAL, WEIGHT, MINVAL, AT, TO, FROM, ON, OFF,",
      "IGNORE, YMIN, YMAX must be followed by a value",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 53",
      "Invalid Measure Line",
      "Netlist error in file bad_dot_measure_line1.cir at or near line 56",
      "Unknown mode in .MEASURE line.  Should be TRAN/TR, DC, AC, FFT, NOISE,",
      "TRAN_CONT, DC_CONT, AC_CONT or NOISE_CONT"
 );

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
