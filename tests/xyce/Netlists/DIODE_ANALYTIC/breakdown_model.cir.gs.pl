#!/usr/bin/env perl

open(INPUT,"breakdown_model.cir.prn");
open(OUTPUT,">breakdown_model.cir.prn.gs");

$IBV=0.001;
$C=1;
$N=1;
$Vth=0.025864186;
$Vout0=1.2;
$BV=1;
$B1=$N*$Vth;
$B2=exp(-1*($BV-$Vout0)/$B1)*$IBV/$C/$B1;

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a = $Vout0-$B1*log($B2*$s[1]+1);
  printf OUTPUT "%3d   %14.8e   %14.8e   \n",$s[0],$s[1],$a;
}
close(INPUT);
close(OUTPUT);
