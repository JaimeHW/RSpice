#!/usr/bin/env perl

# Evaluates the analytical solution for a passive point neuron or membrane patch
# injected with constant current.  Patch just behaves like an RC circuit.
# equations from Koch pg 7-11

open(INPUT,"passivePatch-level1.cir.prn");
open(OUTPUT,">passivePatch-level1.cir.prn.gs");

# parameter values - these must match those used in netlist
$Vrest = -0.07;
$Cm = 1.0e-10;	# 100 pF
$Rm = 1.0e8;	# want R*C = 10 ms
$Iinj = -0.1e-9;	# -0.1 pA   

#derived quantitites used in calculating membrane voltage:
$v1 = $Vrest + $Rm*$Iinj;
$v0 = -$Rm*$Iinj;

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }

  # for data lines, get time and calculate expected V
  @s = split(" ",$line);	
  $t= $s[1];	# time

  $V = $v0 * exp(-$t/($Rm*$Cm)) + $v1;
  
  printf OUTPUT "%3d   %14.8e   %14.8e \n", $s[0], $t, $V;
}

close(INPUT);
close(OUTPUT);
