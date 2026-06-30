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

$CIRFILE_PRN=$CIRFILE . ".prn";
$OUTPUTFILE=$CIRFILE . ".out";
print "OUTPUT file is $OUTPUTFILE\n";

$GOLD_STD=$CIRFILE;
$GOLD_STD=~s/\.cir/baseline.cir/;
print "Gold Standard file is $OUTPUTFILE\n";
$GOLDPRN=$GOLD_STD . ".prn";

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if($retval!=0)
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

$retval=$Tools->wrapXyce($XYCE,$GOLD_STD);
if($retval!=0)
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

# now verify that each warning is NOT there
if ($retval == 0) { 
  my $CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE_PRN > $CIRFILE_PRN.out 2> $CIRFILE_PRN.err";
  $retval = system("$CMD");
  if ($retval != 0)
  {
    $retval = 2;  # xyce_verify returns tons of codes when it fails, none
                  # are what we are expected to return.
    print "Failed comparison between output and gold standard.";
    print "Exit code = $retval\n";
    exit $retval;
  }
}
  
$retval=0;
print "Exit code = $retval\n"; 
exit $retval;

