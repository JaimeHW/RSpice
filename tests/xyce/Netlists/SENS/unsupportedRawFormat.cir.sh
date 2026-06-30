#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

# these search strings are supposed to occur one right after the other in the
# error output.
@searchstrings = ("Sensitivity output cannot be written in PROBE, RAW or Touchstone format, using standard format instead",
);

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# remove old SENS.prn files if they exists
`rm -f $CIRFILE.SENS.prn $CIRFILE.raw`;
`rm -f $CIRFILE.sens.out $CIRFILE.sens.err`;

$XYCE="$XYCE -error-test";

$retval = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);
if ($retval != 0)
{
  print "Exit code = $retval\n";
  exit $retval
}

# Exit if the SENS.prn file was not made
if (not -s "$CIRFILE.SENS.prn" )
{
  print "$CIRFILE.SENS.prn file is missing\n"; 
  print "Exit code = 14\n"; 
  exit 14;
}

# Now check that the .SENS.prn file defaults to FORMAT=STD
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$CMD="$fc $CIRFILE.SENS.prn $CIRFILE.SENS.prn.gs $absTol $relTol $zeroTol > $CIRFILE.sens.out 2> $CIRFILE.sens.err";
$retval = system("$CMD");
if ( $retval != 0 )
{
  print "Test failed comparison of SENS.prn file vs. gold SENS.prn file!\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Passed comparison of SENS.prn files\n";
  print "Exit code = 0\n";
  exit 0;
}



