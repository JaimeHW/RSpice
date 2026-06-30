#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# remove old HOMOTOPY.prn file if it exists
`rm -f $CIRFILE.HOMOTOPY.prn $CIRFILE.prn`;
`rm -f $CIRFILE.homotopy.out $CIRFILE.homotopy.err`;

# run Xyce
$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# Check for the output files
if (not -s "$CIRFILE.HOMOTOPY.prn" )
{
  print "$CIRFILE.HOMOTOPY.prn file is missing\n"; 
  print "Exit code = 14\n"; 
  exit 14;
}

if ( not -s "$CIRFILE.HOMOTOPY.ts1")
{
    print STDERR "Missing output file $CIRFILE.HOMOTOPY.ts1\n";
    print "Exit code = 14\n"; exit 14;
}

if ( not -s "$CIRFILE.HOMOTOPY.ts2")
{
    print STDERR "Missing output file $CIRFILE.HOMOTOPY.ts2\n";
    print "Exit code = 14\n"; exit 14;
}

# these strings should be in the output of this successful Xyce run
@searchstrings = ("Netlist warning: Homotopy output cannot be written in PROBE, RAW or Touchstone",
                  "format, using standard format instead",
                  "Netlist warning: Homotopy output cannot be written in PROBE, RAW or Touchstone",
                  "format, using standard format instead",
                  "Netlist warning: Homotopy output cannot be written in PROBE, RAW or Touchstone",
                  "format, using standard format instead"
);

$retval = $Tools->checkError("$CIRFILE.out",@searchstrings);
if ($retval != 0)
{
  print "Check on warning message failed\n";
  print "Exit code = $retval\n"; exit $retval; 
} 

# Now check the .HOMOTOPY.prn file
print "Now checking output files\n";

$retcode=0;
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$GOLDPRN = substr($GOLDPRN,0,-3)."HOMOTOPY.prn";
$CMD="$fc $CIRFILE.HOMOTOPY.prn $GOLDPRN $absTol $relTol $zeroTol > $CIRFILE.homotopy.out 2> $CIRFILE.homotopy.err";
$retval = system("$CMD");
if ( $retval != 0 ){
  print STDERR "Comparator exited with exit code $retval on file $CIRFILE.HOMOTOPY.prn\n";
  $retcode = 2;
}

# The other two output files should be identical to $CIRFILE.HOMOTOPY.prn
$CMD="diff $CIRFILE.HOMOTOPY.prn $CIRFILE.HOMOTOPY.ts1 > $CIRFILE.HOMOTOPY.ts1.out";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.HOMOTOPY.ts1, see $CIRFILE.HOMOTOPY.ts1\n";
    $retcode = 2;
}

$CMD="diff $CIRFILE.HOMOTOPY.prn $CIRFILE.HOMOTOPY.ts2 > $CIRFILE.HOMOTOPY.ts2.out";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.HOMOTOPY.ts2, see $CIRFILE.HOMOTOPY.ts2\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;



