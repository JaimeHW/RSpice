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
#$GOLDPRN=$ARGV[4];

$CIR0="aAsAtto.cir";
$CIR1="aAsAmp.cir";

# remove old files if they exist
system("rm -f $CIRFILE.out $CIRFILE.err");

# run Xyce with -hspice-ext all to get "a as Atto"
# remove old files if they exist
system("rm -f $CIR0.all*");
$CMD="$XYCE -o $CIR0.all -hspice-ext all $CIR0 > $CIR0.all.out 2> $CIR0.all.err";
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

# run Xyce with -hspice-ext units to get "a as Atto"
# remove old files if they exist
system("rm -f $CIR0.units*");
$CMD="$XYCE -o $CIR0.units -hspice-ext units,separator $CIR0 > $CIR0.units.out 2> $CIR0.units.err";
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

# a as Amp
system("rm -f $CIR1.out $CIR1.err $CIR1.prn");
$CMD="$XYCE $CIR1 > $CIR1.out 2> $CIR1.err";
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

# Exit if the various output files were not made
if (not -s "$CIR0.all.prn" )
{
  print "$CIR0.all.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

if (not -s "$CIR0.units.prn" )
{
  print "$CIR0.units.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

if (not -s "$CIR1.prn" )
{
  print "$CIR1.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

$retcode = 0;
system("tail -n 6 $CIR0.all.prn > $CIR0.all.noheader.prn");
system("tail -n 6 $CIR1.prn > $CIR1.noheader.prn");
$CMD_DIFF="diff $CIR0.all.noheader.prn $CIR1.noheader.prn > $CIRFILE.all.out 2> $CIRFILE.all.err";
if (system("$CMD_DIFF") != 0) { $retcode=2; print STDERR "$CMD_DIFF failed\n";}

$CMD_DIFF="diff $CIR0.units.prn $CIR0.all.prn > $CIRFILE.units.out 2> $CIRFILE.units.err";
if (system("$CMD_DIFF") != 0) { $retcode=2; print STDERR "$CMD_DIFF failed\n";}

print "Exit code = $retcode\n"; exit $retcode;
