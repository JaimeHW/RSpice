#!/usr/bin/env perl

open(INPUT,"lead_pde_dio.cir.prn");
open(OUTPUT,">lead_pde_dio.cir.prn.gs");
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a = 0;
  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e   \n",$s[0],$s[1],$s[2],$a;
}
close(INPUT);
close(OUTPUT);
