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
$CIRBASE=$CIRFILE;
$CIRBASE =~ s/.cir//;
undef(@OTHERCIRS);
for ($i=1;$i<=3;$i++)
{
    push @OTHERCIRS,"$CIRBASE"."_$i.cir";
}

$NOSTEPOUTFILE = "$CIRBASE"."_nostep.cir.prn";

# Clean up droppings from prior runs:
`rm -f $CIRFILE.prn $CIRFILE.res $CIRFILE.err $CIRFILE.out`;
`rm -f $NOSTEPOUTFILE`;

# Run Xyce over the main circuit file, which  is a .step netlist:
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval=system($CMD);
if ($retval !=0)
{
    print STDERR "Failed to run $CIRFILE.\n";
    print "Exit code = 10\n";
    exit 10;
}

# now run all the non-step versions
foreach $CIRCUIT (@OTHERCIRS)
{
    print "Running circuit $CIRCUIT\n";
    $CMD="$XYCE $CIRCUIT > $CIRCUIT.out 2>$CIRCUIT.err";
    $retval=system($CMD);
    if ($retval !=0)
    {
        print STDERR "Failed to run $CIRCUIT.\n";
        print "Exit code = 10\n";
        exit 10;
    }
    # We have carefully constructed the circuit files so they lack just
    # the right headers so that the output can be concatenated and should
    # be precisely the same as the .STEP version 
    `cat $CIRCUIT.prn >> $NOSTEPOUTFILE`;
}

# Now we have a faked gold standard (the "nostep" version) and a real step
# output to compare.  Use xyce_verify:
$CMD="$XYCE_VERIFY --goodres=".$CIRFILE.".res.gs --testres=".$CIRFILE.".res $CIRFILE $NOSTEPOUTFILE $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
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
