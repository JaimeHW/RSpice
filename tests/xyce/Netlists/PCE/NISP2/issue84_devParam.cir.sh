#!/usr/bin/env perl

# This script generates a Xyce netlist that creates a large number of 
# normally distributed random numbers, then runs Xyce on it.  It scans
# the Xyce output for the reported random seed.

# It then re-runs Xyce with the "-randseed" option, and checks that the
# random output so generated is identical to the one produced previously.

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
$CIRFILE1="issue84_devParam1.cir";
$CIRFILE2="issue84_devParam2.cir";

# remove previous output files
system("rm -f $CIRFILE1.ES.* $CIRFILE1.out $CIRFILE1.err $CIRFILE1.prn");
system("rm -f $CIRFILE2.ES.* $CIRFILE2.out $CIRFILE2.err $CIRFILE2.prn");

# Run the netlists
$CMD="$XYCE $CIRFILE1 > $CIRFILE1.out 2>$CIRFILE1.err";
if (system($CMD) != 0)
{
    `echo "Xyce EXITED WITH ERROR! on $CIRFILE1" >> $CIRFILE1.err`;
    $xyceexit=1;
}
else
{
    if (-z "$CIRFILE1.err" ) {`rm -f $CIRFILE1.err`;}
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

if ( !(-f "$CIRFILE1.ES.prn"))
{
    print STDERR "Missing output file $CIRFILE1.ES.prn\n";
    print "Exit code = 14\n"; exit 14;
}

$CMD="$XYCE $CIRFILE2 > $CIRFILE2.out 2>$CIRFILE2.err";
if (system($CMD) != 0)
{
    `echo "Xyce EXITED WITH ERROR! on $CIRFILE2" >> $CIRFILE2.err`;
    $xyceexit=1;
}
else
{
    if (-z "$CIRFILE2.err" ) {`rm -f $CIRFILE2.err`;}
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

if ( !(-f "$CIRFILE2.ES.prn"))
{
    print STDERR "Missing output file $CIRFILE2.ES.prn\n";
    print "Exit code = 14\n"; exit 14;
}

# diff the the ES.prn files.  They should exactly match.
$CMD="diff $CIRFILE1.ES.prn $CIRFILE2.ES.prn > $CIRFILE1.ES.prn.out 2> $CIRFILE1.ES.prn.err";
$passed=1;
if (system("$CMD") != 0)
{
    $passed=0;
}
if ($passed==1)
{
    print "Exit code = 0\n";
    exit 0;
}
else
{
    print "Exit code = 2\n";
    exit 2;
}

