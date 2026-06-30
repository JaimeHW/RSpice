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
    # Two sweep vars, four voltages for single transistor
    printf OUTPUT "%3d  %14.8e  %14.8e  %14.8e  %14.8e  %14.8e  %14.8e",$s[0],$s[1],$s[2],$s[3],$s[4],$s[5],$s[6];
    # gold standard for the multiplicity transistor should be 10 times larger
    for ($i=7;$i<=$#s;$i++)
    {
        printf OUTPUT "  %14.8e",$s[$i-4]*10.0;
    }
    printf OUTPUT "\n";
}
close (INPUT);
close (OUTPUT);
