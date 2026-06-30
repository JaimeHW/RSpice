#!/usr/bin/env perl

open(INPUT,"log.cir.prn");
open(OUTPUT,">log.cir.prn.gs");

use POSIX qw(log10);
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  $a[0] = log10($s[2]);
  $a[1] = log10($s[3]);

  printf OUTPUT "%3d  ",$s[0];
  printf OUTPUT "   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   \n",
     $s[1],$s[2],$s[3],$a[0],$a[1];
}
close(INPUT);
close(OUTPUT);

