#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This tests the HB simulation of a transmission line against the time
# domain simulation of the same circuit.

# THIS SCRIPT IS BASICALLY IDENTICAL TO THE ONE FOR THE LOSSLESS TRANSMISSION
# LINE.  The difference is only in the netlist we create in the createNetlist
# subroutine.

# The process will be this:  first, we create and run a transient netlist
# that consists of a transmission line with mismatched load on one end,
# fed by a sinusoidal signal on the other end.  We compare the output of this
# transient run with an archived gold standard.

# Then we run the same circuit but perform an HB simulation instead.  

# Finally, we modify the HB.TD prn file by adding 99us to the time values,
# and compare that to the time-domain simulation's output using xyce_verify.
# Because xyce_verify interpolates the "gold standard" data points to the
# time values in the file under test, this will result in the data in the
# HB.TD.prn file being compared only to the last cycle of the transient
# output, which is what we intend --- HB's solution is the periodic
# steady state of the simulation in frequency domain, and the time-domain
# output is just the fourier transform of the FD solution.  It *should*
# match the long-time transient behavior.

# We do NOT compare the resulting frequency domain output against a
# gold standard.  This is not an oversight, it is intentional.  The
# reason is that only 2 lines of the FD output are actually
# meaningful: the positive and negative fundamental.  All others are
# theoretically zero, but will have all sorts of random small values
# in them.  That could easily be ignored by appropriate settings of a
# zero tolerance for the *magnitudes*, but the *phases* may be (and
# usually are) random NON-zero numbers.  Further, the TD signals from
# HB are just the Fourier transform of the FD data --- and so are just
# a different representation of the same result.  We're already
# checking TD, so we're safe on this problem.

# We'll use wrapXyce for the runs, so we need a Tools object
use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# The CIRFILE we're passed is a dummy file with nothing in it.  We
# will create our own netlist, but use that file name for simplicity.
undef ($fh);
open ($fh, ">",$CIRFILE);
&createCircuit($fh,"trans");
close $fh;

#The CIRFILE we were passed was a dummy, but the goldprn was not.
$retval=$Tools->wrapXyceAndVerify($XYCE,$XYCE_VERIFY,$GOLDPRN,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval;}

# if we get here, our transient has produced a signal that matches the gold
# standard, which is basically just a sanity check to make sure that
# whatever we're working on hasn't broken basic transient operation of the
# transmission line.

# Now we generate an HB netlist instead and run it.
undef ($fh);
$hbCircuit=$CIRFILE;
$hbCircuit =~ s/.cir/_HB.cir/;
$hbTDPrn = $hbCircuit.".HB.TD.prn";

open ($fh, ">",$hbCircuit);
&createCircuit($fh,"HB");
close $fh;
$retval=$Tools->wrapXyce($XYCE,$hbCircuit,$hbTDPrn);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval;}


# If we got here, we were able to run both transient and HB without crashes
# or missing output.  Now check the HB.TD file against the transient run
# we just did.  To do this, create a fake transient output which is the
# HB.TD.prn file with times shifted by 99 microseconds.
$hbTDFakePrn=$hbTDPrn;
$hbTDFakePrn =~ s/prn/prn_fake/;

open(FAKEPRN,">$hbTDFakePrn");
open(REALPRN,"<$hbTDPrn");
while (<REALPRN>)
{
    @fields=split(" ");
    if ($fields[0] eq "Index" || $fields[0] eq "End")
    {
        print FAKEPRN $_;
    } else {
        printf FAKEPRN "%d  %g  %g  %g\n",$fields[0],$fields[1]+99e-6,
            $fields[2],$fields[3];
    }
}
close FAKEPRN;
close REALPRN;

# Now we use xyce_verify to compare this fake output to the transient.
# We are relying on xyce_verify's interpolation scheme here.
$CMD="$XYCE_VERIFY $CIRFILE $CIRFILE.prn $hbTDFakePrn >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
$retval=system($CMD);
if ($retval != 0)
{
    print STDERR "Comparison between transient and HB TD failed\n";
    print "Exit code = 2";
    exit 2;
}

# If we got here, all is well.
print "Exit code = 0\n";
exit 0;

sub createCircuit
{
    my ($fh,$mode) = @_;
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

    print $fh ".param DRIVEFREQ=1MEG\n";
    print $fh ".param PERIOD={1.0/DRIVEFREQ}\n";
    print $fh "* This represents no real transmission line.  We're just trying to come\n";
    print $fh "* up wth one that has 50 ohm characteristic impedance and the right time delay\n";
    print $fh "* over the total 16-unit-length cable\n";
    print $fh ".param Z0=50                ; desired characteristic impedance\n";
    print $fh ".param TD={PERIOD/3.0}        ; desired total time delay\n";
    print $fh ".param LINELENGTH=16\n";
    print $fh ".param TDperLENGTH={TD/LINELENGTH}\n";
    print $fh ".param LINEINDUCT={TDperLENGTH*Z0} ;td=sqrt(LC),Z0=sqrt(L/C),inductance/length\n";
    print $fh ".param LINECAP={TDperLENGTH**2/LINEINDUCT}; cap/length\n";
    print $fh ".param LINERES=1e-3\n";

    print $fh "VIN 1 0 SIN ( 0.0 5.0 {DRIVEFREQ})\n";

    print $fh "RIN1 1 2 50\n";
    print $fh "* slightly lossy version of Transmission line has time delay of 1/3 period\n";
    print $fh "oLINE1 2 0 3 0 lline\n";
    print $fh "RL1 3 0 1000\n";

    print $fh ".model lline ltra r={LINERES} g=0 L={LINEINDUCT} C={LINECAP} LEN=LINELENGTH\n";

    if ($mode eq "trans")
    {
        print $fh "* 100 periods of 1MEGHz\n";
        print $fh ".TRAN 0 100u\n";
        print $fh "*COMP V(2) offset=2.5\n";
        print $fh "*COMP V(3) offset=5.0\n";
        print $fh ".print TRAN V(2) V(3)\n";
    }
    else
    {
        print $fh ".hb 1MEG\n";
        # The LTRA has real problems with TAHB and no startup periods
        # on this circuit, HB converges to a wrong state because the
        # initial condition is not even close to the right guess.
        # And LTRA with startup periods sometimes fails transient
        # on some platforms.  So let's just not use TAHB
        print $fh ".options hbint numfreq=5 TAHB=0\n";
        print $fh ".PRINT HB_TD V(2) V(3)\n";
        print $fh ".PRINT HB_FD VM(2) VP(2) VM(3) VP(3)\n";
    }
    print $fh ".END\n";
}
