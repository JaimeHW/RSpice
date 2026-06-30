#!/usr/bin/env perl
use Math::Trig;

$cap=1.0e-6;
$res=1.0e+3;
$per=1.0e-6;
$frequency=1/$per;
$td=0.0;
$Vm=1.0;
$pi = 4.0*atan(1.0);
$G = 1/$res;

open(INPUT,"vsrcSensTest.cir.SENS.prn");
open(OUTPUT,">vsrcSensTest.cir.SENS.prn.gs");
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

  # analytic solution
  $v2 = 2*exp(-$G*$time/$cap)*$G*$Vm*$per*$cap*$pi/(4*$cap*$cap*$pi*$pi+$G*$G*$per*$per)-$G*$Vm*$per*(2*$cap*cos(2*$pi*$time/$per)*$pi-$G*sin(2*$pi*$time/$per)*$per)/(4*$cap*$cap*$pi*$pi+$G*$G*$per*$per);

  # derivative of V(2) w.r.t. $Vm from Maple
  #
  
  $dv2_dVm = 2*exp(-$G*$time/$cap)*$G*$per*$cap*$pi/(4*$cap**2*$pi**2+$G**2*$per**2)-$G*$per*(2*$cap*cos(2*$pi*$time/$per)*$pi-$G*sin(2*$pi*$time/$per)*$per)/(4*$cap**2*$pi**2+$G**2*$per**2);

  # derivative of V(2) w.r.t. $frequency  from Maple
  $term3 = (4*$cap*$cap*$pi*$pi*$frequency*$frequency+$G*$G);
  $term4 = (4*$cap*$cap*$pi*$pi*$frequency*$frequency+$G*$G);

  $dv2_dfreq = 2*exp(-$G*$time/$cap)*$G*$Vm*$cap*$pi/(4*$cap*$cap*$pi*$pi*$frequency*$frequency+$G*$G)-16*exp(-$G*$time/$cap)*$G*$Vm*$cap*$cap*$cap*$frequency*$frequency*$pi*$pi*$pi/($term3*$term3)-$G*$Vm*(-4*$cap*$pi*$pi*$time*sin(2*$pi*$time*$frequency)*$frequency+2*$cap*cos(2*$pi*$time*$frequency)*$pi-2*$G*$pi*$time*cos(2*$pi*$time*$frequency))/(4*$cap*$cap*$pi*$pi*$frequency*$frequency+$G*$G)+8*$G*$Vm*(2*$cap*cos(2*$pi*$time*$frequency)*$frequency*$pi-$G*sin(2*$pi*$time*$frequency))*$cap*$cap*$frequency*$pi*$pi/($term4*$term4);

  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e   %14.8e   \n",$s[0],$s[1],$v2,$dv2_dVm,$dv2_dfreq;
}
close(INPUT);
close(OUTPUT);
