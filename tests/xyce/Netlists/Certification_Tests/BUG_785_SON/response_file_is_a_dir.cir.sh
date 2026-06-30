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
# output from comparison to go into $CIRFILE.txt.out and the STDERR output from
# comparison to go into $CIRFILE.txt.err.

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDTXT=$ARGV[4];

# clean up past droppings
system("rm -f $CIRFILE.out $CIRFILE.err");

# execute Xyce to try to make a response file (via -rsf), where the 
# requested file is actually a directory
$CMD="$XYCE -prf params.in -rsf . $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval=system($CMD);
if ($retval == 0)
{
  print "Xyce ran, when it should have failed\n";
  print "Exit code = 2";
  exit 2;
}

# this string should be in the output of this failed Xyce run.
# Note: double escaping dot (\\.) matches a single dot rather 
# than any single character.  
  
@searchstrings = ("Netlist error: Could not open response file \\. for output");

$OUTPUTFILE="$CIRFILE.out";
$retval = $Tools->checkError($OUTPUTFILE,@searchstrings);

print "Exit code = $retval\n"; exit $retval; 
