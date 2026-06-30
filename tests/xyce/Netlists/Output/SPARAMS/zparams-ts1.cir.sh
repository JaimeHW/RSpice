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


@CIR;
$CIR[0]="zparams-ts1-1port.cir";
$CIR[1]="zparams-ts1-2port.cir";

@GOLD;
$GOLD[0]="zparams-ts1-1port.cir.s1p";
$GOLD[1]="zparams-ts1-1port.cir.ma.s1p";
$GOLD[2]="zparams-ts1-1port.cir.db.s1p";
$GOLD[3]="zparams-ts1-2port.cir.s2p";
$GOLD[4]="zparams-ts1-2port.cir.ma.s2p";
$GOLD[5]="zparams-ts1-2port.cir.db.s2p";

# remove old output files if they exist
system("rm -f $CIR[0].s1p* $CIR[0].ma.s1p* $CIR[0].db.s1p* $CIR[0].FD.prn");
system("rm -f $CIR[1].s2p* $CIR[1].ma.s2p* $CIR[1].db.s2p* $CIR[1].FD.prn");

# run Xyce
foreach $i (0 .. 1)
{
  # remove old files if they exist
  system("rm -f $CIR[$i].out $CIR[$i].err");

 $CMD="$XYCE $CIR[$i] > $CIR[$i].out 2> $CIR[$i].err";
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
}

# Exit if the various output files were not made
if (not -s "$CIR[0].s1p" )
{
  print "$CIR[0].s1p file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

if (not -s "$CIR[0].ma.s1p" )
{
  print "$CIR[0].ma.s1p file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

if (not -s "$CIR[0].db.s1p" )
{
  print "$CIR[0].db.s1p file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

if (not -s "$CIR[1].s2p" )
{
  print "$CIR[1].s2p file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

if (not -s "$CIR[1].ma.s2p" )
{
  print "$CIR[1].ma.s2p file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

if (not -s "$CIR[1].db.s2p" )
{
  print "$CIR[1].db.s2p file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

# verify that no .AC output is made, for at least one netlist
if ( -s "$CIR[0].FD.prn" )
{
  print "$CIR[0].FD.prn made when it should not be\n";
  print "Exit code = 2\n";
  exit 2;
}

# Now check the various output files
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$dirname = `dirname $GOLDPRN`;
chomp($dirname);

$retcode = 0;

$CMD="$fc $CIR[0].s1p $dirname/$GOLD[0] $absTol $relTol $zeroTol > $CIR[0].s1p.out 2> $CIR[0].s1p.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIR[0].s1p\n";
  $retcode = 2;
}

$CMD="$fc $CIR[0].ma.s1p $dirname/$GOLD[1] $absTol $relTol $zeroTol > $CIR[0].ma.s1p.out 2> $CIR[0].ma.s1p.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIR[0].ma.s1p\n";
  $retcode = 2;
}

$CMD="$fc $CIR[0].db.s1p $dirname/$GOLD[2] $absTol $relTol $zeroTol > $CIR[0].db.s1p.out 2> $CIR[0].db.s1p.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIR[0].db.s1p\n";
  $retcode = 2;
}

$CMD="$fc $CIR[1].s2p $dirname/$GOLD[3] $absTol $relTol $zeroTol > $CIR[1].s2p.out 2> $CIR[1].s2p.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIR[1].s2p\n";
  $retcode = 2;
}

$CMD="$fc $CIR[1].ma.s2p $dirname/$GOLD[4] $absTol $relTol $zeroTol > $CIR[1].ma.s2p.out 2> $CIR[1].ma.s2p.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIR[1].ma.s1p\n";
  $retcode = 2;
}

$CMD="$fc $CIR[1].db.s2p $dirname/$GOLD[5] $absTol $relTol $zeroTol > $CIR[1].db.s2p.out 2> $CIR[1].db.s2p.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIR[1].db.s1p\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;

