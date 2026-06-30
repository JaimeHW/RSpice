#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce the appropriate output files, then we return exit code 14.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.csv.out and the STDERR output from
# comparison to go into $CIRFILE.csv.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;

$GOLDFFT = $GOLDPRN;
$GOLDFFT =~ s/\.prn$//;

system("rm -f $CIRFILE.fft*");

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) {print "Exit code = $retval\n"; exit $retval;}

if ( !(-f "$CIRFILE.fft0")) {
    print STDERR "Missing output file $CIRFILE.fft0\n";
    print "Exit code = 2\n";
    exit 2;
}

# Verify that no verbose output is in the stdout.
# Check that .out file exists, and open it if it does.
if (not -s "$CIRFILE.out" )
{
  print "Exit code = 17\n";
  exit 17;
}

$stdouterr = `grep "FFT Analyses" $CIRFILE.out`;
if ( $stdouterr != 0 )
{
  print ".OPTIONS FFTOUT text found in stdout, when it should not be\n"; 
  print "Exit code = 2\n"; exit 2;
}

# check the .fft0 output
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;

$CMD="$fc $CIRFILE.fft0 $GOLDFFT.fft0 $absTol $relTol $zeroTol > $CIRFILE.fft0.out 2> $CIRFILE.fft0.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIRFILE.fft0\n";
  print "Exit code = 2\n";
  exit 2;
}

# also check warning messages in stdout
@searchstrings = ("Netlist warning: FFT_ACCURATE values of 0 or 1 are supported.  Setting to",
  "default of 1",
  "Netlist warning: FFT_MODE values of 0 or 1 are supported.  Setting to default",
  "of 0",
  "Netlist warning: NP value on .FFT line reset to minimum value of 4",
  "Netlist warning: NP value on .FFT line not equal to power of 2.  Setting to 8",
  "Netlist warning: NP value on .FFT line not equal to power of 2.  Setting to 16",
  "Netlist warning: Invalid start time on .FFT line, reset to transient start",
  "time of 0",
  "Netlist warning: Invalid start time on .FFT line, reset to transient start",
  "time of 0",
  "Netlist warning: Invalid stop time on .FFT line, reset to transient stop time",
  "of 1",
  "Netlist warning: Invalid stop time on .FFT line, reset to transient stop time",
  "of 1",
  "Netlist warning: Invalid stop time on .FFT line, reset to transient stop time",
  "of 1"
 );

$retval = $Tools->checkError("$CIRFILE.out",@searchstrings);
print "Exit code = $retval\n"; exit $retval;
