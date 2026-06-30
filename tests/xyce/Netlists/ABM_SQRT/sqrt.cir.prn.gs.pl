#!/usr/bin/env perl

open(INPUT,"sqrt.cir.prn");
open(OUTPUT,">sqrt.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  $a[0] = sqrt($s[2]);
  $a[1] = $s[2];

  $end=3;
  printf OUTPUT "%3d  ",$s[0];
  for ($varno=1;$varno<$end; $varno++)
  {
    printf OUTPUT "   %14.8e",$s[$varno];
  }
  printf OUTPUT "   %14.8e",$a[0];
  printf OUTPUT "   %14.8e   \n",$a[1];
}
close(INPUT);
close(OUTPUT);
