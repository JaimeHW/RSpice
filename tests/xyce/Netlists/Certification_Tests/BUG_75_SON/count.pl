#!/usr/bin/env perl

open(INPUT,"output");

while($line=<INPUT>)
{
    if ($line =~m/Undefined inductor L2 in mutual inductor K3 definition/)
    {
        close(INPUT);
        exit 0;
    }
}
close(INPUT);
exit 2;
