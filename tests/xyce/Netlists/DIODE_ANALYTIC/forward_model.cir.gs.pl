#!/usr/bin/env perl

open(INPUT,"forward_model.cir.prn");
open(OUTPUT,">forward_model.cir.prn.gs");

$IS=1e-14;
$C=1e-12;
$N=1;
$Vth=0.025864186;
$Vin=0.2;
$B1=$N*$Vth;
$B2=exp($Vin/$B1);
$alpha=$IS/$B1/$C;

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a = $B1*log($B2 - ($B2-1)*exp(-1*$alpha*$s[1]) );
  printf OUTPUT "%3d   %14.8e   %14.8e   \n",$s[0],$s[1],$a;

}
close(INPUT);
close(OUTPUT);
