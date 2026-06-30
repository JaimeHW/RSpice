#!/usr/bin/env perl

use RawFileCommon;

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
$GOLDRAW=$ARGV[4];

$GOLDRAW =~ s/\.prn$//; # remove the .prn at the end.

$TMPCIRFILE="startup_printline_for_mpde-raw-override-ascii.cir";

# remove previous output files
system("rm -f $CIRFILE.prn $CIRFILE.MPDE.* $CIRFILE.mpde_ic.* $CIRFILE.startup.*");
system("rm -f $CIRFILE.out $CIRFILE.err $CIRFILE.raw");

# run Xyce
$CMD="$XYCE -r $CIRFILE.raw -a $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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

if ( !(-f "$CIRFILE.raw")) {
    print STDERR "Missing output file $CIRFILE.raw\n";
    $xyceexit=14;
}
if ( -f "$CIRFILE.prn") {
    print STDERR "Extra output file $CIRFILE.prn\n";
    $xyceexit=14;
}
if ( -f "$CIRFILE.MPDE.prn") {
    print STDERR "Extra output file $CIRFILE.MPDE.prn\n";
    $xyceexit=14;
}
if ( -f "$CIRFILE.mpde_ic.prn") {
    print STDERR "Extra output file $CIRFILE.mpde_ic.prn\n";
    $xyceexit=14;
}
if ( -f "$CIRFILE.startup.prn") {
    print STDERR "Extra output file $CIRFILE.startup.prn\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# compare output raw file
$absTol=1e-6;
$relTol=1e-3;
$zeroTol=1e-9;

$retcode = RawFileCommon::compareRawFiles($XYCE_VERIFY,$CIRFILE,$GOLDRAW,$absTol,$relTol,$zeroTol);

print "Exit code = $retcode\n"; exit $retcode;
