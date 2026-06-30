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
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$XPLAT_DIFF = $ARGV[1];
$XPLAT_DIFF =~ s/xyce_verify/xplat_diff/;

$DAT1="FET1_data.txt";
$DAT2 = $GOLDPRN;
$DAT2 =~ s/bug520\.cir\.prn/FET1_data\.txt/;
#print "\nDAT1 = $DAT1\n";
#print "DAT2 = $DAT2\n";

`rm -f $DAT1`;
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }

if (not -s "$DAT1") { print "Exit code = 14\n"; exit 14; }

system("grep -v current $DAT1 | grep -v charge | grep -v dI > $DAT1.dat");
system("grep -v current $DAT2 | grep -v charge | grep -v dI > $DAT2.dat");

# This is just a nonlinear poisson calculation, so ignore all the electrode
# stuff (currents, charges, etc) except for the potential.
$CMD="$XPLAT_DIFF $DAT1.dat $DAT2.dat > $DAT1.out 2> $DAT1.err";
$result = system("$CMD");
if ( not -z "$DAT1.out" ) { $failure = 1; }

system("rm $DAT1.dat");
system("rm $DAT2.dat");

if ($failure)
{
  print "Exit code = 2\n"; exit 2;
}
else
{
  print "Exit code = 0\n"; exit 0;
}

print "Exit code = 1\n"; exit 1;
