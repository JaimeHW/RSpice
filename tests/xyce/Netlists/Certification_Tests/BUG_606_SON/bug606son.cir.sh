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


# No gold standard needed, only these three required.
$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

# Clean up droppings from prior runs:
`rm -f $CIRFILE.prn $CIRFILE.res $CIRFILE.err $CIRFILE.out`;

# Run Xyce over the main circuit file, 
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval=system($CMD);
if ($retval !=0)
{
    print STDERR "Failed to run $CIRFILE.\n";
    print "Exit code = 10\n";
    exit 10;
}

$CIRPRN=$CIRFILE.".prn";

if ( ! -e $CIRPRN)
{
    print STDERR "Failed to produce $CIRPRN.\n";
    print "Exit code = 14\n";
    exit 14;
}
    
# The test circuit runs a .step loop over temperature from 27 to 30 degrees C,
# and should output 5 columns of data: 2 copies of T in Kelvin, and 3 copies
# of Vt = T*(1.3806226e-23/1.6021918e-19) computed via multiple means.
# Let's test this by reading the file and checking each column in turn.
open(CIRPRN,"<$CIRPRN") ;
$TEMP=27;
$retval=0;
while (<CIRPRN>)
{
    @linefields=split(' ');
    next if ($linefields[0] eq "Index" or $linefields[0] eq "End");
    # We have a line of actual data.  Check the fields
    
    $TEMP_IN_K=$TEMP+273.15;
    $VT = $TEMP_IN_K*(1.3806226e-23/1.6021918e-19);
    if (abs($linefields[1]-$TEMP_IN_K) > 1e-3)
    {
        print STDERR "Column V(3) out of spec for T=$TEMP\n";
        print STDERR "test file has $linefields[1], we have $TEMP_IN_K\n";
        $retval=2;
    } 
    if (abs($linefields[2]-$TEMP_IN_K) > 1e-3)
    {
        print STDERR "Column {TEMP+273.15} out of spec for T=$TEMP\n";
        print STDERR "test file has $linefields[2], we have $TEMP_IN_K\n";
        $retval=2;
    } 
    if (abs($linefields[3]-$VT) > 1e-10)
    {
        print STDERR "Column {(TEMP+273.15)*(1.30806226e-23/1.6021918e-19)} out of spec for T=$TEMP\n";
        print STDERR "test file has $linefields[3], we have $VT\n";
        $retval=2;
    }
    if (abs($linefields[4]-$VT) > 1e-10)
    {
        print STDERR "Column V(4) out of spec for T=$TEMP\n";
        print STDERR "test file has $linefields[4], we have $VT\n";
        $retval=2;
    }
    if (abs($linefields[5]-$VT) > 1e-10)
    {
        print STDERR "Column {Vt} out of spec for T=$TEMP\n";
        print STDERR "test file has $linefields[5], we have $VT\n";
        $retval=2;
    }
    $TEMP = $TEMP+1;
}

print "Exit code = $retval\n";
exit $retval;
