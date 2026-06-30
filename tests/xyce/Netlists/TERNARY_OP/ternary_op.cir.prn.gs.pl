#!/usr/bin/env perl

$infile=$ARGV[0];
$outfile=$infile . ".gs";

open (INPUT,$infile);
open (OUTPUT,">$outfile");
while ($line = <INPUT>)
{
    if ($line =~ m/^[IE]/)
    {
        print OUTPUT $line;
        next;
    }
    @s=split(" ",$line);
    printf OUTPUT "%3d  %14.8e",$s[0],$s[1];
    for ($i=2;$i<=$#s;$i++)
    {
        printf OUTPUT "  %14.8e",0.0;
    }
    printf OUTPUT "\n";
}
close (INPUT);
close (OUTPUT);
