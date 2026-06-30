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

# output should be every 1.0e-6 until 500e-6 then every 10e-6
$step1 =  1.0e-6;
$step2 = 10.0e-6;

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

open(CIRPRN,"$CIRFILE.prn");
$N1 = 0;
$N2 = 1;
#print "N1 = $N1, N2 = $N2\n";
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
  if ($N1 <= 500) 
  {
    $mag = 10**(log($time)/log(10));
    if ( ($time >= $N1*$step1-$zerotol*$mag) and ($time < ($N1+1)*$step1+$zerotol*$mag) )
    { 
      #print "N1 = $N1, time = $time\n";
      $N1++; 
    } 
  }
  else
  {
    $mag = 10**(log($time)/log(10));
    #printf ("zerotol*mag = %.20g\n",$zerotol*$mag);
    if ( ($time >= 5.0e-4+$N2*$step2-$zerotol*$mag) and ($time < 5.0e-4+($N2+1)*$step2+$zerotol*$mag) )
    { 
      #print "          N2 = $N2, time = $time\n";
      $N2++; 
    } 
  }
}
close(CIRPRN);
#print "N1 = $N1, N2 = $N2\n";

if (($N1 == 501) and ($N2 == 51))
{
  #print "Test Passed!\n";
  print "Exit code = 0\n"; exit 0;
}
else
{
  #print "Test Failed!\n";
  print "Exit code = 2\n"; exit 2;
}



