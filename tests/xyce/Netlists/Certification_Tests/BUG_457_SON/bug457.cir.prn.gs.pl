#!/usr/bin/env perl

# This script merely outputs the correct results of exponentiating
# 10 to various fractional and negative powers, precisely as the 
# netlist should cause Xyce to do.

@values=(3e15,4e15,5e15,6e15);
open (OUTPUT,">bug457.cir.prn.gs");
open (RESFILE,">bug457.cir.res.gs");
$i=0;
print OUTPUT "Index V(1) V(test1) V(test2) V(test3) V(test4)\n";
print RESFILE "STEP DOPING\n";
foreach(@values)
{
    printf OUTPUT "%3d 1.00000000e+00 %14.8e %14.8e %14.8e %14.8e\n",
                   0,
                   10.0**log10($_),
                   10.0**(0.2*log10($_)),
                   10.0**(-1.0*log10($_)),
                   10.0**(-0.2*log10($_));
    printf OUTPUT "%3d 2.00000000e+00 %14.8e %14.8e %14.8e %14.8e\n",
                   1,
                   10.0**log10($_),
                   10.0**(0.2*log10($_)),
                   10.0**(-1.0*log10($_)),
                   10.0**(-0.2*log10($_));
    printf RESFILE "%3d %14.8e\n",$i,$_;
    $i++;
}
print OUTPUT "End of Xyce(TM) Simulation\n";
print RESFILE "End of Xyce(TM) Parameter Sweep\n";
close(OUTPUT);
close(RESFILE);
exit(0);

sub log10
{
    my $n=shift;
    return log($n)/log(10);
}
