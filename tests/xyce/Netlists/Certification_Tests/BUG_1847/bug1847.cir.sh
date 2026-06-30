#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# Notes on what this test does:
#
# This script runs bug1847.cir which includes the file "includedNetlist.cir"  
# The included file has a ".print " line that would cause extra data 
# from Xyce to be written to a file.  This test checks that the extra 
# data file has not been created by Xyce.

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

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);

# check that Xyce exited without error
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }


# check that Xyce did NOT make a raw file
# this was the point of the bug report and it 
# should not exist 
if (!(-s "bug1847inc.raw"))
{ 
  print STDERR "Xyce did not create file bug1847inc.raw\n"; 
  print "Exit code = 14\n"; 
  exit 14; 
}

# call xyce verify to ensure that the prn file created is good 
# specifically that it doesn't have any added variables from 
# the .print line in the include file.
$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn";
#
# lower byte is not relevent here.  xyce_verify's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;

if ($retval != 0)
{
  print "Exit code = $retval\n";
  exit $retval;
}
print "Exit code = 0\n";
exit 0;


