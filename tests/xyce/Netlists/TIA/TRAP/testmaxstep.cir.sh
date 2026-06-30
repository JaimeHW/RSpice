#!/usr/bin/env perl

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

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.  

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$testfile="testmaxstep.cir";

# --------------------------------
# Run Xyce on the circuits, baseline first, then restarted.

$CMD="$XYCE $testfile > $testfile.out 2> $testfile.err";
#
# lower byte is not relevent here.  xyce's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0) { print "Xyce failed running $testfile.\nExit code = 10\n"; exit 10; }

if ( not -s "$testfile.prn" ) { print "Exit code = 14\n"; exit 14; }

open(CIRPRN, "$testfile.prn");    

$zerotol = 10e-15;
$failed=0;

while ($line = <CIRPRN> )
{

#  $line1 = <CIRPRN>;

  if ($line =~ m/^index/i) { next; }
  if ($line =~ m/^end/i) { next; }

  print "$line \n" ;
  @linelist = split(" ",$line);

  $index = $linelist[0];
  $time = $linelist[1];


   if ($index == 1) 
   {
     if (  ($time >= 1e-10 -$zerotol) and ($time < 1e-10 +$zerotol) )
     {
       $failed = 0;
     }
     else
     {
       $failed = 1;
     }
     last;

   }

}

close(CIRPRN);

if ($failed != 1)
{
  print "Test Passed!\n";
  print "Exit code = 0\n"; exit 0;
}
else
{
  print "Exit code = 2\n"; exit 2;
}

