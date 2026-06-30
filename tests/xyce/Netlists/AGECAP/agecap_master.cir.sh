#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

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
$CIRFILE1="agecap.cir";
$CIRFILE2="agecap_ref.cir";

$retval = -1;

$retval=$Tools->wrapXyce($XYCE,$CIRFILE1);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE1.prn" ) { print "Exit code = 14\n"; exit 14; }

$retval=$Tools->wrapXyce($XYCE,$CIRFILE2);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE2.prn" ) { print "Exit code = 14\n"; exit 14; }

# now check if we're running the open mp version of
# xyce.  If we are then diff isn't reliable and 
# we'll need to use Xyce verify.

$OpenMPFlag=$Tools->checkXyceVersionStringForText( $XYCE, "OpenMP" );

if( $OpenMPFlag == 1 )
{
  $retval = system("$XYCE_VERIFY $CIRFILE1 $CIRFILE1.prn $CIRFILE2.prn $CIRFILE1.plotfile > $CIRFILE1.prn.out 2> $CIRFILE1.prn.err");
}
else
{
  $retval = system("diff $CIRFILE1.prn $CIRFILE2.prn > $CIRFILE1.prn.out 2> $CIRFILE1.prn.err");
}
if ($retval == 0) 
{ 
  print "Exit code = 0\n"; exit 0; 
} 
else 
{ 
  print "Exit code = 2\n"; exit 2; 
}
