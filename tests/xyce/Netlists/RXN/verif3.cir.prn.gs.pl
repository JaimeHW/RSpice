#!/usr/bin/env perl

# This script produces the analytical solution for the time-dependent 
# concentrations in the reaction network:
#
# A+B->C (rate constant K1)
# C->A+B (rate constant K2)
#
# The differential equations to be solved are:
#
# dA/dt = -K1*A*B+K2*C
# dB/dt = -K1*A*B+K2*C
# dC/dt =  K1*A*B-K2*C
#
#  By construction, A(t)-A0 = B(t)-B0, and C(t)-C0=-(B(t)-B0), so we reduce
# to a single differential equation to solve:
#
#  dB/dt = -K1*B**2-(K1*C1+K2)*B+K2*C2
# where C1 = (A0-B0) and C2=(C0+B0)
#
#  This can be re-written as:
#
#
#      dB                        
# --------------------------     =   -dt
# K1*B**2+(K1*C1+K2)*B-K2*C2   
#
# or
#
#      dB                        
# --------------------------     =   -dt
# c*B**2+b*B+a
# with c=k1, b=(K1*C1+K2) and a=-K2*C2
#
# Integrating both sides, and realizing that q=4*a*c-b**2 is always negative
# if A0>B0 and all the rate constants are positive, we can use the tabulated
# integral:
#
#
#    /           dB               1       (2*c*B-sqrt(-q)+b)
#    |   -----------------  =  -------  ln(----------------)
#    /    cB**2+b*B+a          sqrt(-q)   (2*c*B+sqrt(-q)+b)
#
#
#
# Let K = sqrt(-q) 
#
#  1   (2*c*B-K+b)
#  - ln(---------)   =  -t + V   (V is an arbitrary constant if integration)
#  K   (2*c*B+K+b)
#
#
# OR:
#   (2*c*B-K+b)
#   (---------)   =  W*exp(-Kt)   (W=exp(K*V), still an arbitrary constant)
#   (2*c*B+K+b)
#
#Solving for B and using the initial conditions, we get:
#  W = (K1*(A0+B0)+K2-K)/(K1*(A0+B0)+K2+K)
#
#   and 
#  (Wexp(-Kt)*(K+K1*C1+K2)+K-(K1*C1+K2))
#B=(-----------------------------------)
#  (     2*K1*(1-Wexp(-Kt))            )    
#
#----------------------------------------------------------------


open(INPUT,"verif3.cir.prn");
open(OUTPUT,">verif3.cir.prn.gs");

$A0 = 100.0;
$B0 = 75.0;
$C0 = 25.0;
$K1 = 1.0e-3;
$K2 = 3.0e-3;

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

  $C1 = $A0-$B0;
  $C2 = $C0+$B0;

  # $K = sqrt($K1^2.0*$C1^2.0+$K1*$K2*(4.0*$C2+2.0*$C1)+$K2^2.0);
  $K = sqrt($K1*$K1*$C1*$C1+$K1*$K2*(4.0*$C2+2.0*$C1)+$K2*$K2);

  $W = ($K1*($A0+$B0)+$K2-$K)/($K1*($A0+$B0)+$K2+$K);

  $B = ((($K1*$C1+$K2)+$K)*$W*exp(-$K*$time)-($K1*$C1+$K2)+$K)/(2*$K1*(1.0-$W*exp(-$K*$time)));

  $A = $A0-($B0-$B);
  $C = $C0+($B0-$B);

  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e   %14.8e\n",$s[0],$s[1],$A,$B,$C;
}
close(INPUT);
close(OUTPUT);

