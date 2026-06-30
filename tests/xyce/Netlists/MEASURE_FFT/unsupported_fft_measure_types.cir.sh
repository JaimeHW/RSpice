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
@searchstrings = ("Netlist error in file unsupported_fft_measure_types.cir at or near line 21",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 22",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 23",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 24",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 25",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 26",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 27",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 28",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 29",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 30",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 31",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 32",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 33",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 34",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode",
   "Netlist error in file unsupported_fft_measure_types.cir at or near line 35",
   "Only ENOB, EQN/PARAM, FIND, SDFR, SNDR, SNR and THD measure types are",
   "supported for FFT measure mode"
 );

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval;
