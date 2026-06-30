#!/usr/bin/env perl

open(INPUT,"abmpow2.cir.prn");
open(OUTPUT,">abmpow2.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  $a[0] = $s[0];
  $a[1] = $s[1];
  $tmp = 1.0/$s[1];
  $a[2] = ($tmp*$tmp);
  $a[3] = ($tmp*$tmp*$tmp);

  $end=3;
  printf OUTPUT "%2g  ",$s[0];
  printf OUTPUT "    %14.8e",$a[1];
  printf OUTPUT "    %14.8e",$a[2];
  printf OUTPUT "  %14.8e  \n",$a[3];
}
close(INPUT);
close(OUTPUT);
