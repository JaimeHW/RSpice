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
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# name of other output and gold files
$OTHERPRN="$CIRFILE.prn2";
$OTHERGOLD= $GOLDPRN.2;

# Comparison tolerances
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;

# cleanup from past runs
`rm -f $CIRFILE.out $CIRFILE.prn $CIRFILE.err`;
`rm -f $CIRFILE.prn.out $CIRFILE.prn.err`;
`rm -f $OTHERPRN.out $OTHERPRN.err`;

# create the output files
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval=system($CMD);
if ($retval != 0) 
{
  if ($retval & 127)
  {
    print "Exit code = 13\n"; 
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIRFILE; 
    exit 13;
  }
  else
  {
    print "Exit code = 10\n"; 
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE; 
    exit 10;
  }
}

# check for both output files
if (not -s "$CIRFILE.prn" ) 
{ 
  print STDERR "Missing output file $CIRFILE.prn\nExit code = 14\n"; 
  print "Exit code = 14\n";
  exit 14;
}
if (not -s "$OTHERPRN" ) 
{ 
  print STDERR "Missing output file $OTHERPRN\nExit code = 14\n"; 
  print "Exit code = 14\n";
  exit 14;
}

# compare both sets of output and gold files
$exitCode=0;
$CMD="$fc $CIRFILE.prn $GOLDPRN $absTol $relTol $zeroTol > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
if (system("$CMD") != 0) 
{
  print STDERR "Verification failed on file $CIRFILE.prn, see $CIRFILE.prn.err\n";
  $exitCode = 2;
}
else
{
  print "passed comparison of $CIRFILE.prn\n";
}

$CMD="$fc $OTHERPRN $OTHERGOLD $absTol $relTol $zeroTol > $OTHERPRN.out 2> $OTHERPRN.err";
if (system("$CMD") != 0) 
{
  print STDERR "Verification failed on file $OTHERPRN, see $OTHERPRN.err\n";
  $exitCode = 2;
}
else
{
    print "Passed comparison of $OTHERPRN\n";
}

print "Exit code = $exitCode\n";
exit $exitCode;
