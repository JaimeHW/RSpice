#!/usr/bin/env perl

open(INPUT,"acos_asin.cir.prn");
open(OUTPUT,">acos_asin.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  $a[0] = asin($s[1]);
  $a[1] = acos($s[1]);

  printf OUTPUT "%3d  ",$s[0];
  printf OUTPUT "   %14.8e   %14.8e   %14.8e   \n",$s[1],$a[0],$a[1];
}
close(INPUT);
close(OUTPUT);

sub acos 
{

   my($x) = @_;
   my $ret = atan2(sqrt(1 - $x**2), $x);
   return $ret;
}

sub asin 
{ 
   my($x) = @_;
   my $ret = atan2($x, sqrt(1 - $x * $x)) ;
   return $ret;
 }



