#!/usr/bin/env perl

use Math::Trig;

$Epsilon = 1.e-12;

open(INPUT,"Bsrc_B1.cir.prn");
open(OUTPUT,">Bsrc_B1.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  $a[0] = tanh($s[2]);

  $arg = $a[0];

  if ($arg < ($Epsilon -1.0))
  {
    $arg = $Epsilon - 1.0;
  }
  elsif ($arg > (1.0 - $Epsilon))
  {
    $arg = 1.0 - $Epsilon;
  }

  $a[1] = atanh($arg);

  printf OUTPUT "%3d  ",$s[0];
  printf OUTPUT "   %14.8e   %14.8e   %14.8e   %14.8e   \n",$s[1],$s[2],$a[0],$a[1];
}
close(INPUT);
close(OUTPUT);

#sub tan 
#{ 
#sin($_[0]) / cos($_[0])  
#}

