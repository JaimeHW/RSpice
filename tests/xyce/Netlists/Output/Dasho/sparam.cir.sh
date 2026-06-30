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
# output from comparison to go into $CIRFILE.raw.out and the STDERR output from
# comparison to go into $CIRFILE.raw.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$DASHOFILE="sparamOutput";

# Remove the previous output files, including some that are only made if the
# previous run failed.
system("rm -f $CIRFILE.s1p $CIRFILE.FD.prn sparamFoo $CIRFILE.err $CIRFILE.out");
system("rm -f $DASHOFILE*");

# run Xyce
$CMD="$XYCE -o $DASHOFILE.s1p $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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

# check for output files
$xyceexit=0;
if ( (-f "$CIRFILE.s1p") || (-f "$CIRFILE.FD.prn") )  {
  print STDERR "Extra output files $CIRFILE.s1p or $CIRFILE.FD.prn\n";
  $xyceexit=2;
}

if ( -f "sparamFoo") {
  print STDERR "Extra output file sparamFoo\n";
  $xyceexit=2;
}

if ( !(-f "$DASHOFILE.s1p") ){
  print STDERR "Missing -o output file for .LIN, $DASHOFILE.s1p\n";
  $xyceexit=14;
}

if ( !(-f "$DASHOFILE.FD.prn") ){
  print STDERR "Missing -o output file for .PRINT AC, $DASHOFILE.FD.prn\n";
  $xyceexit=14;
}

if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# Now verify the output files
$retcode=0;

$abstol=1e-5;
$reltol=1e-3;
$zerotol=1e-10;
$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;
$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end;

$CMD="$fc $DASHOFILE.s1p $GOLDPRN.s1p $abstol $reltol $zerotol > $DASHOFILE.out 2> $DASHOFILE.s1p.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.s1p, see $DASHOFILE.s1p.err\n";
    $retcode = 2;
}

$CMD="$fc $DASHOFILE.FD.prn $GOLDPRN.FD.prn $abstol $reltol $zerotol > $DASHOFILE.FD.prn.out 2> $DASHOFILE.FD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.FD.prn, see $DASHOFILE.FD.prn.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
