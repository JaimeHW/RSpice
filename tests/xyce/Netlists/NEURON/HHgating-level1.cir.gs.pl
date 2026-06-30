#!/usr/bin/env perl

# Calculates the steady-state values of the gating variables in the Hodgkin-Huxley equations:
# K activation variable n
# Na activation variable m
# Na inactivation variable h
# equations from Koch pg 147 and 149

sub steadyState
{
  # takes two inputs, assumed to be alpha(V) and beta(V)
  # steady state value of a gating variables is alpha(V) / (alpha(V)+beta(V))
 return $_[0] / ($_[0] + $_[1]);
}

sub alphaN
{
  # takes single input, assumed to be membrane voltage in mV
  $V = $_[0];
  if (abs(10-$V)<0.01)
  {
    return 1.0/(10.0 * exp( (10.0 - $V)/10.0 ) );
  }
  else
  {
    return (10.0 - $V) / (100.0 * (exp( (10.0 - $V)/10.0 ) - 1.0));
  }   
}

sub betaN
{
  # takes single input, assumed to be membrane voltage in mV
  $V = $_[0];
  return 0.125 * exp( -$V/80.0 );
}

sub alphaM
{
  # takes single input, assumed to be membrane voltage in mV
  $V = $_[0];
  if (abs(25.0-$V) < 0.01)
  {
     return 1.0 / exp( (25-$V)/10.0 );
  }
  else
  {
    return (25.0 - $V) / ( 10.0 * ( exp( (25.0-$V)/10.0 ) - 1.0 ) );
  }
}

sub betaM
{
  # takes single input, assumed to be membrane voltage in mV
  $V = $_[0];
  return 4.0 * exp( -$V/18.0 );
}

sub alphaH
{
  # takes single input, assumed to be membrane voltage in mV
  $V = $_[0];
  return 0.07 * exp( -$V/20.0 );
}

sub betaH
{
  # takes single input, assumed to be membrane voltage in mV
  $V = $_[0];
  return 1.0 / ( exp( (30.0 - $V)/10.0 ) + 1.0 );
}

open(INPUT,"HHgating-level1.cir.prn");
open(OUTPUT,">HHgating-level1.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);	#V is at s[1]
  # alpha and beta equations assume resting V of 0; 
  # normally need to subtract Vrest, but test uses Vrest = 0. 
  # convert to mv
  $V = $s[1] * 1000;		
  $n = alphaN($V) / (alphaN($V)+betaN($V));
  $m = alphaM($V) / (alphaM($V)+betaM($V));
  $h = alphaH($V) / (alphaH($V)+betaH($V));
  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e   %14.8e \n",$s[0],$s[1],$n,$m,$h;
}
close(INPUT);
close(OUTPUT);

