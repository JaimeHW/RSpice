#!/usr/bin/env perl

open(INPUT,"gnd_and_redund.cir.prn");
open(OUTPUT,">gnd_and_redund.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a = 0.5*$s[1];
  printf OUTPUT "%3d   %14.8e   %14.8e   \n",$s[0],$s[1],$a;

}
close(INPUT);
close(OUTPUT);
