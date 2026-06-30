#!/usr/bin/env perl

sub dirname($) {my $file = shift; $file =~ s!/?[^/]*/*$!!; return $file; }

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
# output from comparison to go into $CIRFILE.TD.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.TD.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$GOLDDIR = dirname($GOLDPRN);
$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;
$fc = $XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;

# alternate output from .PRINT AC_IC line
$FILEOUT="op-print-line-order-fileout";

# comparison tolerances for ACComparator.pl
$abstol=1e-7;
$reltol=1e-4;  #1%
$zerotol=1e-8;
$freqreltol=1e-6;

# remove files from previous run
system("rm -f $CIRFILE.out $CIRFILE.err $CIRFILE.TD.* $CIRFILE.FD.prn $FILEOUT*");

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

if ( !(-f "$CIRFILE.TD.prn")) {
    print STDERR "Missing output file $CIRFILE.TD.prn\n";
    $xyceexit=14;
}

if ( !(-f "$CIRFILE.FD.prn")) {
    print STDERR "Missing output file $CIRFILE.FD.prn\n";
    $xyceexit=14;
}

if ( !(-f "$FILEOUT")) {
    print STDERR "Missing output file $FILEOUT\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

$retcode = 0;

# checkout the .TD.prn output.  Use file_compare because .PRINT AC and .PRINT AC_IC
# have different variables on them, which confuses xyce_verify.
$CMD="$fc $CIRFILE.TD.prn $GOLDPRN.TD.prn $abstol $reltol $zerotol > $CIRFILE.TD.prn.out 2> $CIRFILE.TD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIRFILE.TD.prn, see $CIRFILE.TD.prn.err\n";
    $retcode = 2;
}

# check the .FD.prn output
$CMD="$XYCE_ACVERIFY $GOLDPRN.FD.prn $CIRFILE.FD.prn $abstol $reltol $zerotol $freqreltol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0) 
{ 
  print STDERR "Comparator exited with exit code $retval\n";
  $retcode = 2; 
}

# check the FILE= output for .PRINT AC_IC
$CMD="$fc $FILEOUT $GOLDDIR/$FILEOUT $abstol $reltol $zerotol > $FILEOUT.out 2> $FILEOUT.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $FILEOUT, see $FILEOUT.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
