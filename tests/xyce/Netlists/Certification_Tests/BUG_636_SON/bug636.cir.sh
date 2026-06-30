#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

@searchstrings = (".TRAN line has an unexpected number of fields",
                  "Unrecognized dot line will be ignored");

$XYCE=$ARGV[0];
$CIRFILE=$ARGV[3];

$cmd = "$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
$retval=system($cmd);

print STDERR "System returns $retval\n";
$exitcode = $retval >> 8;
$signal = $retval & 127;
$coredump = ($signal == 11);

# Correct behavior is to exit with an error message
if ($exitcode == 1 && $coredump == 0)
{

    print STDERR "Checking for strings in $CIRFILE.err\n";
    $rv1 = $Tools->checkError("$CIRFILE.err",@searchstrings);
    print STDERR "Checking for strings in $CIRFILE.out\n";
    $rv2 = $Tools->checkError("$CIRFILE.out",@searchstrings);
    if (($rv1 == 0) or ($rv2 ==0))
    {
        print STDERR "Found error strings and code properly exited without core dump.\n";
        print "Exit code = 0\n";
        exit 0;
    } else {
        print STDERR "Code exited without core dump and with proper exit code, but not with correct error message.\n";
        print "Exit code = 2\n";
        exit 2;
    }
} else {
    printf STDERR "Code exited %s coredump, with exit code %d, and with signal %d\n", ($coredump==1)?'with':'without', $exitcode,$signal;
    print "Exit code = 10\n";
    exit 10;
}

