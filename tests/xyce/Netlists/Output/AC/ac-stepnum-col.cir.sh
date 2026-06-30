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
# output from comparison to go into $CIRFILE.csv.out and the STDERR output from
# comparison to go into $CIRFILE.csv.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.
$fc = $XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;

# comparison tolerances for file_compare.pl
$abstol=1e-5;
$reltol=1e-3;  #0.1%
$zerotol=1e-8;

# remove previous output files
system("rm -f $CIRFILE.FD.* $CIRFILE.out $CIRFILE.err");

# run Xyce
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

# checkout for output files
$xyceexit=0;
if ( !(-f "$CIRFILE.FD.prn") )
{
  print STDERR "Missing output file $CIRFILE.FD.prn\n";
  $xyceexit=14;
}

if ( !(-f "$CIRFILE.FD.noindex.prn") )
{
  print STDERR "Missing output file $CIRFILE.FD.noindex.prn\n";
  $xyceexit=14;
}

if ( !(-f "$CIRFILE.FD.gnuplot.prn") )
{
  print STDERR "Missing output file $CIRFILE.FD.gnuplot.prn\n";
  $xyceexit=14;
}

if ( !(-f "$CIRFILE.FD.splot.prn") )
{
  print STDERR "Missing output file $CIRFILE.FD.splot.prn\n";
  $xyceexit=14;
}

if ($xyceexit != 0)
{
  print "Exit code = $xyceexit\n"; exit $xyceexit;
}

# compare gold and test files
$retcode = 0;
$CMD="$fc $GOLDPRN.FD.prn $CIRFILE.FD.prn $abstol $reltol $zerotol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited on file $CIRFILE.FD.prn with exit code $retval\n";
  $retcode = 2;
}

$CMD="$fc $GOLDPRN.FD.noindex.prn $CIRFILE.FD.noindex.prn $abstol $reltol $zerotol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited on file $CIRFILE.FD.noindex.prn with exit code $retval\n";
  $retcode = 2;
}

$CMD="$fc $GOLDPRN.FD.gnuplot.prn $CIRFILE.FD.gnuplot.prn $abstol $reltol $zerotol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited on file $CIRFILE.FD.gnuplot.prn with exit code $retval\n";
  $retcode = 2;
}

$CMD="$fc $GOLDPRN.FD.splot.prn $CIRFILE.FD.splot.prn $abstol $reltol $zerotol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited on file $CIRFILE.FD.splot.prn with exit code $retval\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
