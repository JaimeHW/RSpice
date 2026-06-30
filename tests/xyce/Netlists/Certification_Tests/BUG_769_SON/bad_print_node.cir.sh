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
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
#
# lower byte is not relevent here.  xyce's return code is in
# the upper byte to shift it 8 bits.
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0) 
{ 
  print "Xyce failed running $CIRFILE.\nExit code = 10\n"; 
  exit 10; 
}
else
{
  print "Exit code = 0\n"; exit 0;
}

print "Exit code = 1\n"; exit 1;
