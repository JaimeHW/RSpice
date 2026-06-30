#!/usr/bin/env perl
use Math::Trig;

$rval=4.7e+03;
$cval=4.7e-08;
$vinval=10.0;
$pi = 4.0*atan(1.0); 

open(INPUT,"lowpass.cir.FD.SENS.prn");
open(OUTPUT,">lowpass.cir.FD.SENS.prn.gs");
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  $freq = $s[1];
  $Xc = 1.0/(2*$pi*$freq*$cval);
  $vmag = $vinval / sqrt(1.0 + (2*$pi*$freq*$rval*$cval)*(2*$pi*$freq*$rval*$cval));
  $vphase = -atan(2*$pi*$freq*$rval*$cval);
  $vreal = $vmag * cos($vphase); 
  $vimag = $vmag * sin($vphase);

  # ------ Mag sensitivities ($freqrom Maple):
  $dvmag_dr = -$vinval * $Xc * $rval / (($rval*$rval + $Xc*$Xc)**(1.5));
  $dvmag_dc = -$vinval / (($pi*$freq*$cval*$cval)*sqrt(4*$rval*$rval+(1/($pi*$pi*$freq*$freq*$cval*$cval)))) + $vinval / ($pi*$pi*$pi*$freq*$freq*$freq*$cval*$cval*$cval*$cval*(4*$rval*$rval+1/($pi*$pi*$freq*$freq*$cval*$cval))**1.5);
  $dvmag_dvin = $Xc/sqrt($rval*$rval + $Xc*$Xc);

  # ------ Phase sensitivities ($freqrom Maple):
  $dvphase_dr = -2*$pi*$freq*$cval/(4*$cval*$cval*$pi*$pi*$rval*$rval*$freq*$freq+1);
  $dvphase_dc = -2*$pi*$freq*$rval/(4*$cval*$cval*$pi*$pi*$rval*$rval*$freq*$freq+1);
  $dvphase_dvin = 0.0;

  # ------ Real sensitivities ($freqrom Maple):
  $dvreal_dr = -4.0*$vinval*$pi*$freq*$cval*$rval/((4.0*$cval**2*$pi**2*$rval**2*$freq**2+1.0)**(3/2)*sqrt(4.0*$rval**2+1.0/($pi**2*$freq**2*$cval**2)))-4.0*$vinval*$rval/(sqrt(4.0*$cval**2*$pi**2*$rval**2*$freq**2+1.0)*$pi*$freq*$cval*(4.0*$rval**2+1.0/($pi**2*$freq**2*$cval**2))**(3/2));

  $dvreal_dc = -4.0*$vinval*$pi*$freq*$rval**2/((4.0*$cval**2*$pi**2*$rval**2*$freq**2+1.0)**(3/2)*sqrt(4.0*$rval**2+1.0/($pi**2*$freq**2*$cval**2)))-$vinval/(sqrt(4.0*$cval**2*$pi**2*$rval**2*$freq**2+1.0)*$pi*$freq*$cval**2*sqrt(4.0*$rval**2+1.0/($pi**2*$freq**2*$cval**2)))+$vinval/(sqrt(4.0*$cval**2*$pi**2*$rval**2*$freq**2+1.0)*$pi**3*$freq**3*$cval**4.0*(4.0*$rval**2+1.0/($pi**2*$freq**2*$cval**2))**(3/2));

  $dvreal_dvin = 1.0/(sqrt(4.0*$cval*$cval*$pi*$pi*$rval*$rval*$freq*$freq+1)*$pi*$freq*$cval*sqrt(4.0*$rval*$rval+1.0/($pi*$pi*$freq*$freq*$cval*$cval)));

  # ------ Imag sensitivities ($freqrom Maple):
  $dvimag_dr = -2*$vinval/(sqrt(4*$cval**2*$pi**2*$rval**2*$freq**2+1)*sqrt(4*$rval**2+1/($pi**2*$freq**2*$cval**2)))+8*$vinval*$rval**2*$cval**2*$pi**2*$freq**2/((4*$cval**2*$pi**2*$rval**2*$freq**2+1)**(3/2)*sqrt(4*$rval**2+1/($pi**2*$freq**2*$cval**2)))+8*$vinval*$rval**2/(sqrt(4*$cval**2*$pi**2*$rval**2*$freq**2+1)*(4*$rval**2+1/($pi**2*$freq**2*$cval**2))**(3/2));

  $dvimag_dc = 8*$vinval*$rval**3*$cval*$pi**2*$freq**2/((4*$cval**2*$pi**2*$rval**2*$freq**2+1)**(3/2)*sqrt(4*$rval**2+1/($pi**2*$freq**2*$cval**2)))-2*$vinval*$rval/(sqrt(4*$cval**2*$pi**2*$rval**2*$freq**2+1)*(4*$rval**2+1/($pi**2*$freq**2*$cval**2))**(3/2)*$pi**2*$freq**2*$cval**3);

  $dvimag_dvin = -2*$rval/(sqrt(4*$cval**2*$pi**2*$rval**2*$freq**2+1)*sqrt(4*$rval**2+1/($pi**2*$freq**2*$cval**2)));
  #
  printf OUTPUT "%3d   %14.8e   ",$s[0],$freq;
  #
  printf OUTPUT "%14.8e   ",$vreal;
  printf OUTPUT "%14.8e   ",$vimag;
  printf OUTPUT "%14.8e   ",$vmag;
  printf OUTPUT "%14.8e   ",$vphase;
  #
  # For comparison to direct sensitivities
  printf OUTPUT "%14.8e   ",$dvreal_dr;
  printf OUTPUT "%14.8e   ",$dvimag_dr;
  printf OUTPUT "%14.8e   ",$dvmag_dr;
  printf OUTPUT "%14.8e   ",$dvphase_dr;
  #
  printf OUTPUT "%14.8e   ",$dvreal_dc;
  printf OUTPUT "%14.8e   ",$dvimag_dc;
  printf OUTPUT "%14.8e   ",$dvmag_dc;
  printf OUTPUT "%14.8e   ",$dvphase_dc;
  #
  printf OUTPUT "%14.8e   ",$dvreal_dvin;
  printf OUTPUT "%14.8e   ",$dvimag_dvin;
  printf OUTPUT "%14.8e   ",$dvmag_dvin;
  printf OUTPUT "%14.8e   ",$dvphase_dvin;
  #
  # For comparison to adjoint sensitivities
  printf OUTPUT "%14.8e   ",$dvreal_dr;
  printf OUTPUT "%14.8e   ",$dvimag_dr;
  printf OUTPUT "%14.8e   ",$dvmag_dr;
  printf OUTPUT "%14.8e   ",$dvphase_dr;
  #
  printf OUTPUT "%14.8e   ",$dvreal_dc;
  printf OUTPUT "%14.8e   ",$dvimag_dc;
  printf OUTPUT "%14.8e   ",$dvmag_dc;
  printf OUTPUT "%14.8e   ",$dvphase_dc;
  #
  printf OUTPUT "%14.8e   ",$dvreal_dvin;
  printf OUTPUT "%14.8e   ",$dvimag_dvin;
  printf OUTPUT "%14.8e   ",$dvmag_dvin;
  printf OUTPUT "%14.8e   ",$dvphase_dvin;
  #
  printf OUTPUT "\n";
}
close(INPUT);
close(OUTPUT);

