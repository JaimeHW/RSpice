#!/usr/bin/env perl

open(INPUT,"time_param.cir.prn");
open(OUTPUT,">time_param.cir.prn.gs");

#
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  printf OUTPUT "%3d  ",$s[0];
  printf OUTPUT "   %14.8e   %14.8e   %14.8e   \n",$s[1],$s[2],($s[1]*$s[2]);
}
close(INPUT);
close(OUTPUT);

