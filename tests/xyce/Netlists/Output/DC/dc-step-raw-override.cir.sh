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
$GOLDRAW=$ARGV[4];

$GOLDRAW =~ s/\.prn$//; # remove the .prn at the end.

system("rm -f $CIRFILE.prn $CIRFILE.err");

$CMD="$XYCE -r $CIRFILE.raw $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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
        printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE; exit 10;
    }
}

if ( -f "$CIRFILE.prn" ) {
    print STDERR "Extra output file $CIRFILE.prn\n";
    $xyceexit=14;
}

if ( !(-f "$CIRFILE.raw" )) {
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

if (system("$XYCE_RAW_READ $GOLDRAW.raw | grep -v 'The file:' | grep -v 'Date:' | grep -v 'No. Points' > $CIRFILE.raw.gold-info 2>$CIRFILE.raw.gold-out") != 0) {
    print STDERR "spice_read failed on file $GOLDRAW.raw, see $CIRFILE.raw.gold-out\n";
    $retcode = 2;
}
if (system("$XYCE_RAW_READ $CIRFILE.raw | grep -v 'The file:' | grep -v 'Date:' | grep -v 'No. Points' > $CIRFILE.raw.info 2>$CIRFILE.raw.out") != 0) {
    print STDERR "spice_read failed on file $CIRFILE.raw, see $CIRFILE.raw.out\n";
    $retcode = 2;
}

$CMD="diff -bi $CIRFILE.raw.gold-info $CIRFILE.raw.info > $CIRFILE.raw.out";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.raw, see $CIRFILE.raw.err\n\t$CMD\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
