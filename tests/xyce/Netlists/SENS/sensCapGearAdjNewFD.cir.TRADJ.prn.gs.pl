#!/usr/bin/env perl
use Math::Trig;

$cap=1.0e-6;
$res=1.0e+3;
$con=1.0/$res;
$pi = 4.0*atan(1.0); 
$maxTime = 0.5e-2;
$period = $maxTime;
$Vmag = 1.0;

open(INPUT,"sensCapGearAdjNewFD.cir.TRADJ.prn");
open(OUTPUT,">sensCapGearAdjNewFD.cir.TRADJ.prn.gs");
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

 
  $time = $s[1];
  $tFrac = $time/$period;

  $denominator = (4*$cap*$cap*$pi*$pi+$con*$con*$period*$period);

  $denom2 = 4*$cap*$cap*$pi*$pi*$res*$res + $period*$period;

  $v = 2*exp(-$con*$time/$cap)*$con*$Vmag*$period*$cap*$pi/$denominator -$con*$Vmag*$period*(2*$cap*cos(2*$pi*$tFrac)*$pi -$con*sin(2*$pi*$tFrac)*$period)/$denominator;

  $dvdR =2*$time*exp(-$time/($res*$cap))*$Vmag*$period*$pi/($res*($denom2)) +2*exp(-$time/($res*$cap))*$Vmag*$period*$cap*$pi/($denom2) -16*exp(-$time/($res*$cap))*$Vmag*$period*$cap*$cap*$cap*$res*$res*$pi*$pi*$pi/($denom2*$denom2) -2*$Vmag*$period*$cap*cos(2*$pi*$tFrac)*$pi/($denom2) +8*$Vmag*$period*(2*$cap*cos(2*$pi*$tFrac)*$res*$pi -sin(2*$pi*$tFrac)*$period)*$cap*$cap*$res*$pi*$pi/($denom2*$denom2);


  $dvdC = 2*$con*$con*$time*exp(-$con*$time/$cap)*$Vmag*$period*$pi/($cap*($denominator)) +2*exp(-$con*$time/$cap)*$con*$Vmag*$period*$pi/($denominator) -16*exp(-$con*$time/$cap)*$con*$Vmag*$period*$cap*$cap*$pi*$pi*$pi/($denominator*$denominator) -2*$con*$Vmag*$period*cos(2*$pi*$tFrac)*$pi/($denominator) +8*$con*$Vmag*$period*(2*$cap*cos(2*$pi*$tFrac)*$pi -$con*sin(2*$pi*$tFrac)*$period)*$cap*$pi*$pi/($denominator*$denominator);

  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e   %14.8e   \n", $s[0], $s[1], $v,$dvdR,$dvdC; } 

close(INPUT);
close(OUTPUT);

