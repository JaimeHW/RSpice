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

$retval = -1;

# run the test
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";

if (system($CMD) != 0)
{
  print "Compare failed \n";
  print "Exit code = 2\n";
  exit 2;
}

# check various error cases
$CIRFILE1=$CIRFILE;
$CIRFILE1 =~ s/.cir/RepeatFail.cir/;
# this string should be in the output of this failed Xyce run  
@searchstrings = ( "PWL source repeat value","PWL source repeat value" );
print "Running error test\n";
$retval = $Tools->runAndCheckError($CIRFILE1,$XYCE,@searchstrings);
print "Exit code = $retval\n"; exit $retval; 
