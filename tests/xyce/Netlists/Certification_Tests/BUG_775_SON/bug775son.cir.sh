#!/usr/bin/env perl

use XyceRegression::Tools;
use MeasureCommon;

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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

#
# this runs Xyce and verifies that the prn file was made
#
MeasureCommon::checkTranFilesExist($XYCE,$CIRFILE);


# Process .mt0 file to get the measure names and measured values 
# for each measure statement. Then deference to get the underlying arrays.
# This makes the subsequent code more readable since the variable are 
# arrays or scalars, and aren't mixed in with array references. 
my $numMeasures=8;
my ($measureNamesRef,$measureValsRef) 
          = MeasureCommon::parseMeasureNamesValues($CIRFILE,$numMeasures);
my @measureNames = @$measureNamesRef;
my @measureVals = @$measureValsRef;

# For this test to succeed, MAXM1 should be about the same as MAXR1 
# and MAXR2 should be about 3 times MAXR1.  The tolerances are big because
# M and R are different variables and we are just testing the scaling 
# of those variables

my $retval = 0;

# this is just for debugging
foreach $j (0 .. 7){
  print "$j @measureNames[$j], @measureVals[$j]\n";
}

# these are the comparisons I'm checking
my $res1 = abs( @measureVals[0] - @measureVals[2] );
my $res1Max = 0.01;

my $res2 = abs( 3*@measureVals[0] - @measureVals[6] );
my $res2Max = 0.05;
print "To pass: $res1 must be less than $res1Max\n";
print "     and $res2 must be less than $res2Max\n";

if( $res1 > $res1Max ) {
  print "Failed comparing MAXM1 to MAXR1: @measureVals[0], @measureVals[2] : $res1 max allowed is $res1Max\n";
  $retval = 10;
}

if( $res2 > $res2Max ) {
  print "Failed comparing 3*MAXM1 to MAXR2:  @measureVals[0], @measureVals[6] : $res2 max allowed is $res2Max\n";
  $retval = 10;
}


print "Exit code = $retval\n";
exit $retval;

