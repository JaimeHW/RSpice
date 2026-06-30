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

system("rm -f $CIRFILE.prn $CIRFILE.err");

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";

if (system($CMD) != 0) {
    `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
    $xyceexit=1;
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

if ( !(-f "$CIRFILE.prn")) {
    print STDERR "Missing output file $CIRFILE.prn\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

$retcode = 0;

$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.prn, see $CIRFILE.prn.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
