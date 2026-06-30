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
$abstol=1e-6;
$reltol=1e-3;
$zerotol=1e-12;

$CIR1="fullWaveRectDiodeOldFD.cir";
$CIR2="fullWaveRectDiodeNewFD.cir";

`rm -f $CIR1.SENS.prn`;
`rm -f $CIR2.SENS.prn`;

$retval=$Tools->wrapXyce($XYCE,$CIR1);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

if ( not -s "$CIR1.SENS.prn" ) 
{ 
  print STDERR "$CIR1.SENS.prn not found\n";
  print "Exit code = 14\n"; 
  exit 14; 
}

$retval=$Tools->wrapXyce($XYCE,$CIR2);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

if ( not -s "$CIR2.SENS.prn" ) 
{ 
  print STDERR "$CIR2.SENS.prn not found\n";
  print "Exit code = 14\n"; 
  exit 14; 
}

`rm -f $CIR1.SENS.prn.out $CIR1.SENS.prn.err`; 
$retval = system("diff $CIR1.SENS.prn $CIR2.SENS.prn > $CIR1.SENS.prn.out 2> $CIR1.SENS.prn.err");
if ($retval == 0) 
{
  print "Exit code = 0\n";
  exit 0;
}
else
{
   $CMD = "$XYCE_VERIFY $CIR1.SENS.prn $CIR2.SENS.prn $abstol $reltol $zerotol >> $CIR1.SENS.prn.out 2>> $CIR1.SENS.prn.err";
   print STDERR "diff failed on $CIR1.SENS.prn $CIR2.SENS.prn\nRunning file_compare instead\n";
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
}

