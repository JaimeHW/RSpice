#!/usr/bin/env perl

open(INPUT,"verif1.cir.prn");
open(OUTPUT,">verif1.cir.prn.gs");

$a0 = 1.0e6;
$z0 = 0.0;
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
  $a = $a0 * exp(-$s[1]*$K);
  $z = $z0 + ($a0 - $a);

  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e\n",$s[0],$s[1],$a,$z;
}
close(INPUT);
close(OUTPUT);

