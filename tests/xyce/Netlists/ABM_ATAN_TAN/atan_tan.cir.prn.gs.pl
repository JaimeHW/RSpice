#!/usr/bin/env perl

open(INPUT,"atan_tan.cir.prn");
open(OUTPUT,">atan_tan.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  $a[0] = tan($s[2]);
  $a[1] = $s[2];

  printf OUTPUT "%3d  ",$s[0];
  printf OUTPUT "   %14.8e   %14.8e   %14.8e   %14.8e   \n",$s[1],$s[2],$a[0],$a[1];
}
close(INPUT);
close(OUTPUT);

sub tan 
{ 
  sin($_[0]) / cos($_[0])  
}

