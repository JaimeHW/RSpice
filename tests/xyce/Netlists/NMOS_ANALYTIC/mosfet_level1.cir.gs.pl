#!/usr/bin/env perl

open(INPUT,"mosfet_level1.cir.prn");
open(OUTPUT,">mosfet_level1.cir.prn.gs");

$C=1e-5;
$Vth=1;
$VGS=2;
$K=2e-5;
$VCC=5;
$B1=$K/$C*($VGS-$Vth);
$t0=2*$C/$K*($VCC-($VGS-$Vth))/($VGS-$Vth)**2;
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  if ($s[1] <= $t0)
    {$a=$VCC-$B1/2*($VGS-$Vth)*$s[1];
   }
  else
    {$a=2*($VGS-$Vth)*exp(-1*$B1*($s[1]-$t0))/(1+exp(-1*$B1*($s[1]-$t0)))
   }
  printf OUTPUT "%3d   %14.8e   %14.8e   \n",$s[0],$s[1],$a;
}
close(INPUT);
close(OUTPUT);
