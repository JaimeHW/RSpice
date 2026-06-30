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

# output should be every 1.0e-3 for the simulation 
$step1 =  1.0e-3;

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# call Xyce verify to ensure the output is still correct
#
$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn";
$retval = system("$CMD");
if ($retval != 0) 
{
    # failed compare
    print "Exit code = 2\n"; 
    exit 2;
}

# count the output lines to ensure that output interpolation was followed 
# and produced exactly the number expected.
open(CIRPRN,"$CIRFILE.prn");
$N1 = 0;
#print "N1 = $N1\n";
while ($line = <CIRPRN>)
{
  if ($line =~ m/^index/i) { next; }
  if ($line =~ m/^end/i) { next; }
  @linelist = split(" ",$line);
  $time = $linelist[1];
  if ($N1 == 0) 
  { 
    $digits = $linelist[1];
    $digits = length($digits)-6;
    $zerotol = 10**(-$digits-1);
    #print "zerotol = $zerotol\n";
    $N1++; 
    next; 
  }
  $mag = 10**(log($time)/log(10));
  if ( ($time >= $N1*$step1-$zerotol*$mag) and ($time < ($N1+1)*$step1+$zerotol*$mag) )
  { 
    #print "N1 = $N1, time = $time\n";
    $N1++; 
  } 
}
close(CIRPRN);
#print "N1 = $N1\n";

if ($N1 == 201)
{
  #print "Test Passed!\n";
  print "Exit code = 0\n"; exit 0;
}
else
{
  #print "Test Failed!\n";
  print "Exit code = 2\n"; exit 2;
}



