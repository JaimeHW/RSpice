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

# Remove files from previous runs
system("rm -f $CIRFILE.err $CIRFILE.out");

# run Xyce, which should exit with a fatal error
$CMD="$XYCE -r $CIRFILE.raw -a $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval=system($CMD);

if ($retval == 0)
{
  print "Xyce ran, when it should have failed\n";
  print "Exit code = 2";
  exit 2;
}

# This string should be in the output of this failed Xyce run.
@searchstrings = ("Conflict between FILE= on .PRINT line and -r command line option");
$retval = $Tools->checkError("$CIRFILE.out",@searchstrings);

print "Exit code = $retval\n"; exit $retval; 
