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
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# remove old files if they exist
`rm -f $CIRFILE.PCE.* $CIRFILE.err $CIRFILE.out $CIRFILE.trim $CIRFILE.goldtrim`;

# run Xyce
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
$retval = system("$CMD");
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

# Exit if the PCE.dat file was not made
if (not -s "$CIRFILE.PCE.dat" )
{
  print "$CIRFILE.PCE.dat file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

# Now check the .PCE.dat file
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;

# must remove the data line from the gold file and test file
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$GOLDPCE = substr($GOLDPRN,0,-3)."PCE.dat";

`grep -v "TIME" $CIRFILE.PCE.dat > $CIRFILE.trim`;
`grep -v "TIME" $GOLDPCE > $CIRFILE.goldtrim`;

$retcode=0;
$CMD="$fc $CIRFILE.trim $CIRFILE.goldtrim $absTol $relTol $zeroTol > $CIRFILE.PCE.out 2> $CIRFILE.PCE.err";
$retval = system("$CMD");
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIRFILE.PCE.dat\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
