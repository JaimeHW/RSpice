#!/usr/bin/env perl

open(INPUT,"resistor.cir.prn");
open(OUTPUT,">resistor.cir.prn.gs");

$R = 1000.0;
$G = 1.0/$R;

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a = -$G * $s[1];
  printf OUTPUT "%3d   %14.8e   %14.8e   \n",$s[0],$s[1],$a;
}
close(INPUT);
close(OUTPUT);
