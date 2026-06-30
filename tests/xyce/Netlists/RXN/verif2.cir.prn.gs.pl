#!/usr/bin/env perl

open(INPUT,"verif2.cir.prn");
open(OUTPUT,">verif2.cir.prn.gs");

$A0 = 100.0;
$B0 = 50.0;
$C0 = 0.0;
$K = 1.0e-3;

while ($line = <INPUT>)
{

  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }

  @s = split(" ",$line);
  $time = $s[1];

  $tmp = ($B0/$A0)*exp(-$K*($A0-$B0)*$time);
  $ratio = $tmp/(1.0-$tmp);
  $B = ($A0-$B0)*$ratio;

  $A = ($A0-$B0)+$B;
  $C = $C0+($B0-$B);

  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e   %14.8e\n",$s[0],$s[1],$A,$B,$C;
}
close(INPUT);
close(OUTPUT);

