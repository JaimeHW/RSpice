#!/usr/bin/env perl

open(INPUT,"ramp_test2.cir.prn");
open(OUTPUT,">ramp_test2.cir.prn.gs");

$Vth=0.025864186;
$IS=1e-10;
$ISE=1e-12;
$BF=100;
$VAR=1;
$TF=0.02;

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a=((1-$s[1])*$IS+$IS/$BF+$ISE-$TF*$IS)*(exp($s[1]/$Vth)-1);
  $b=(1-$s[1])*$TF*$IS/$Vth*exp($s[1]/$Vth);
  printf OUTPUT "%3d   %14.8e   %14.8e  %14.8e \n",$s[0],$s[1],$s[2],-$a-$b;
}
close(INPUT);
close(OUTPUT);
