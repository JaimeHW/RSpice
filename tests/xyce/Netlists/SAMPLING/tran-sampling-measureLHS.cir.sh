#!/usr/bin/env perl

# This script generates a Xyce netlist that creates a large number of 
# normally distributed random numbers, then runs Xyce on it.  It scans
# the Xyce output for the reported random seed.

# It then re-runs Xyce with the "-randseed" option, and checks that the
# random output so generated is identical to the one produced previously.

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
$CIRFILE="tran-sampling-measureLHS.cir";

# Some platforms we test on (esp Windows) use a really bad random number generator.
# To accomodate that I've made the comparison tolerances loose.  Also, if the test
# initially fails the comparison, I run it one more time.  Usually, even if it 
# fails the first time, it will pass the second time.

$num_tries=0;
$finished=0;

while ($num_tries < 2 && $finished == 0)
{
  # Now run that netlist
  $CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
  if (system($CMD) != 0)
  {
      `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
      $xyceexit=1;
  }
  else
  {
      if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
  }

  if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

  $CIRPRN=$CIRFILE;
  $CIRPRN =~ s/\.cir/\.cir\.prn/g;

  $CMD2 = "rm tran-sampling-measureLHS.cir.mt*";
  system($CMD2);

  $R1_analytical_mean = 1.0e+3;
  $R1_analytical_stddev = 1e+2;
  $R1_analytical_variance = 1e+4;
  $R1_analytical_skew = 0.0;
  $R1_analytical_kurtosis = 3.0;

  $maxSine_analytical_mean = 875.0;
  $maxSine_analytical_stddev = 10.93;
  $maxSine_analytical_variance = 119.62;
  $maxSine_analytical_skew = 0.0;
  $maxSine_analytical_kurtosis = 3.0;

  print STDERR  "\nTry # $num_tries :\n";

  # Read in the Xyce std output and locate various statistical moments of R1
  $R1mean=`grep -m 1 'mean of {R1:R}' $CIRFILE.out`;
  chomp($R1mean);
  $R1mean=~ s/LHS sampling mean of \{R1:R\} = ([0-9]*)/\1/;
  print STDERR  "R1 sampling mean is $R1mean, analytical mean is $R1_analytical_mean\n";

  $R1variance=`grep -m 1 'variance of {R1:R}' $CIRFILE.out`;
  chomp($R1variance);
  $R1variance=~ s/LHS sampling variance of \{R1:R\} = ([0-9]*)/\1/;
  print STDERR  "R1 sampling variance is $R1variance, analytical variance is $R1_analytical_variance\n";

  $R1stddev=`grep -m 1 'stddev of {R1:R}' $CIRFILE.out`;
  chomp($R1stddev);
  $R1stddev=~ s/LHS sampling stddev of \{R1:R\} = ([0-9]*)/\1/;
  print STDERR  "R1 sampling stddev is $R1stddev, analytical stddev is $R1_analytical_stddev\n";

  $R1skew=`grep -m 1 'skew of {R1:R}' $CIRFILE.out`;
  chomp($R1skew);
  $R1skew=~ s/LHS sampling skew of \{R1:R\} = ([0-9]*)/\1/;
  print STDERR  "R1 sampling skew is $R1skew, analytical skew is $R1_analytical_skew\n";

  $R1kurtosis=`grep -m 1 'kurtosis of {R1:R}' $CIRFILE.out`;
  chomp($R1kurtosis);
  $R1kurtosis=~ s/LHS sampling kurtosis of \{R1:R\} = ([0-9]*)/\1/;
  print STDERR  "R1 sampling kurtosis is $R1kurtosis, analytical kurtosis is $R1_analytical_kurtosis\n";


  # Read in the Xyce std output and locate various statistical moments of maxSine
  print STDERR  "\n";

  $maxSine_mean=`grep -m 1 'mean of MAXSINE' $CIRFILE.out`;
  chomp($maxSine_mean);
  $maxSine_mean=~ s/LHS sampling mean of MAXSINE = ([0-9]*)/\1/;
  print STDERR  "max Sine sampling mean is $maxSine_mean, analytical mean is $maxSine_analytical_mean\n";

  $maxSine_variance=`grep -m 1 'variance of MAXSINE' $CIRFILE.out`;
  chomp($maxSine_variance);
  $maxSine_variance=~ s/LHS sampling variance of MAXSINE = ([0-9]*)/\1/;
  print STDERR  "max Sine sampling variance is $maxSine_variance, analytical variance is $maxSine_analytical_variance\n";

  $maxSine_stddev=`grep -m 1 'stddev of MAXSINE' $CIRFILE.out`;
  chomp($maxSine_stddev);
  $maxSine_stddev=~ s/LHS sampling stddev of MAXSINE = ([0-9]*)/\1/;
  print STDERR  "max Sine sampling stddev is $maxSine_stddev, analytical stddev is $maxSine_analytical_stddev\n";

  $maxSine_skew=`grep -m 1 'skew of MAXSINE' $CIRFILE.out`;
  chomp($maxSine_skew);
  $maxSine_skew=~ s/LHS sampling skew of MAXSINE = ([0-9]*)/\1/;
  print STDERR  "max Sine sampling skew is $maxSine_skew, analytical skew is $maxSine_analytical_skew\n";

  $maxSine_kurtosis=`grep -m 1 'kurtosis of MAXSINE' $CIRFILE.out`;
  chomp($maxSine_kurtosis);
  $maxSine_kurtosis=~ s/LHS sampling kurtosis of MAXSINE = ([0-9]*)/\1/;
  print STDERR  "max Sine sampling kurtosis is $maxSine_kurtosis, analytical kurtosis is $maxSine_analytical_kurtosis\n";

  $passed=1;
  $reltol=5e-2;
  $stddev_reltol=1e-1;

  print STDERR  "\n";
  $result1 = abs($R1mean-$R1_analytical_mean)/abs($R1mean);
  print STDERR  "R1 mean result = $result1 \n"; 
  if ($result1 >= $reltol)
  {
    print STDERR "Try # $num_tries failed  :  R1 mean ($result1 >= $reltol)\n";
    $passed=0;
  }

  $result2 = abs($R1stddev-$R1_analytical_stddev)/abs($R1stddev);
  print STDERR  "R1 stddev result = $result2 \n";
  if ($result2 >= $stddev_reltol)
  {
    print STDERR "Try # $num_tries failed  :  R1 std dev ($result2 >= $stddev_reltol)\n";
    $passed=0;
  }

  $result3 = abs($maxSine_mean-$maxSine_analytical_mean)/abs($maxSine_mean);
  print STDERR  "maxSin mean result = $result3 \n"; 
  if ($result3 >= $reltol)
  {
    print STDERR "Try # $num_tries failed  :  maxSine mean ($result3 >= $reltol)\n";
    $passed=0;
  }

  $result4 = abs($maxSine_stddev-$maxSine_analytical_stddev)/abs($maxSine_stddev);
  print STDERR "maxSin stddev result = $result4 \n";
  if ($result4 >= $stddev_reltol)
  {
    print STDERR "Try # $num_tries failed  :  maxSine stddev ($result4 >= $stddev_reltol)\n";
    $passed=0;
  }

  if($passed==1)
  {
    $finished=1;
  }
  $num_tries++;
}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl

if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE junk $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
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


if ($passed==1)
{
#    print STDERR "Ran $num_tries, passed the last one.\n";
    print "Exit code = 0\n";
    exit 0;
}
else
{
#    print STDERR "Ran $num_tries times, failed all of them.\n";
    print "Exit code = 2\n";
    exit 2;
}

