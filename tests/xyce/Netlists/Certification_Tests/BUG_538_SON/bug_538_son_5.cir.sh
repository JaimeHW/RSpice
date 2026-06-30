#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

# No gold standard needed, only these three required.
$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

$CIRFILE_baseline=$CIRFILE;
$CIRFILE_baseline =~ s/.cir/_baseline.cir/;

# clean up droppings from prior runs
`rm -rf $CIRFILE.prn $CIRFILE.err $CIRFILE.out`;
`rm -rf $CIRFILE_baseline.prn $CIRFILE_baseline.err $CIRFILE_baseline.out`;

# Comparison tolerances
my $absTol = 1.0e-6;
my $relTol = 0.01;
my $zeroTol = 1.0e-12;

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval=system($CMD);
if ($retval !=0)
{
    print STDERR "Failed to run $CIRFILE.\n";
    print "Exit code = 10\n";
    exit 10;
}

$CMD="$XYCE $CIRFILE_baseline > $CIRFILE_baseline.out 2>$CIRFILE_baseline.err";
$retval=system($CMD);
if ($retval !=0)
{
    print STDERR "Failed to run $CIRFILE_baseline.\n";
    print "Exit code = 10\n";
    exit 10;
}
else
{
    # use file_compare because we basically want a DIFF here, and
    # don't need all of the other things that xyce_verify does.
    my $dirname = `dirname $XYCE_VERIFY`;
    chomp $dirname;
    my $fc = "$dirname/file_compare.pl";

    # There are TWO output files to compare here.
    `$fc $CIRFILE.HB.FD.prn $CIRFILE_baseline.HB.FD.prn $absTol $relTol $zeroTol > $CIRFILE.prn.out 2> $CIRFILE.prn.err`;
    $retval=$? >> 8;
    `$fc $CIRFILE.HB.TD.prn $CIRFILE_baseline.HB.TD.prn $absTol $relTol $zeroTol >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err`;
    $retval2=$? >> 8;

    if ($retval != 0 || $retval2 != 0)
    {
        print STDERR "One or more compares failed.  Compare of default output file got $retval, compare of second output got $retval2\n";
    }

    $retval=$retval2 if $retval2>$retval;
}

print "Exit code = $retval\n";
exit $retval;
