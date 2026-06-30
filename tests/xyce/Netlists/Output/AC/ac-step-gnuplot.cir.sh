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
system("rm -f $CIRFILE.FD.* $CIRFILE.TD.prn $CIRFILE.out $CIRFILE.err");

# run Xyce
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
$retval = system($CMD);
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
if ( not -s "$CIRFILE.FD.prn") 
{
    print STDERR "Missing output file $CIRFILE.FD.prn\n";
    $xyce_exit = 14;
}

if ( !(-f "$CIRFILE.FD.splot.prn"))
{
    print STDERR "Missing output file $CIRFILE.FD.splot.prn\n";
    $xyce_exit = 14;
}

if ($xyce_exit != 0) { print "Exit code = 14\n"; exit $xyce_exit;}

# now check the .FD.prn and .FD.splot.prn files.  Use file_compare.pl
# because of the blank lines.
$retcode=0;

$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$CMD="$fc $CIRFILE.FD.prn $GOLDPRN.FD.prn $absTol $relTol $zeroTol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited on file $CIRFILE.FD.prn with exit code $retval\n";
  $retcode = 2;
}

$CMD="$fc $CIRFILE.FD.splot.prn $GOLDPRN.FD.splot.prn $absTol $relTol $zeroTol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited on file $CIRFILE.FD.splot.prn with exit code $retval\n";
  $retcode = 2;
}

# the test should not make a .TD.prn file
if ( -f "$CIRFILE.TD.prn") 
{
    print "Output file $CIRFILE.TD.prn produced, when it should not\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;

