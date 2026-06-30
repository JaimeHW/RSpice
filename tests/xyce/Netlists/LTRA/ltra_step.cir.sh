#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This script tests the behavior of an lossy transmission line when a
# .step loop changes its parameters.

# In the course of fixing bug 1014 (SON) it was discovered that this
# feature was not working correctly, and part of fixing bug 1014
# required fixing this issue.

# We first run a series of non-step simulations and concatenate their
# output, then use this as a gold standard for a .step simulation.

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

# The CIRFILE we're passed is a dummy file with nothing in it.  We
# will create our own netlist, but use that file name for simplicity.
undef ($fh);
open ($fh, ">",$CIRFILE);
&createCircuit($fh,"step",0);
close $fh;

# Run the .step version of the circuit
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval;}

# now generate and run the 3 non-step files
for ($i=0;$i<=2;$i++)
{
    undef ($fh);
    $fname=$CIRFILE."_nonstep".$i;
    open ($fh, ">",$fname);
    &createCircuit($fh,"nonstep",$i);
    close $fh;
    $CMD="$XYCE $fname > $fname.out 2> $fname.err";
    $retval=system($CMD);
    if ($retval != 0)
    {
        print STDERR "Xyce command failed on netlist $fname\n";
        print "Exit code = 13\n"; exit 13;
    }
}

# Concatenate the three output files
$NOSTEPOUTFILE=$CIRFILE."_nonstep.prn";
open (NONSTEPOUT,">$NOSTEPOUTFILE");
for ($i=0;$i<=2;$i++)
{
    $fname=$CIRFILE."_nonstep".$i.".prn";
    open (STEPOUT, "<",$fname);
    while (<STEPOUT>)
    {
        print NONSTEPOUT $_;
    }
    close STEPOUT;
}
close NONSTEPOUT;

# Now compare with xyce_verify using non-step output as gold standard.
$CMD="$XYCE_VERIFY --goodres=".$CIRFILE.".res.gs --testres=".$CIRFILE.".res $CIRFILE $NOSTEPOUTFILE $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
$retval=system($CMD);
if ($retval !=0)
{
    print "Exit code = 2\n";
    exit 2;
}

print "Exit code = 0\n";
exit 0;

sub createCircuit
{
    my ($fh,$mode,$stepnum) = @_;
    print $fh "Transmission Line Circuit\n";
    print $fh "*\n";
    print $fh "* A transmission line with 50 ohm characteristic impedance is fed\n";
    print $fh "* with a sinusoidal input.\n";
    print $fh "*\n";
    print $fh "* The line is terminated with a mismatched load, and so there will be\n";
    print $fh "* reflections present on the input\n";
    print $fh "* The intent is to create a circuit with\n";
    print $fh "* periodic steady state that can be used to compare with a harmonic balance\n";
    print $fh "* simulation when frequency domain loads are implemented.\n";

    if ($mode eq "step")
    {
        print $fh ".global_param td_denom=3\n";
        print $fh ".step td_denom list 3.0 4.0 5.0\n";
    }
    else
    {
        my @stepvals=(3.0, 4.0, 5.0);
        print $fh ".global_param td_denom=$stepvals[$stepnum]\n";
        if ($stepnum == 0)
        {
            print $fh ".options output PRINTFOOTER=false\n";
        }
        elsif ($stepnum == 1)
        {
            print $fh ".options output PRINTFOOTER=false PRINTHEADER=false\n";
        }
        elsif ($stepnum == 2)
        {
            print $fh ".options output PRINTHEADER=false\n";
        }
    }

    print $fh ".param DRIVEFREQ=1MEG\n";
    print $fh ".param PERIOD={1.0/DRIVEFREQ}\n";
    print $fh "* This represents no real transmission line.  We're just trying to come\n";
    print $fh "* up wth one that has 50 ohm characteristic impedance and the right time delay\n";
    print $fh "* over the total 16-unit-length cable\n";
    print $fh ".param Z0=50                ; desired characteristic impedance\n";
    print $fh ".global_param TD={PERIOD/td_denom}        ; desired total time delay\n";
    print $fh ".param LINELENGTH=16\n";
    print $fh ".global_param TDperLENGTH={TD/LINELENGTH}\n";
    print $fh ".global_param LINEINDUCT={TDperLENGTH*Z0} ;td=sqrt(LC),Z0=sqrt(L/C),inductance/length\n";
    print $fh ".global_param LINECAP={TDperLENGTH**2/LINEINDUCT}; cap/length\n";
    print $fh ".param LINERES=1e-3\n";
    print $fh "\n";
    print $fh "VIN 1 0 SIN ( 0.0 5.0 {DRIVEFREQ})\n";
    print $fh "\n";
    print $fh "RIN1 1 2 50\n";
    print $fh "* slightly lossy version of Transmission line has time delay of 1/3 period\n";
    print $fh "oLINE1 2 0 3 0 lline\n";
    print $fh "RL1 3 0 1000\n";
    print $fh "\n";
    print $fh ".model lline ltra r={LINERES} g=0 L={LINEINDUCT} C={LINECAP} LEN=LINELENGTH\n";
    print $fh "\n";
    print $fh "* 10 periods of 1MEGHz\n";
    print $fh ".TRAN 0 10u\n";
    print $fh ".PRINT TRAN V(2) V(3) {TD_DENOM}\n";
    print $fh ".END\n";
}
