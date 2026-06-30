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
$CIRFILE="paramUniformPCE.cir";

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

  $V1_analytical_mean = 600.0;
  $V1_analytical_min  = 500.0;
  $V1_analytical_max  = 750.0;
  $V1_analytical_variance  = 5208.3333333333;
  $V1_analytical_stddev  = 72.1687836487;

  print STDERR  "\nTry # $num_tries :\n";

  # Read in the Xyce std output and locate the mean, max and min of V(1)
  print STDERR  "\n";

  $V1mean=`grep -m 1 'projection PCE mean of {V(1)}' $CIRFILE.out`;
  chomp($V1mean);
  $V1mean=~ s/\(traditional sampling\) projection PCE mean of \{V\(1\)\} = ([0-9]*)/\1/;
  print STDERR  "v(1) PCE mean is $V1mean, analytical mean is $V1_analytical_mean\n";

  $V1stddev=`grep -m 1 'projection PCE stddev of {V(1)}' $CIRFILE.out`;
  chomp($V1stddev);
  $V1stddev=~ s/\(traditional sampling\) projection PCE stddev of \{V\(1\)\} = ([0-9]*)/\1/;
  print STDERR  "V(1) PCE stddev is $V1stddev, analytical stddev is $V1_analytical_stddev\n";

  $passed=1;
  $reltol=5e-2;
  $stddev_reltol=5e-2;

  print STDERR  "\n";

  $result4 = abs($V1mean-$V1_analytical_mean)/abs($V1mean);
  print STDERR  "V1 mean result = $result4 \n"; 
  if ($result4 >= $reltol)
  {
    print STDERR "Try # $num_tries failed  :  V1 mean ($result4 >= $reltol)\n";
    $passed=0;
  }

  $result6 = abs($V1stddev-$V1_analytical_stddev)/abs($V1stddev);
  print STDERR "V1 stddev result = $result6 \n";
  if ($result6 >= $stddev_reltol)
  {
    print STDERR "Try # $num_tries failed  :  V1 stddev ($result6 >= $stddev_reltol)\n";
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

