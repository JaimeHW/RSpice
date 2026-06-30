#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file 

# If Xyce does not produce a prn file, then we return exit code 14.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If Xyce fails, return 10.

# The test netlist runs a B source that *should* produce exactly the same
# voltage on its node as a V source with a global node.  We don't bother
# with a gold standard, just check that the two columns agree.


$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10 }

if (! (-f "$CIRFILE.prn"))
{
    print "Exit code = 14\n";
    exit 14;
}

$fail=0;
open(OUT,"$CIRFILE.prn");
while ($line = <OUT>)
{
    @linelist = split(" ",$line);
    if ($linelist[0] != "Index" && $linelist[0] != "End")
    {
        $val1 = $linelist[2];
        $val2 = $linelist[3];
        if ($val1 != $val2)
        {
            $fail=1;
            print STDERR "Comparison failed at time $linelist[1]: $val1 != $val2\n";
        }
    }
}
close(OUT1);

if ($fail)
{
  print "Exit code = 2\n";
  exit 2
}

print "Exit code = 0\n";
exit 0


