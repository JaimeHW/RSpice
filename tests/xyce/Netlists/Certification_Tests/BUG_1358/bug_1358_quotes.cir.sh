#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);

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

$CIRQ="bug_1358_quotes.cir"; 
$CIRB="bug_1358_braces.cir"; 


# create the output files
$result = $Tools->wrapXyce( "$XYCE", "$CIRQ" );
$result += $Tools->wrapXyce( "$XYCE", "$CIRB" );

if ( $result != 0 ) 
{
  print "Failed to produce prn file\n";
  print "Exit code = 10\n";
  exit 10;
}


# diff the results
$diffs = system( "diff -i $CIRQ.prn $CIRB.prn >> $CIRQ.stdout 2>> $CIRQ.stderr");

if ( $diffs ) 
{ 
  print "Results are NOT identical\n";
  print "Exit code = 2\n"; 
  exit 2; 
}


# output clean result
print "Exit code = 0\n"; 
exit 0; 


