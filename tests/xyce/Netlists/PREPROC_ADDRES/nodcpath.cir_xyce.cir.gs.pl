#!/usr/bin/env perl

open(INPUT,"$ARGV[0]");
open(OUTPUT,">$ARGV[0].gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a=exp(-1 * $s[1]);
  printf OUTPUT "%3d   %14.8e   %14.8e  %14.8e \n",$s[0],$s[1],$a,$a;
}
close(INPUT);
close(OUTPUT);
