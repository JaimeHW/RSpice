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

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;

# comparison tolerances for ACComparator.pl
$abstol=1e-5;
$reltol=1e-3;
$zerotol=1e-10;
$freqreltol=1e-6;

# remove old .prn files if they exist
system("rm -f $CIRFILE.FD.* $CIRFILE.TD.* $CIRFILE.err $CIRFILE.out");

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

# Exit if the FD.prn and TD.prn files were not made
if (not -s "$CIRFILE.FD.prn" )
{
  print "$CIRFILE.FD.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

if (not -s "$CIRFILE.TD.prn" )
{
  print "$CIRFILE.TD.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

# these strings should be in the output of this successful Xyce run
@searchstrings = ("Netlist warning: AC output cannot be written in Touchstone format, using",
                  "standard format",
                  "Netlist warning: AC output cannot be written in Touchstone format, using",
                  "standard format"
);

$retval = $Tools->checkError("$CIRFILE.out",@searchstrings);
if ($retval != 0)
{
  print "Check on warning message failed\n";
  print "Exit code = $retval\n"; exit $retval;
}

$retcode = 0;

# Now check the .prn files
$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN.TD.prn $CIRFILE.TD.prn > $CIRFILE.TD.prn.out 2> $CIRFILE.TD.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.TD.prn, see $CIRFILE.TD.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_ACVERIFY $GOLDPRN.FD.prn $CIRFILE.FD.prn $abstol $reltol $zerotol $freqreltol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0)
{
  print STDERR "Comparator exited with exit code $retval\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;

