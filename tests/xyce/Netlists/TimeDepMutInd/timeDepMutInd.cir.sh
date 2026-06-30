#!/usr/bin/env perl

# Test for the time-dependent linear mutual inductor.
#
# The test circuit has a constant-current source running trough a resistor and
# inductor in the primary, and an inductor and resistor in the secondary
# loop.  The coupling between the primary and secondary is explicitly time
# dependent.
#
# The original pair of coupled differential equations becomes (since I1 is 
# a constant in this case):
#   dI2/dt+R2/L2*I2 = -I1/L2*dM/dt
# where M is the mutual inductance
#
# Such a system has an analytic solution if M is of the form:
#   M=k(t)*sqrt(L1*L2)
# with k(t)=M0*(sin(2*pi*f*t)+1)/2
#
# I2 = K*a/(denom)*exp(-a*t) - K/(denom)*(a*cos(bt)+b*sin(bt))
# where a=R2/L2, b=2*pi*f, and K=M0*pi*f*I1/L2
# The analytic solution for the voltage drop across the primary inductor is
# v2 = -M*dI2/dt-I2*dM/dt

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CIRFILE="testMutIndLin.cir";
$GOLDPRN="testMutIndLin_analytic.cir.prn";

# Parameters:
$frequency=1e2;
$coupmag=.75;
$pi=3.14159265358979;
$L1=.001;
$L2=.001;
$I1=0.001;
$R2=1000;
$M0=$coupmag*sqrt($L1*$L2);
# The mutual inductance is $M0*(sin(2*pi*f*t)+1)/2
$K=$M0*$pi*$frequency*$I1/$L2;
$a=$R2/$L2;
$b=2*$pi*$frequency;
$denom=$a*$a+$b*$b;
$c1=$K*$a/$denom;
$vp2off=1e-6;
$v2off=1e-6;

# Write out a netlist:
open(CIROUT,">$CIRFILE") || die "Cannot open $CIRFILE for output\n";
print CIROUT "Test of mutual inductor with time-dependent coupling\n";
print CIROUT ".param frequency=$frequency\n";
print CIROUT "IS 1a 0 $I1\n";
print CIROUT "Vprobe1 1a 1 0\n";
print CIROUT "R1 1 2 1K\n";
print CIROUT "vprobe2 3 3a 0\n";
print CIROUT "R2 3a 0 $R2\n";
print CIROUT "L1 2 0 $L1\n";
print CIROUT "L2 3 0 $L2\n";
print CIROUT "K1 L1 L2 {$coupmag*(sin(2*PI*TIME*frequency)+1)/2}\n";
print CIROUT ".TRAN 100US 100MS 10ms\n";

print CIROUT ".OPTIONS TIMEINT method=8 reltol=1e-6 abstol=1e-8\n";
print CIROUT ".OPTIONS NONLIN-TRAN reltol=1e-6 abstol=1e-8\n";

print CIROUT ".PRINT TRAN I(Vprobe1) {I(Vprobe2)+$vp2off} V(1) {V(2)+$v2off} V(3) \n";

print CIROUT "*COMP {V(2)+$v2off} reltol=0.06\n";
print CIROUT ".END\n";
close (CIROUT);

# Now run it:
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";

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
$CIRPRN =~ s/\.cir/\.cir\.prn/g;

# Now generate a gold standard from the analytic solution:
open (GOLDOUT,">$GOLDPRN") || die "Cannot open $GOLDPRN for write";
open (CIRPRN,"<$CIRPRN") || die "cannot open $CIRPRN for read";

$firstline=1;
while (<CIRPRN>)
{
    ($index,$timeval,$i1,$i2,$v1,$v2,$v3)=split(" ");
    if ($firstline==1)
    {
        print GOLDOUT $_;
    }
    elsif ($index =~ /End/)
    {
      print GOLDOUT $_;
    }
    else 
    {
        $i2analytic=$c1*exp(-$a*$timeval)-$K/$denom*($a*cos($b*$timeval)+$b*sin($b*$timeval));
        $di2dt=-$a*$c1*exp(-$a*$timeval)-$K/$denom*(-$a*$b*sin($b*$timeval)+$b*$b*cos($b*$timeval));

        $M=$M0*(sin($b*$timeval)+1)/2;
        $dmdt=$b/2*$M0*cos($b*$timeval);
        $v2analytic=-($M*$di2dt+$i2analytic*$dmdt);
        $v2analytic += $v2off;
        $i2analytic += $vp2off;
        print GOLDOUT "$index\t$timeval\t$i1\t$i2analytic\t$v1\t$v2analytic\t$v3\n";
    } 
    $firstline=0;
}
close (GOLDOUT);
close (CIRPRN);

# Now compare the analytic to the xyce output:
$retval=-1;
$retval=system("$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRPRN > $CIRPRN.out 2> $CIRPRN.err");

if ($retval == 0) { print "Exit code = 0\n"; exit 0; } else {print "Exit code = 2\n"; exit 2;}
