#!/usr/bin/env perl

open(INPUT,"ramp_test1.cir.prn");
open(OUTPUT,">ramp_test1.cir.prn.gs");

$CJE=1e-8;
$VJ=0.75;
$Vth=0.025864186;
$FC=0.5;
$M=0.33;
$IS=1e-10;
$ISE=1e-12;
$BF=100;
$IKF=1;

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a= $IS*(exp($s[1]/$Vth)-1);
  $b = (0.25+$a) ** 0.5 +0.5;
  $c = $a/$b +(1/$BF+$ISE/$IS)*$a;
  if ($s[1] < $FC*$VJ)
  {
      $d = $CJE*(1-$s[1]/$VJ) ** (-1*$M);
  }
  else
  {
      $d = $CJE *(1-$FC) ** (-1 - $M)*(1-$FC*(1+$M)+$M/$VJ*$s[1]);  
  }
  printf OUTPUT "%3d   %14.8e   %14.8e  %14.8e \n",$s[0],$s[1],$s[2],-$c-$d;
}
close(INPUT);
close(OUTPUT);
