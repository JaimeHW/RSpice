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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

# these search strings are supposed to occur one right after the other in the
# error output.
@searchstring = ( "Dev Warning: Voltage Node [(]X0:X6:HANGING[)] connected to only 1 device Terminal",
                  "Dev Warning: Voltage Node [(]X0:X6:HANGING[)] does not have a DC path to ground" );

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
$retval = system("$CMD");
if ($retval != 0)
{
  print "Exit code = 10\n";
  exit 10
}

$found = 0;
open(OUT,"$CIRFILE.out");
while ($line = <OUT>)
{
  if ($line =~ "$searchstring[$found]")
  { $found++; }
  if ($found == $#searchstring+1) 
  { close(OUT); }
}

if ($found == $#searchstring+1)
{
  print "Exit code = 0\n";
  exit 0
}
else
{
  print "Exit code = 2\n";
  exit 2 
}

print "Exit code = 1\n";
exit 1


