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

$CIRFILE_modelpar=$CIRFILE;
$CIRFILE_modelpar =~ s/.cir/_modpar.cir/;

# Clean up droppings from any previous run in this directory.
`rm -f $CIRFILE.prn $CIRFILE.res $CIRFILE.err $CIRFILE.out`;
`rm -f $CIRFILE_modelpar.prn $CIRFILE_modelpar.res $CIRFILE_modelpar.err $CIRFILE_modelpar.out`;

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

$CMD="$XYCE $CIRFILE_modelpar > $CIRFILE_modelpar.out 2>$CIRFILE_modelpar.err";
$retval=system($CMD);
if ($retval !=0)
{
    print STDERR "Failed to run $CIRFILE_modelpar.\n";
    print "Exit code = 10\n";
    exit 10;
}
else
{
  # use file_compare since xyce verify doesn't like NOINDEX format.
    my $dirname = `dirname $XYCE_VERIFY`;
    chomp $dirname;
    my $fc = "$dirname/file_compare.pl";
    `$fc $CIRFILE.prn $CIRFILE_modelpar.prn $absTol $relTol $zeroTol > $CIRFILE.prn.out 2> $CIRFILE.prn.err`;  
    $retval=$? >> 8;
}

if ($retval != 0)
{
    print "Exit code = 2\n";
}
else
{
    print "Exit code = 0\n";
}

exit $retval;

