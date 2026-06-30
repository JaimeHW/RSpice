#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setFork(1);
#$Tools->setDebug(1);

# The input arguments to this script are set up in 
# Xyce_Test/TestScripts/tsc_run_test_suite and are as follows:
# $1 = location of Xyce binary
# $2 = location of xyce_verify.pl script
# $3 = location of compare script 
# $4 = location of circuit file to test
# $5 = location of gold standard prn file

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
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$CIRFILEDAE="sandler24.cir";
$CIRFILEMPDE="sandler24_WAMPDE.cir";

$retval=$Tools->wrapXyce($XYCE,$CIRFILEDAE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

$retval=$Tools->wrapXyce($XYCE,$CIRFILEMPDE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

`rm -f $CIRFILEDAE.prn.out $CIRFILEDAE.prn.err $CIRFILEMPDE.prn.out $CIRFILEMPDE.prn.err`;

$retcode = 14;
if ( (-f "$CIRFILEDAE.prn") && (-f "$CIRFILEMPDE.prn") )
{
  #$CMD="$XYCE_COMPARE $GOLDPRN $CIRFILE.prn 0.02 1.e-6 0.02 > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
  $CMD="$XYCE_VERIFY $CIRFILEDAE $CIRFILEDAE.prn $CIRFILEMPDE.prn > $CIRFILEMPDE.prn.out 2> $CIRFILEMPDE.prn.err";
  $retval = system("$CMD");
  if ($retval == 0) { $retcode = 0; }
  else { $retcode = 2; }
}

print "Exit code = $retcode\n"; exit $retcode;

