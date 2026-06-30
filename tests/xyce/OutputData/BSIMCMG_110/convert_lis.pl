#!/usr/local/bin/perl
$i=0;
while (<>)
{
    chomp;
    s/ */ /;
    @foo=split(' ');
#    print "$foo[0],$foo[1],$foo[2]\n";
    printf("%d     %e     %e    %e\n",$i,$foo[0],$foo[1]+1.0,$foo[2]+1.0);
    $i++;
}
