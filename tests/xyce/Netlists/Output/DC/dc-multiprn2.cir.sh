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

$V1="$CIRFILE.V1.prn";
$V2="$CIRFILE.V2.prn";

$GOLDPRNV1="`dirname $ARGV[4]`/$V1";
$GOLDPRNV2="`dirname $ARGV[4]`/$V2";

system("rm -f $V1 $V2 $CIRFILE.prn $CIRFILE.err");

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
        printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE; exit 10;
    }
}

if ( !(-f "$V1")) {
    print STDERR "Output file missing, $V1\n";
    $xyceexit=14;
}

if ( !(-f "$V2")) {
    print STDERR "Output file missing, $V2\n";
    $xyceexit=14;
}

if ( -f "$CIRFILE.prn") {
    print STDERR "Extra output file $CIRFILE.prn, it should not exist\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl

if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE junk $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
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

$retcode = 0;

system("grep -v file=dc-multiprn.cir.V2.prn $CIRFILE > $CIRFILE.V1.cir");
system("grep -v file=dc-multiprn.cir.V1.prn $CIRFILE > $CIRFILE.V2.cir");
$CMD="$XYCE_VERIFY -debug -verbose $CIRFILE.V1.cir $GOLDPRNV1 $V1 > $V1.out 2> $V1.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $V1, see $V1.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY -debug -verbose $CIRFILE.V2.cir $GOLDPRNV2 $V2 > $V2.out 2> $V2.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $V2, see $V2.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
