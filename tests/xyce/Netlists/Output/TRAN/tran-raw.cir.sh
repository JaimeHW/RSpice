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
$GOLDRAW=$ARGV[4];

$GOLDRAW =~ s/\.prn$//; # remove the .prn at the end.

system("rm -f $CIRFILE.raw $CIRFILE.err");

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";

if (system($CMD) != 0) {
    `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
    $xyceexit=1;
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

$GOLDRAW =~ s/\.raw$//; # remove the .raw at the end.

if ( !(-f "$CIRFILE.raw")) {
    print STDERR "Missing output file $CIRFILE.raw\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

$retcode = 0;

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl

if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE $GOLDRAW $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
    {
        print "Exit code = 2 \n";
        exit 2;
    }
    else
    {
        print "Exit code = 0 \n";
        exit 0;
    }
}

$XYCE_RAW_READ = $XYCE_VERIFY;
$XYCE_RAW_READ =~ s?xyce_verify\.pl?spice_read.py?;

if (system("$XYCE_RAW_READ $GOLDRAW.raw | grep -v 'The file:' | grep -v 'Date:' | grep -v 'Vector-Length' > $CIRFILE.raw.gold-info 2>$CIRFILE.raw.gold-out") != 0) {
    print STDERR "spice_read failed on file $GOLDRAW.raw, see $CIRFILE.raw.gold-out\n";
    $retcode = 2;
}
if (system("$XYCE_RAW_READ $CIRFILE.raw | grep -v 'The file:' | grep -v 'Date:' | grep -v 'Vector-Length' > $CIRFILE.raw.info 2>$CIRFILE.raw.out") != 0) {
    print STDERR "spice_read failed on file $CIRFILE.raw, see $CIRFILE.raw.out\n";
    $retcode = 2;
}

$CMD="diff -bi $CIRFILE.raw.gold-info $CIRFILE.raw.info > $CIRFILE.raw.out";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.raw, see $CIRFILE.raw.out\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
