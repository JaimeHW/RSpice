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

open(F, "<$CIRFILE.prn");
$line = <F>;
$line = <F>;
close(F);

# Check exponent style
# Rather than trying to make a one-size-fits-all regexp that works both on
# Windows (3-digit exponents) and good systems (2-digit exponents),
# let's just figure out which one we have, and use a tailored regexp.
# 
# This is necessary because if the number of digits in the exponent is
# 3, then there are only 5 spaces following the number, not 6.  Just using
# [0-9].[0-9]{2,3}[ ]{5,6}[ -] could catch this case, but it could also 
# catch a case where 2 digits of exponent were printed AND an insufficient
# number of subsequent spaces were printed.  So let's not try to be fancy,
# and instead go for strictly correct.
if ($line =~ m/^[0-9]+[ ]*[0-9].[0-9]{8}[Ee].[0-9]{3}/ )
{
   $regexp = "[0-9]+[ ]*[0-9].[0-9]{8}[Ee].[0-9]{3}[ ]{5}[ -][0-9].[0-9]{8}[Ee].[0-9]{3}[ ]{5}[ -][0-9].[0-9]{8}[Ee].[0-9]{3}";
}
else
{
   $regexp = "[0-9]+[ ]*[0-9].[0-9]{8}[Ee].[0-9]{2}[ ]{6}[ -][0-9].[0-9]{8}[Ee].[0-9]{2}[ ]{6}[ -][0-9].[0-9]{8}[Ee].[0-9]{2}";
}

if ($line !~ $regexp) {
    print STDERR "Format failed on file $CIRFILE.prn, see $CIRFILE.prn.err\n";
    print STDERR "$CMD\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
