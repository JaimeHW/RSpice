#!/usr/bin/env perl

use Math::Complex;

open(INPUT,"abmpow3.cir.prn");
open(OUTPUT,">abmpow3.cir.prn.gs");

while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);

  # Note:  The new Xyce expression library implements raising negative
  # numbers to fractional powers correctly.  However, it does this by 
  # using complex numbers internally.  The correctly solution to 
  # (-V(1))**2.1, when -V(1) is negative, is a complex number.
  #
  # However, for a DC calculation, Xyce assumes everything is real, so 
  # it only uses the real part of an expression evaluation.  As that is 
  # what Xyce is doing, that is what is being evaluated here by perl as 
  # well.  
  #
  # This is really not correct, because for real-valued inputs and outputs, a 
  # fractional power applied to a negative number isn't valid. That 
  # calculation would, for a purely real-numbered computation, return Nan.  But 
  # for the code to be well-behaved and stable, it is important to have the Bsrc 
  # evaluate to something well-behaved.  So, having the Bsrc use the real part 
  # of the complex result is about the best we can do in this case.
  $a[0] = $s[0];
  $a[1] = $s[1];

  $ac1 = cplxe($a[1], 0.0); 
  $ac2 = cplxe(0.0,0.0); 
  $ac3 = cplxe(0.0,0.0);

  $ac2 = (( $ac1)**(2.1)) ;
  $ac3 = ((-$ac1)**(3.1)) ;

  $end=3;
  printf OUTPUT "%2g  ",$s[0];
  printf OUTPUT "    %14.8e",$a[1];
  printf OUTPUT "    %14.8e",Re($ac2);
  printf OUTPUT "  %14.8e  \n",Re($ac3);
}
close(INPUT);
close(OUTPUT);
