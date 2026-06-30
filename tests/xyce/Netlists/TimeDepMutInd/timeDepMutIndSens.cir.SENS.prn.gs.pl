#!/usr/bin/env perl

$cap=1.0e-6;
$res=1.0e+3;


$frequency=1e2;
$coupmag=0.75;
$pi=3.14159265358979;
$l1=.001;
$l2=.001;
$i1=0.001;
$r2=1000;
$m0=$coupmag*sqrt($l1*$l2);
$k=$m0*$pi*$frequency*$i1/$l2;
$a=$r2/$l2;
$b=2*$pi*$frequency;
$denom=$a*$a+$b*$b;
$c1=$k*$a/$denom;

#$i2analytic ={$c1*exp(-$a*time)-$K/$denom*($a*cos($b*time)+$b*sin($b*time))}
#* analytic sensitivity from Maple:
#.param di2_dl1 = {-coupmag * pow(l1 * l2, -0.1e1 / 0.2e1) * pi * frequency * i1 / (r2 * r2 * pow(l2, -0.2e1) + (4 * pi * pi * frequency * frequency)) * (r2 / l2 * cos((2 * pi * frequency * time)) + 0.2e1 * pi * frequency * sin((2 * pi * frequency * time))) / 0.2e1}

open(INPUT,"timeDepMutIndSens.cir.SENS.prn");
open(OUTPUT,">timeDepMutIndSens.cir.SENS.prn.gs");
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }

  @s = split(" ",$line);
  $index=$s[0];
  $timeval=$s[1];
  $i2=$s[2];

  $i2analytic = $c1*exp(-$a*$timeval)-$k/$denom*($a*cos($b*$timeval)+$b*sin($b*$timeval));

  $di2dl1analytic = -$coupmag * (1.0/sqrt($l1 * $l2)) * $pi * $frequency * $i1 / ($r2 * $r2 * (1.0/($l2*$l2)) + (4 * $pi * $pi * $frequency * $frequency)) * ($r2 / $l2 * cos((2 * $pi * $frequency * $timeval)) + 2 * $pi * $frequency * sin((2 * $pi * $frequency * $timeval))) / 2;

  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e \n",$s[0],$s[1],$i2analytic,$di2dl1analytic;

}
close(INPUT);
close(OUTPUT);
