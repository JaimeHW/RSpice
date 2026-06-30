#!/usr/bin/env perl

open(INPUT,"exp_ln.cir.prn");
open(OUTPUT,">exp_ln.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  $a[0] = exp($s[2]);
  $a[1] = $s[2];

  printf OUTPUT "%3d  ",$s[0];
  printf OUTPUT "   %14.8e   %14.8e   %14.8e   %14.8e   \n",$s[1],$s[2],$a[0],$a[1];
}
close(INPUT);
close(OUTPUT);

