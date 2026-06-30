#!/usr/bin/env perl

open(INPUT,"tempcap_instance.cir.prn");
open(OUTPUT,">tempcap_instance.cir.prn.gs");

# These are values from the netlist.
$R1 = 1000.0;
#$baseCap = 20.0e-9;
$baseCap = 1.0e-6;
$temp=727.0;
$tnom=55.0;
$tdif=$temp-$tnom;
$TC1=0.00177;
$TC2=-6.3e-7;

$factor = 1.0 + ($TC1)*$tdif + ($TC2)*$tdif*$tdif;
$C1 = $baseCap*$factor;

#
#  tdif = temp - tnom;
#  factor = 1.0 + (TC1)*tdif + (TC2)*tdif*tdif;
#  C = baseCap*factor;
# 

$RC_const = $R1 * $C1;

# print "baseCap is $baseCap\n";
# print "temp is $temp\n";
# print "tnom is $tnom\n";
# print "tdif is $tdif\n";
# print "TC1  is $TC1 \n";
# print "TC2  is $TC2 \n";
# print "C1 is $C1\n";
# print "R1 is $R1\n";
# print "RC time constant is $RC_const\n";

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a = exp(-$s[1]/$RC_const);
  printf OUTPUT "%3d   %14.8e   %14.8e   \n",$s[0],$s[1],$a;
}
close(INPUT);
close(OUTPUT);
