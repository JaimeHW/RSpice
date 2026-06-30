#!/usr/bin/env perl

$cap=1.0e-6;
$res=1.0e+3;

open(INPUT,"sensCapTrapOrder1.cir.SENS.prn");
open(OUTPUT,">sensCapTrapOrder1.cir.SENS.prn.gs");
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $v1 = exp(-$s[1]/($res*$cap));
  $dv1dR = ($s[1]/($res*$res*$cap))*exp(-$s[1]/($res*$cap));
  $dv1dC = ($s[1]/($res*$cap*$cap))*exp(-$s[1]/($res*$cap));
  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e   %14.8e   \n",$s[0],$s[1],$v1,$dv1dR,$dv1dC;
}
close(INPUT);
close(OUTPUT);
