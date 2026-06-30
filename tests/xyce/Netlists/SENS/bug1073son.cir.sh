#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.  

$XYCE=$ARGV[0];

$XYCE_VERIFY=$ARGV[1];
# use file_compare since the compared files will have their header lines
# stripped off.  Note: file_compare will only be used if the diff fails
# on $CIR1.SENS.prn2 vs $CIR2.SENS.prn2
$XYCE_VERIFY=~ s/xyce_verify/file_compare/;

#comparison tolerances for file_compare.pl
# these is the original abstol
#$abstol=1e-6;
# these is the new abstol
$abstol=5e-3;

$reltol=1e-3;
$zerotol=1e-12;

$CIR1 = "sensCapGear_baseline.cir";
$CIR2 = "sensCapGear_numerical.cir";

`rm -f $CIR1.SENS.prn`;

$retval=$Tools->wrapXyce($XYCE,$CIR1);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

`rm -f $CIR2.SENS.prn`;
$retval=$Tools->wrapXyce($XYCE,$CIR2);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

if ( not -s "$CIR1.SENS.prn" ) 
{ 
  print STDERR "$CIR1.SENS.prn not found\n";
  print "Exit code = 14\n"; 
  exit 14; 
}
`grep -v "Index" $CIR1.SENS.prn | grep -v "End" > $CIR1.SENS.prn2`;

if ( not -s "$CIR2.SENS.prn" ) 
{
  print STDERR "$CIR2.SENS.prn not found\n";
  print "Exit code = 14\n";
  exit 14;
}

`grep -v "Index" $CIR2.SENS.prn | grep -v "End" > $CIR2.SENS.prn2`;
`rm -f $CIR1.SENS.prn.out $CIR1.SENS.prn.err`; 

$CMD = "$XYCE_VERIFY $CIR1.SENS.prn2 $CIR2.SENS.prn2 $abstol $reltol $zerotol >> $CIR1.SENS.prn.out 2>> $CIR1.SENS.prn.err";
if ( system("$CMD") != 0 )
{
 print STDERR "Verify failed\n";
 print "Exit code = 2\n"; 
 exit 2;
}
else
{
  print "Exit code = 0\n";
  exit 0;
}


