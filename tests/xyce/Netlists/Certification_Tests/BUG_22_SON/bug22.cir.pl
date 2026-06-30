#!/usr/bin/env perl

open(INPUT,"bug22.cir.out");

while($line=<INPUT>)
{
    if ($line=~m/connected to only 1 device Terminal/)
    {
        #print "Exiting with 2!\n";
        exit 2;
    }
    if ($line=~m/does not have a DC path to ground/)
    {
        #print "Exiting with 2!\n";
        exit 2;
    }
}

close(INPUT);
#print "Exiting with 0!\n";
exit 0;

