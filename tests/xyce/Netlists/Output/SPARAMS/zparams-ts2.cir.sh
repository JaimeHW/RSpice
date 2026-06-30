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

# remove old files if they exist
system("rm -f $CIRFILE.out $CIRFILE.err $CIRFILE.s2p* $CIRFILE.ma.* $CIRFILE.db.*");
system("rm -f $CIRFILE.FD.*");

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

# Exit if the .s2p files were not made
if (not -s "$CIRFILE.s2p" )
{
  print "$CIRFILE.s2p file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

if (not -s "$CIRFILE.ma.s2p" )
{
  print "$CIRFILE.ma.s2p file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

if (not -s "$CIRFILE.db.s2p" )
{
  print "$CIRFILE.db.s2p file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

# the .PRINT AC output should not happen
if ( -s "$CIRFILE.FD.prn" )
{
  print "$CIRFILE.FD.prn file made when it should not be\n";
  print "Exit code = 2\n";
  exit 2;
}

# Now check the .s2p files
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$freqRelTol=1e-6;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$GOLDS2P = $GOLDPRN;
$GOLDS2P =~ s/\.prn$//;

$retcode = 0;
$CMD="$fc $CIRFILE.s2p $GOLDS2P.s2p $absTol $relTol $zeroTol > $CIRFILE.s2p.out 2> $CIRFILE.s2p.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIRFILE.s2p\n";
  $retcode = 2;
}

$CMD="$fc $CIRFILE.ma.s2p $GOLDS2P.ma.s2p $absTol $relTol $zeroTol > $CIRFILE.ma.s2p.out 2> $CIRFILE.ma.s2p.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIRFILE.ma.s2p\n";
  $retcode = 2;
}

$CMD="$fc $CIRFILE.db.s2p $GOLDS2P.db.s2p $absTol $relTol $zeroTol > $CIRFILE.db.s2p.out 2> $CIRFILE.db.s2p.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIRFILE.db.s2p\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
