#!/usr/bin/env perl

open(INPUT,"rc_osc.cir.prn");
open(OUTPUT,">rc_osc.cir.prn.gs");

# These are values from the netlist.
$R1 = 1000.0;
$C1 = 2e-6;
$mpi=3.1415927;
$RC_const = $R1 * $C1;
$FREQ=1e5;
$ampl=0.0008;

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $sinmult = $ampl * cos (2.0*$mpi*($FREQ*$s[1] ));
  $a = exp(-$s[1]/$RC_const) * $sinmult;
  $a = 0.002 + $ampl * exp(-$s[1]/$RC_const) - $sinmult;
  printf OUTPUT "%3d   %14.8e   %14.8e\n",$s[0],$s[1],$a
}
close(INPUT);
close(OUTPUT);
