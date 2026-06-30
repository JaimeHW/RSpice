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

# No gold standard or comparator needed, only these two required.
$XYCE=$ARGV[0];
$CIRFILE=$ARGV[3];
$CIR2=$CIRFILE;
$CIR2 =~ s/_globalpar/_nopar/;

# Clean up droppings from prior runs:
`rm -f $CIRFILE.prn $CIRFILE.res $CIRFILE.err $CIRFILE.out`;
`rm -f $CIR2.prn $CIR2.res $CIR2.err $CIR2.out`;

# Run Xyce over the main circuit file
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval=system($CMD);
if ($retval !=0)
{
    print STDERR "Failed to run $CIRFILE.\n";
    print "Exit code = 10\n";
    exit 10;
}

# now run the otherp versions
print "Running circuit $CIR2\n";
$CMD="$XYCE $CIR2 > $CIR2.out 2>$CIR2.err";
$retval=system($CMD);
if ($retval !=0)
{
    print STDERR "Failed to run $CIR2.\n";
    print "Exit code = 10\n";
    exit 10;
}

# Now we have a faked gold standard (the "nostep" version) and a real step
# output to compare.  Use xyce_verify:
$CMD="diff $CIR2.prn $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
$retval=system($CMD);
if ($retval !=0)
{
    print "Exit code = ";
    print $retval >> 8;
    print "\n";
    exit $retval >> 8;
}

print "Exit code = 0\n";
exit 0;
