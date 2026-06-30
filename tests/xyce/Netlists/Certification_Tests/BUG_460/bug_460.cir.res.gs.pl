#!/usr/bin/env perl

open(INPUT,"bug_460.cir.res");
open(OUTPUT,">bug_460.cir.res.gs");
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[SE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $R1 = $s[1];
  $v_ampl = $s[2];

  # v_b is calcualted by:
  #
  # (v_a - v_b)/R1 = v_b/35.0;   (35 is the sum of the other resistors)
  #
  # v_b = v_a * (35/R1)/(1.0 + (35/R1))
  #
  $v_a = 5.0;
  $tmp = 35.0/$R1;
  $v_b = $v_a * $tmp / (1.0 + $tmp);
  $exp1 = abs($v_a) - 10.0;
  $exp2 = (($v_b+2.0)**2.0)*1.0e+3;
  
  printf OUTPUT "%d     %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e \n",
     $s[0],$R1,$v_ampl, $v_a,$v_b,$exp1,$exp2;
}
close(INPUT);
close(OUTPUT);
