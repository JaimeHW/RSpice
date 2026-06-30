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

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

# remove old files if they exist
`rm -f $CIRFILE.HOMOTOPY.* $CIRFILE.homotopy.* $CIRFILE.prn $CIRFILE.err`;

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

# Exit if the HOMOTOPY.prn files were not made
$xyce_exit = 0;
if (not -s "$CIRFILE.HOMOTOPY.prn" )
{
  print STDERR "$CIRFILE.HOMOTOPY.prn file is missing\n";
  $xyce_exit = 14;
}

if (not -s "$CIRFILE.HOMOTOPY.noindex.prn" )
{
  print STDERR "$CIRFILE.HOMOTOPY.noindex.prn file is missing\n";
  $xyce_exit = 14;
}

if (not -s "$CIRFILE.HOMOTOPY.gnuplot.prn" )
{
  print STDERR "$CIRFILE.HOMOTOPY.gnuplot.prn file is missing\n";
  $xyce_exit = 14;
}

if (not -s "$CIRFILE.HOMOTOPY.splot.prn" )
{
  print STDERR "$CIRFILE.HOMOTOPY.splot.prn file is missing\n";
  $xyce_exit = 14;
}

if ($xyce_exit != 0) { print "Exit code = 14\n";  exit $xyce_exit;}

# Now check the output files
$retcode=0;

$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$CMD="$fc $CIRFILE.HOMOTOPY.prn $GOLDPRN.HOMOTOPY.prn $absTol $relTol $zeroTol > $CIRFILE.homotopy.out 2> $CIRFILE.homotopy.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator failed on file $CIRFILE.HOMOTOPY.prn with exit code $retval\n";
  $retcode = 2;
}

$CMD="$fc $CIRFILE.HOMOTOPY.noindex.prn $GOLDPRN.HOMOTOPY.noindex.prn $absTol $relTol $zeroTol > $CIRFILE.homotopy.noindex.out 2> $CIRFILE.homotopy.noindex.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator failed on file $CIRFILE.HOMOTOPY.noindex.prn with exit code $retval\n";
  $retcode = 2;
}

$CMD="$fc $CIRFILE.HOMOTOPY.gnuplot.prn $GOLDPRN.HOMOTOPY.gnuplot.prn $absTol $relTol $zeroTol > $CIRFILE.homotopy.gnuplot.out 2> $CIRFILE.homotopy.gnuplot.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator failed on file $CIRFILE.HOMOTOPY.gnuplot.prn with exit code $retval\n";
  $retcode = 2;
}

$CMD="$fc $CIRFILE.HOMOTOPY.splot.prn $GOLDPRN.HOMOTOPY.splot.prn $absTol $relTol $zeroTol > $CIRFILE.homotopy.splot.out 2> $CIRFILE.homotopy.splot.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator failed on file $CIRFILE.HOMOTOPY.splot.prn with exit code $retval\n";
  $retcode = 2;
}

print "Exit code = $retcode\n";
exit $retcode;


