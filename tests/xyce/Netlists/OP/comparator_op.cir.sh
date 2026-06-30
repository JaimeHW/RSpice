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

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];

$CIR1="comparator_op.cir";

# first comparison

if (system("$XYCE $CIR1 > $CIR1.out 2> $CIR1.err"))
{
  print "Exit code = 10\n";
  exit 10;
}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl

if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIR1 junk $CIR1.prn > $CIR1.prn.out 2>&1 $CIR1.prn.err"))
    {
        print "Exit code = 2 \n";
        exit 2;
    }
    else
    {
        print "Exit code = 0 \n";
        exit 0;
    }
}

`sed -n '/Operating point information/,/---------------/p' $CIR1.out | grep -f patfile > $CIR1.stripped.out`;
if (system("diff -i -w $CIR1.stripped.out  $CIR1.op.gold > $CIR1.op.out 2> $CIR1.op.err"))
{
  print "Exit code = 2\n";
  exit 2;
}

print "Exit code = 0\n";
exit 0;

