#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

$XYCE=$ARGV[0];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

`mkdir -p q1`;

$result=system("(cd q1; $XYCE -doc Q 1) > doc 2> $CIRFILE.err");

$exitcode=0;

# This test passes if Xyce exits without errors.
if ( $result != 0)
{
  $exitcode=10;
}


@texfiles=split(/\s+/,`ls q1/*.tex`);
@qtexfiles=split(/\s+/,`ls q1/Q_1_*.tex`);
$texfilescount=$#texfiles+1; # perl $# syntax returns highest index, not count
$qtexfilescount=$#qtexfiles+1;

if ( $texfilescount == 2 && $qtexfilescount == 2 )
{
  $exitcode=0;
}
else
{
    print STDERR "Must be only 2 *.tex files and 2 Q_1_*.tex files in the q1 directory, and there are $texfilescount tex files and $qtexfilescount in the q1 directory\n" ;
    $exitcode=2;
}

print "Exit code = $exitcode\n";

exit $exitcode;



