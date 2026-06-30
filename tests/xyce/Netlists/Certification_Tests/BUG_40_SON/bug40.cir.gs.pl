#!/usr/bin/env perl

open(INPUT,"bug40.cir.prn");
open(OUTPUT,">bug40.cir.prn.gs");


while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a = 1-0.5*exp(-2/3*$s[1])-0.5*exp(-2*$s[1]);
  $b = 0.5*exp(-2/3*$s[1])-0.5*exp(-2*$s[1]);
  printf OUTPUT "%3d   %14.8e   %14.8e  %14.8e %14.8e %14.8e \n",$s[0],$s[1],$a,$b,0,0;
}
close(INPUT);
close(OUTPUT);
