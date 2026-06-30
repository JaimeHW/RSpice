#!/usr/bin/env perl

open(INPUT,"rlc.cir.prn");
open(OUTPUT,">rlc.cir.prn.gs");

#$R = 3.0;
#$L = 1.0;
#$C = 0.50;

#$root1=-$R/2+sqrt($R^2-4*$L/$C)/2;
#$root2=-$R/2-sqrt($R^2-4*$L/$C)/2;

#print $root1."\n";
#print $root2."\n";

while ($line = <INPUT>)
{

  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);  
  $a = 10*(exp(-2*$s[1])-exp(-$s[1]))-1.0;
  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e\n",$s[0],$s[1],$s[2],$a;
}
close(INPUT);
close(OUTPUT);
