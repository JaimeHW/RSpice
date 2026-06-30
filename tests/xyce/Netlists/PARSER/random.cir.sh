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

# try a sampling of command line options, not all
# possible ones.
@XyceCMD;
#$XyceCMD[0]="$XYCE -hspice-ext random";
#$XyceCMD[1]="$XYCE -hspice-ext all";
#$XyceCMD[2]="$XYCE -hspice-ext random,units,math";

$XyceCMD[0]="$XYCE ";
$XyceCMD[1]="$XYCE -hspice-ext all";

# remove old files if they exist
system("rm -f $CIRFILE.out $CIRFILE.err");

$retcode = 0;
#foreach $i (0 ... 2)
foreach $i (0 ... 1)
{
  system("rm -f $CIRFILE.prn*");
  $retval = -1;
  $retval=$Tools->wrapXyce($XyceCMD[$i],$CIRFILE);
  if ($retval != 0)
  {
    print "$XyceCMD[$i] failed to run\n";
    print "Exit code = $retval\n"; exit $retval;
  }

  # Exit if the output file was not made
  if (not -s "$CIRFILE.prn" )
  {
    print "$CIRFILE.prn file is missing for $XyceCMD[$i]\n";
    print "Exit code = 14\n";
    exit 14;
  }

  $CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
  if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.prn for $XyceCMD[$i], see $CIRFILE.prn.err\n";
    $retcode = 2;
  }
}

print "Exit code = $retcode\n"; exit $retcode;
