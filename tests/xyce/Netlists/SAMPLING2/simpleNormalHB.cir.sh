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
$CIRFILE="simpleNormalHB.cir";

# Now run that netlist
$CMD="$XYCE -hspice-ext all $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
if (system($CMD) != 0)
{
    `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
    $xyceexit=1;
}
else
{
    if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

$CIRPRN=$CIRFILE;
$CIRPRN =~ s/\.cir/\.cir\.HB\.FD\.prn/g;

# Now read in the Xyce std output and locate the random seed
$seed=`grep -m 1 'Seeding random' $CIRFILE.out`;
chomp($seed);
$seed =~ s/Seeding random number generator with ([0-9]*)/\1/;

print "Seed is $seed\n";

#Save the output
$CIRPRN_save=$CIRPRN."_save";

`mv $CIRPRN $CIRPRN_save`;

#Now re-run Xyce with reported seed:
$CMD="$XYCE -hspice-ext all -randseed $seed $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
if (system($CMD) != 0)
{
    `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
    $xyceexit=1;
}
else
{
    if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

# Now diff the new output with the old --- match should be exact
$CMD="diff $CIRPRN_save $CIRPRN > $CIRPRN.out 2> $CIRPRN.err";
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

