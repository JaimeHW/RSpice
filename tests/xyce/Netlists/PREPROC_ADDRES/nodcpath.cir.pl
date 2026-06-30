#!/usr/bin/env perl

#Strip nodcpath.cir_xyce.cir of any random '\r' characters that might be at the
#end of a line and put these augmented lines in a temp file
open(INPUT,"nodcpath.cir_xyce.cir");
open(OUTPUT,">nodcpath.cir_xyce.cir.temp");

while ($line = <INPUT>)
{
    $line =~s/\r//;
    print OUTPUT "$line";
    next;
}

close(INPUT);
close(OUTPUT);
