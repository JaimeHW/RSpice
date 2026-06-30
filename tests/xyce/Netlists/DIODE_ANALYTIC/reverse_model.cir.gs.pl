#!/usr/bin/env perl

open(INPUT,"reverse_model.cir.prn");
open(OUTPUT,">reverse_model.cir.prn.gs");

$IS=1;
$C=10;
$N=1;
$Vth=0.025864186;
$V0=1;
$a=3*$N*$Vth/exp(1);


while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $temp1=$s[2]-$V0;
  $temp2=$a/3 * log(abs(($s[2]-$a)/($V0-$a)));
  $temp3=-1*$a/6 * log(abs(($s[2]*$s[2]+$a*$s[2]+$a*$a)/($V0*$V0+$V0*$a+$a*$a)));
  $temp4=-1*$a/sqrt(3)*atan2(2/$a/sqrt(3)*$s[2]+1/sqrt(3),1);
  $temp5=   $a/sqrt(3)*atan2(2/$a/sqrt(3)*$V0+1/sqrt(3),1);
  $entry= -1*$C/$IS*($temp1 + $temp2 + $temp3 + $temp4 + $temp5);
  printf OUTPUT "%3d   %14.8e   %14.8e   \n",$s[0],$entry,$s[2];
}
close(INPUT);
close(OUTPUT);
