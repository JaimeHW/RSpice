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

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$NUMRUNS = 2;
for($i=0;$i<$NUMRUNS;$i++)
{
  $CMD="$XYCE $CIRFILE > /dev/null 2> $CIRFILE.err";
  if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }

  open(OUT,"$CIRFILE.prn");
  $line = <OUT>;
  $line = <OUT>;
  ($INDX,$TIME,$V1,$V2,$V3,$V4) = split(" ",$line);
  if ( ($V1 < 1) or ($V1 > 2) or 
       ($V2 < 1) or ($V2 > 2) or 
       ($V3 < 1) or ($V3 > 2) or 
       ($V4 < 1) or ($V4 > 2)    ) 
  { $failure = 1; }
  while ($line = <OUT>)
  {
    if ($line =~ "Xyce") { next; }
    ($indx,$time,$v1,$v2,$v3,$v4) = split(" ",$line);
    if ( ($V1 != $v1) or ($V2 != $v2) or ($V3 != $v3) or ($V4 != $v4) )
    { $failure = 1; }
  }
  close(OUT);
  if ($i == 0) { $V1old = $V1;  $V2old = $V2;  $V3old = $V3;  $V4old = $V4; }
  if ($i > 0)
  {
    if ( ($V1 == $V1old) or ($V2 == $V2old) or ($V3 == $V3old) or ($V4 == $V4old) )
    { $failure = 1; }
  }
  if ($i < $NUMRUNS-1) { sleep(1); }
}

if (defined($failure))
{
  print "Exit code = 2\n"; exit 2;
}
else
{ 
  print "Exit code = 0\n"; exit 0;
}
