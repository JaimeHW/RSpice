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
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

open(CIRIN,"$CIRFILE");
while ($line = <CIRIN>)
{
  if ($line =~ m/^\.tran/i)
  {
    @linelist = split(" ",$line);
    $maxdt = $Tools->modVal2Float($linelist[4]);
    $mag = 10**(log($maxdt)/log(10));
  }
}
close(CIRIN);

open(CIRPRN,"$CIRFILE.prn") or $retval=14;
if ($retval == 14) {print "Exit code = 14\n"; exit 14;}
while ($line = <CIRPRN>)
{
  if ($line =~ m/^Index/) { next; }
  if ($line =~ m/^End/) { next; }
  @linelist = split(" ",$line);
  $time = $linelist[1];
  #printf ("time = %.20g\n",$time);
  if ($linelist[0] > 0)
  {
    $dt = $time - $time_old;
    #printf ("dt = %.20g\n",$dt);
    $diff = $dt-$maxdt;
    if ($diff > $zerotol*$mag)
    {
      print "Exit code = 2\n"; exit 2;
    }
  }
  else
  {
    $digits = $linelist[1];
    $digits = length($digits)-6;
    $zerotol = 10**(-$digits+1);
  }
  $time_old = $time;
}
close(CIRPRN);

# At this point the test has passed, so lets remove the Xyce error output.
`rm -f $CIRFILE.err`; 

print "Exit code = 0\n"; exit 0;


