#!/usr/bin/env perl

open(INPUT,"bug_460.cir.prn");
open(OUTPUT,">bug_460.cir.prn.gs");
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }

  @s = split(" ",$line);

  # Handle header information:
  if ($s[0] eq "TITLE")
  {
    print OUTPUT "$line"; 
    next; 
  }
  if ($s[0] eq "VARIABLES")
  {
    print OUTPUT "$line"; 
    next; 
  }
  if ($s[0] eq "\"")
  {
    print OUTPUT "$line"; 
    next; 
  }
  if ($s[0] eq "DATASETAUXDATA")
  {
    print OUTPUT "$line"; 
    next; 
  }
  if ($s[0] eq "ZONE")
  {
    print OUTPUT "$line"; 
    next; 
  }

  # handle the step variables, which (for tecplot format)
  # are specified inside of AUXDATA:
  if ($s[0] eq "AUXDATA")
  {
    print OUTPUT "$line"; 

    if ($s[1] eq "R1")
    {
      @tmp = split("\"",$s[4]);
      $R1 = $tmp[0];
      # print "R1 is $R1\n";
    }

    if ($s[1] eq "v_amplitude")
    {
      @tmp = split("\"",$s[4]);
      $v_ampl = $tmp[0];
    }
    next; 
  }

  $time= $s[0];
  $twopi = 8.0 * atan2 (1.0,1.0);
  $v_a = 5.0 + $v_ampl * sin($twopi*$time*0.05);

  # v_b is calcualted by:
  #
  # (v_a - v_b)/R1 = v_b/35.0;   (35 is the sum of the other resistors)
  #
  # v_b = v_a * (35/R1)/(1.0 + (35/R1))
  #

  $tmp = 35.0/$R1;
  $v_b = $v_a * $tmp / (1.0 + $tmp);
  $exp1 = abs($v_a) - 10.0;
  $exp2 = (($v_b+2.0)**2.0)*1.0e+3;
  
  printf OUTPUT "%14.8e   %14.8e   %14.8e   %14.8e   %14.8e \n",
     $time,$v_a,$v_b,$exp1,$exp2;
}
close(INPUT);
close(OUTPUT);
