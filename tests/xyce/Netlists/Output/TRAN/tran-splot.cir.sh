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
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

`rm -f $CIRFILE.prn $CIRFILE.err`;
`rm -f $CIRFILE.prn.out $CIRFILE.prn.err`;

# run Xyce
$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# now check the .prn file
$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
$retval=system($CMD);
if ($retval!= 0) 
{
  print "test Failed comparison of .prn file vs. gold .prn file!\n";
  print "Exit code= 2\n";
  exit 2;
}
else
{ 
  print "Passed comparison of .prn files\n";
  print "Exit code = 0\n"; 
  exit 0;
}
