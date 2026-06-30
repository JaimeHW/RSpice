#!/usr/bin/env perl

#ForcedDampedHO.cir.sh
# This test script tests the YACC (accelerated mass) device by running a 
# forced, damped harmonic oscillator problem and comparing Xyce results to
# analytic results.
#
# The differential equation for the forced, damped oscillator is:
#
#  d^2x/dt^2 + c/m dx/dt + k/m x = a(t)
#where a(t) is the forcing term
#
# By choosing a(t) = f_0*sin(\tilde{\omega}) and assuming a solution of form
# x(t)=Acos(\tilde{\omega}t)+Bsin(\tilde{\omega}t 
# + exp(-\lambda t)*(C cos(\omega t) + D sin(\omega t)
#
# where \lambda = 1/2 c/m
# and   \omega = sqrt(k/m-lambda^2)
#
# and making note of the orthogonality of the transcendental functions,
# one can obtain expressions for A, B, C and D in terms of initial conditions
# and the coefficients.
#
# This script generates a Xyce netlist, runs it, then computes the analytic
# solution at the times output by Xyce and compares with xyce_verify.pl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CIRFILE="testHO.cir";

# These are the parameters of the harmonic oscillator:
$m= 2;  # kg
$k=25.5; # N/m
$c=.5; # N/(m/s)
$f0=1/$m;
$pi=3.14159265358979;
$omega_twiddle = 1/1.76*2*$pi;
$x0 = 0.4; # m
$v0 = 0.0; # m/s

$lambda = .5*$c/$m;
$omega = sqrt($k/$m - $lambda**2);

# These are the coefficients for the analytic solution
$A = ($c*$omega_twiddle/$m*$f0)/( ($k/$m-$omega_twiddle**2)**2 - ($c*$omega_twiddle/$m)**2);

$B = ($k/$m - $omega_twiddle**2)*$f0/( ($k/$m-$omega_twiddle**2)**2 - ($c*$omega_twiddle/$m)**2);

$C = $x0-$A;

$D = (1/$omega)*($v0 + $lambda*$C-$omega_twiddle*$B);

# for signal offsets:
$accoff=8;
$veloff=2;
$posoff=.6;

# Output a Xyce netlist to simulate that oscillator:


open (CIROUT,">$CIRFILE") || die "Cannot open circuit file for output.";

print CIROUT "*Test of B-source computation of position- and velocity-dependent acceleration\n";
print CIROUT ".param mass=$m\n";
print CIROUT ".param K=$k\n";
print CIROUT ".param c=$c\n";
print CIROUT ".param amag=$f0\n";
print CIROUT ".param omega=$omega_twiddle\n";
print CIROUT "B1 acc 0 V={(-K*v(pos)-c*v(vel))/mass+amag*sin(omega*TIME)}\n";
print CIROUT "r1 acc 0 1\n";
print CIROUT "yacc abc acc vel pos v0=$v0 x0=$x0\n";
# The "method=2" is there to force old DAE to use second order, which is
# necessary
print CIROUT ".options timeint reltol=1e-4 abstol=1e-8 minord=2\n";
print CIROUT ".tran 1ns 36s\n";
print CIROUT ".print tran {v(acc)+$accoff} {v(vel)+$veloff} {v(pos)+$posoff}\n";
print CIROUT "*COMP {v(acc)+$accoff} reltol=0.02 abstol=1e-6\n";
print CIROUT "*COMP {v(vel)+$veloff} reltol=0.02 abstol=1e-6\n";
print CIROUT "*COMP {v(pos)+$posoff} reltol=0.02 abstol=1e-6\n";
print CIROUT ".end\n";
close (CIROUT);


# Now run that netlist
#$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
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

$GOLDPRN="testHO_analytic.cir.prn";
$CIRPRN=$CIRFILE;
$CIRPRN =~ s/\.cir/\.cir\.prn/g;

# Now generate a gold standard from the analytic solution:
open (GOLDOUT,">$GOLDPRN") || die "Cannot open $GOLDPRN for write";
open (CIRPRN,"<$CIRPRN") || die "cannot open $CIRPRN for read";

# now read in the Xyce output and replace the X and V columns with the 
# analytic expressions for acceleration, velocity, and position:
                 
$firstline=1;
while (<CIRPRN>)
{
    ($index,$timeval,$accel,$veloc,$posit)=split(" ");
    if ($firstline==1)
    {
        print GOLDOUT $_;
        $firstline=0;
    }
    elsif ($index =~ /End/)
    {
        print GOLDOUT $_;
    }
    else
    {
        $cos1=cos($omega_twiddle*$timeval);
        $cos2=cos($omega*$timeval);
        $sin1=sin($omega_twiddle*$timeval);
        $sin2=sin($omega*$timeval);
        $exp1=exp(-$lambda*$timeval);
        $posit=$A*$cos1+$B*$sin1+$exp1*($C*$cos2+$D*$sin2);
        $veloc=-$A*$omega_twiddle*$sin1 + $B*$omega_twiddle*$cos1
            -$lambda*$exp1*($C*$cos2+$D*$sin2)
            +$exp1*(-$omega*$C*$sin2+$omega*$D*$cos2);
        $accel=-$A*$omega_twiddle**2*$cos1-$B*$omega_twiddle**2*$sin1
            +$lambda**2*$exp1*($C*$cos2+$D*$sin2)
            -2*$lambda*$exp1*(-$C*$omega*$sin2+$D*$omega*$cos2)
            +$exp1*(-$C*$omega**2*$cos2-$D*$omega**2*$sin2);


# Now offset everything to avoid zero crossings:
        $accel += $accoff;
        $veloc += $veloff;
        $posit += $posoff;
        print GOLDOUT "$index\t$timeval\t$accel\t$veloc\t$posit\n";
    }
}

close (GOLDOUT);
close (CIRPRN);

# Now compare the analytic to the xyce output:
$retval=-1;
$retval=system("$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRPRN > $CIRPRN.out 2> $CIRPRN.err");

if ($retval == 0) { print "Exit code = 0\n"; exit 0; } else {print "Exit code = 2\n"; exit 2;}
