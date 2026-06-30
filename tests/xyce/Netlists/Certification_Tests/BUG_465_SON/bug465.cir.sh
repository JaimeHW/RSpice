#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setFork(1);
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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CIRFILE="bug465.cir";
$OUTFILE="bug465.cir.out";

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
# check to make sure Xyce crashed (as it should for this bug)
$retcode = 0;
if ($retval != 10) 
{ 
  print "Exit code = $retval\n"; 
  $retcode = 1;
  exit $retcode;
}

#
# now use grep to look for the phrase "Transient failure history:";
#
$expr="Too few fields in the .RESULT line";
open(OUT,"$OUTFILE") or die "Error, cannot open output file $OUTFILE!\n";
$count=1;
while ($line = <OUT>) 
{
  if( $line =~ $expr )
  { 
    $count = 0;
  }
}
close(OUT);

$retcode = $count;

print "Exit code = $retcode\n"; 
exit $retcode;

