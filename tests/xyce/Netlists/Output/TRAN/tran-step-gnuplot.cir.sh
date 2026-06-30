#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

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

$GOLDSPLOT =$GOLDPRN;
$GOLDSPLOT =~ s/\.prn$/\.splot\.prn/; # change ending .splot.prn at the end.

`rm -f $CIRFILE.prn $CIRFILE.splot.* $CIRFILE.out $CIRFILE.err`;
`rm -f $CIRFILE.prn.out $CIRFILE.prn.err`;

# run Xyce
$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

if ( !(-f "$CIRFILE.splot.prn"))
{
  print "Missing output file $CIRFILE.splot.prn\n";
  print "Exit code = 14\n";
  exit 14;
}

# Now check the .prn and .splot.prn files.  Use file_compare.pl because of the blank lines.
$retcode=0;
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$CMD="$fc $CIRFILE.prn $GOLDPRN $absTol $relTol $zeroTol > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
$retval=system($CMD);
$retval = $retval >> 8;
if ($retval!= 0){
  print STDERR "Comparator exited on file $CIRFILE.prn with exit code $retval\n";
  $retcode = 2;
}

$CMD="$fc $CIRFILE.splot.prn $GOLDSPLOT $absTol $relTol $zeroTol > $CIRFILE.splot.prn.out 2> $CIRFILE.splot.prn.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited on file $CIRFILE.splot.prn with exit code $retval\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
