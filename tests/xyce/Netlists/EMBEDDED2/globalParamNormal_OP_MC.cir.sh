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
$CIRFILE="globalParamNormal_OP_MC.cir";

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

  $V1_analytical_mean = 875.0;
  $V1_analytical_stddev = 10.93;
  $V1_analytical_variance = 119.62;
  $V1_analytical_skew = 0.0;
  $V1_analytical_kurtosis = 3.0;

  print STDERR  "\nTry # $num_tries :\n";

  # Read in the Xyce std output and locate various statistical moments of V(1)
  print STDERR  "\n";

  $V1mean=`grep -m 1 'mean of {V(1)}' $CIRFILE.out`;
  chomp($V1mean);
  $V1mean=~ s/Embedded MC sampling mean of \{V\(1\)\} = ([0-9]*)/\1/;
  print STDERR  "V(1) sampling mean is $V1mean, analytical mean is $V1_analytical_mean\n";

  $V1variance=`grep -m 1 'variance of {V(1)}' $CIRFILE.out`;
  chomp($V1variance);
  $V1variance=~ s/Embedded MC sampling variance of \{V\(1\)\} = ([0-9]*)/\1/;
  print STDERR  "V(1) sampling variance is $V1variance, analytical variance is $V1_analytical_variance\n";

  $V1stddev=`grep -m 1 'stddev of {V(1)}' $CIRFILE.out`;
  chomp($V1stddev);
  $V1stddev=~ s/Embedded MC sampling stddev of \{V\(1\)\} = ([0-9]*)/\1/;
  print STDERR  "V(1) sampling stddev is $V1stddev, analytical stddev is $V1_analytical_stddev\n";

  $V1skew=`grep -m 1 'skew of {V(1)}' $CIRFILE.out`;
  chomp($V1skew);
  $V1skew=~ s/Embedded MC sampling skew of \{V\(1\)\} = ([0-9]*)/\1/;
  print STDERR  "V(1) sampling skew is $V1skew, analytical skew is $V1_analytical_skew\n";

  $V1kurtosis=`grep -m 1 'kurtosis of {V(1)}' $CIRFILE.out`;
  chomp($V1kurtosis);
  $V1kurtosis=~ s/Embedded MC sampling kurtosis of \{V\(1\)\} = ([0-9]*)/\1/;
  print STDERR  "V(1) sampling kurtosis is $V1kurtosis, analytical kurtosis is $V1_analytical_kurtosis\n";

  $passed=1;
  $reltol=5e-2;
  $stddev_reltol=5e-2;

  print STDERR  "\n";

  $result3 = abs($V1mean-$V1_analytical_mean)/abs($V1mean);
  print STDERR  "V1 mean result = $result3 \n"; 
  if ($result3 >= $reltol)
  {
    print STDERR "Try # $num_tries failed  :  V1 mean ($result3 >= $reltol)\n";
    $passed=0;
  }

  $result4 = abs($V1stddev-$V1_analytical_stddev)/abs($V1stddev);
  print STDERR "V1 stddev result = $result4 \n";
  if ($result4 >= $stddev_reltol)
  {
    print STDERR "Try # $num_tries failed  :  V1 stddev ($result4 >= $stddev_reltol)\n";
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

