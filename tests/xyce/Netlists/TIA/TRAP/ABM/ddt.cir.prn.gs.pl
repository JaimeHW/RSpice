#!/usr/bin/env perl

open(INPUT,"ddt.cir.prn");
open(OUTPUT,">ddt.cir.prn.gs");

# Estimate PI.
$pi = 2.0 * asin(1.0);

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a = $pi * cos ($pi * $s[1] );
  if ($s[1] <= 0.0)
  {
    $a = 0.0;
  }

  printf OUTPUT "%3d   %14.8e   %14.8e    ",$s[0],$s[1],$s[2];
  printf OUTPUT "%14.8e\n", $a;
}
close(INPUT);
close(OUTPUT);

sub asin 
{
   my($x) = @_;
   my $ret = atan2($x, sqrt(1 - $x * $x)) ;
   return $ret;
}


