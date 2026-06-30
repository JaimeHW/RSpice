#!/usr/bin/env perl

$INPUTPRN=$ARGV[0];
$OUTPUTGS=$INPUTPRN.".gs";
open(INPUT,"$INPUTPRN") || die "Cannot open $INPUTPRN for reading";
open(OUTPUT,">$OUTPUTGS") || die "Cannot open $OUTPUTGS for writing";
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e\n",$s[0],$s[1],$s[2],$s[3],$s[4],0.0;
}
close(INPUT);
close(OUTPUT);
