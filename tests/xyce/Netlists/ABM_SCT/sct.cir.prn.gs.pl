#!/usr/bin/env perl

open(INPUT,"sct.cir.prn");
open(OUTPUT,">sct.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  $a[0] = sin($s[1]);
  $a[1] = cos($s[1]);
  $a[2] = tan($s[1]);
  $a[3] = $a[2];

  printf OUTPUT "%3d  ",$s[0];
  printf OUTPUT "   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   \n",$s[1],$a[0],$a[1],$a[2],$a[3];
}
close(INPUT);
close(OUTPUT);

sub tan 
{ 
  sin($_[0]) / cos($_[0])  
}

