#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce the appropriate output files, then we return exit code 14.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

# remove old files if they exist
`rm -f $CIRFILE.NOISE.* $CIRFILE.out $CIRFILE.err`;

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

$xyce_exit = 0;
if ( not -s "$CIRFILE.NOISE.prn")
{
    print STDERR "Missing output file $CIRFILE.NOISE.prn\n";
    $xyce_exit = 14;
}

if ( not -s "$CIRFILE.NOISE.noindex.prn")
{
    print STDERR "Missing output file $CIRFILE.NOISE.noindex.prn\n";
    $xyce_exit = 14;
}

if ( not -s "$CIRFILE.NOISE.gnuplot.prn")
{
    print STDERR "Missing output file $CIRFILE.NOISE.gnuplot.prn\n";
    $xyce_exit = 14;
}

if ( not -s "$CIRFILE.NOISE.splot.prn")
{
    print STDERR "Missing output file $CIRFILE.NOISE.splot.prn\n";
    $xyce_exit = 14;
}

if ($xyce_exit != 0) { print "Exit code = 14\n"; exit $xyce_exit;}

# now check the output files.
$xyce_exit = 0;

$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-16;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$CMD="$fc $CIRFILE.NOISE.prn $GOLDPRN.NOISE.prn $absTol $relTol $zeroTol > $CIRFILE.NOISE.prn.out 2> $CIRFILE.NOISE.prn.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0)
{
  print STDERR "Comparator failed on file $CIRFILE.NOISE.prn with exit code $retval\n";
  $xyce_exit = 2;
}

$CMD="$fc $CIRFILE.NOISE.noindex.prn $GOLDPRN.NOISE.noindex.prn $absTol $relTol $zeroTol > $CIRFILE.NOISE.noindex.prn.out 2> $CIRFILE.NOISE.noindex.prn.err";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator failed on file $CIRFILE.NOISE.noindex.prn with exit code $retval\n";
  $xyce_exit = 2;
}

$CMD="$fc $CIRFILE.NOISE.gnuplot.prn $GOLDPRN.NOISE.gnuplot.prn $absTol $relTol $zeroTol > $CIRFILE.NOISE.gnuplot.prn.out 2> $CIRFILE.NOISE.gnuplot.prn.err";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator failed on file $CIRFILE.NOISE.gnuplot.prn with exit code $retval\n";
  $xyce_exit = 2;
}

$CMD="$fc $CIRFILE.NOISE.splot.prn $GOLDPRN.NOISE.splot.prn $absTol $relTol $zeroTol > $CIRFILE.NOISE.splot.prn.out 2> $CIRFILE.NOISE.splot.prn.err";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator failed on file $CIRFILE.NOISE.splot.prn with exit code $retval\n";
  $xyce_exit = 2;
}

print "Exit code = $xyce_exit\n";
exit $xyce_exit;

