#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

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
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];


$failure=0;

$CIR="bug_1416_tran.cir";
$CSVFILE=$CIR . ".csv";
system("rm -f $CSVFILE");
$CMD="$XYCE $CIR > $CIR.out 2> $CIR.err";
if (system("$CMD") != 0) { print "Exit code = 1\n"; exit 1;}

# Now check if the CSV conforms to the expectations
open(CSV,"<$CSVFILE") or $retval = 14;
if ($retval == 14) {print "Exit code = 14\n"; exit 14; }
$firstline=1;
while ($line = <CSV>)
{

  if ($firstline)
  {
      #check for proper header
      # Make sure first character is not a comma
      $failure=1 if ($line =~ /^\s*,/);
      # Now check for proper fields
      @fields=split(",",$line);
      chomp $fields[2];
      $failure=1 if (!($fields[0] eq "TIME" && $fields[1] eq "V(1)" && $fields[2] =~ m/^I\(V1\)/));
      $firstline=0;
  }
  else
  {
      # Make sure first character is not a comma
      $failure=2 if ($line =~ /^\s*,/);
      # Now check for proper number of comma-separated fields
      @fields=split(",",$line);
      $failure=3 if ($#fields != 2);
      $failure=4 if ($line =~ /End of Xyce/);  # fail if we find this 
  }
}

if ($failure)
{
    open(MYERR,">$CIR.csv.err");
    print MYERR "Header incorrect\n" if $failure=1;
    print MYERR "Line incorrectly beings with comma\n" if $failure=2;
    print MYERR "Wrong number of fields\n" if $failure=3;
    print MYERR "Footer present\n" if $failure=4;
  print "Exit code = 2\n"; exit 2;
}
else
{
  print "Exit code = 0\n"; exit 0;
}

print "Exit code = 1\n"; exit 1;
