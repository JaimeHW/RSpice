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
#$GOLDPRN=$ARGV[4];
$OUTPUTFILE=$CIRFILE . ".out";

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if($retval!=0)
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

# now search the output file for the summary line
#  Failed Steps                          <some number>
# Prior to fixing Xyce/backlog/bugs/-/issues/1
# the failes steps were over 6000.
# now just verify it's under 200.

open my( $outputfile), '<', $OUTPUTFILE or die "Cannot opne file: $!\n";
while (<$outputfile>)
{
  #my $currentLine = $_ if /^ +Failed Steps/;
  if( $_ =~ /^ +Failed Steps/)
  {
    my ($numSteps) = $_ =~ /(\d+)/;
    if( $numSteps > 200)
    {
      print("Xyce had too many failed stesps $numSteps\n");
      $retval=2;
    }
  }
}

print "Exit code = $retval\n"; 
exit $retval;

