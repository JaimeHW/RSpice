#!/usr/bin/env perl

open(INPUT,"just_redund.cir.prn");
open(OUTPUT,">just_redund.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  printf OUTPUT "%3d   %14.8e   %14.8e   \n",$s[0],$s[1],$s[1];

}
close(INPUT);
close(OUTPUT);
