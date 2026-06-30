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
$CIRFILE1="colpitts_osc1.cir";
$CIRFILE2="colpitts_osc2.cir";
$CIRFILE3="colpitts_osc3.cir";

$retval = -1;

$retval=$Tools->wrapXyce($XYCE,$CIRFILE1);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE1.prn" ) { print "Exit code = 14\n"; exit 14; }

$retval=$Tools->wrapXyce($XYCE,$CIRFILE2);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE2.prn" ) { print "Exit code = 14\n"; exit 14; }

$retval=$Tools->wrapXyce($XYCE,$CIRFILE3);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE3.prn" ) { print "Exit code = 14\n"; exit 14; }

$retval1 = system("diff $CIRFILE1.prn $CIRFILE2.prn > $CIRFILE2.prn.out 2> $CIRFILE2.prn.err");
$retval2 = system("diff $CIRFILE1.prn $CIRFILE3.prn > $CIRFILE3.prn.out 2> $CIRFILE3.prn.err");

# system returns the exit code of diff in the upper byte of the return code 
# the return code of wait() is in the lower byte.  Thus we need to shift 
# the return values by 8 bits.
$retval1 = $retval1 >> 8;
$retval2 = $retval2 >> 8;

if (($retval1 == 0) && ($retval2 == 0))
{ 
  print "Exit code = 0\n"; exit 0; 
} 
else 
{ 
  print "diff compare failed.  Trying xyce_verify.pl\n";
  # try doing a very tight comparison with xyce_verify.pl using CIRFILE1.prn as the gold standard
  # when comparing to CIRFILE 2 and 3's output  
  $CMD="$XYCE_VERIFY  $CIRFILE1 $CIRFILE1.prn $CIRFILE2.prn > $CIRFILE2.prn.xv_out 2> $CIRFILE2.prn.xv_err";
  $retval1 = system("$CMD") >> 8;
  $CMD="$XYCE_VERIFY  $CIRFILE1 $CIRFILE1.prn $CIRFILE3.prn > $CIRFILE3.prn.xv_out 2> $CIRFILE3.prn.xv_err";
  $retval2 = system("$CMD") >> 8;
  if (($retval1 == 0) && ($retval2 == 0))
  { 
    print "Exit code = 0\n"; exit 0; 
  } 
  else
  {  
    print "Exit code = 2\n"; exit 2; 
  }
}
