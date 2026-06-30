#!/usr/bin/env perl

# Evaluates the analytical solution for the synapse model based on NEURON's exp2syn
# assuming presynaptic spike happens at (or very close to) time 0, with no delay in
# postsynaptic response.

# from exp2syn.mod:
# i = g*(V-Esyn)
# g = B-A
# since A and B both change by exponential decay, 
# A(t) = A_0 * exp(-t/tau1)
# B(t) = B_0 * exp(-t/tau2)
# A_0 = B_0 = factor*weight
# factor = 1 / (-exp(-tp/tau1) + exp(-tp/tau2))
# weight is user-specified

open(INPUT,"synapticCurrent-level3.cir.prn");
open(OUTPUT,">synapticCurrent-level3.cir.prn.gs");

# parameter values - these must match those used in netlist
# these are taken from the values I used in NEURON network sims
$Esyn = 0.0;
$tau1 = 0.002;	# 2 ms
$tau2 = 0.0063;	# 6.3 ms
$weight = 0.01e-6;	# S (0.01 uS)
$Vthresh = 0.01;
$t0 = 10000.0;	# just some large value for now

#derived quantitites 
$tp = ($tau1*$tau2)/($tau2 - $tau1) * log($tau2/$tau1);
$factor = -exp(-$tp/$tau1) + exp(-$tp/$tau2);
$factor = 1/$factor;
$A0 = 0;
$B0 = 0;
$A = 0;
$B = 0;

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }

  # for data lines, get time and voltages; calculate postsynaptic current
  @s = split(" ",$line);	
  $t = $s[1];		# time
  $preV = $s[2];	# presynaptic voltage
  $postV = $s[3];	# postsynaptic voltage

  if ( ($t < $t0) )
  {
    if ($preV > $Vthresh)
    {
      $A0 = $factor*$weight;
      $B0 = $factor*$weight;
      $t0 = $t;
    }
  }

  if ($t>=$t0)
  { 
    $A = $A0 * exp(-($t-$t0)/$tau1);
    $B = $B0 * exp(-($t-$t0)/$tau2);
  } 

  $i = ($B-$A)*($postV-$Esyn);
  
  # output has index, time, preV, postV, and i
  # circuit we're comparing to doesn't print postsynaptic current directly; it prints
  # resulting current through a fixed voltage source, which is the negative of the
  # synaptic current.  So print -$i here.
  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e   %14.8e \n", $s[0], $t, $preV, $postV, -$i;
}

close(INPUT);
close(OUTPUT);
