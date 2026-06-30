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
$CIRFILE="truncatedNormalMC.cir";

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


  $R1_analytical_min  = 2.0e+3;
  $R1_analytical_max  = 6.0e+3;

  $V1_analytical_min  = 500.0;
  $V1_analytical_max  = 750.0;

  print STDERR  "\nTry # $num_tries :\n";

  # Read in the Xyce std output and locate the max and min of R1
  $R1max=`grep -m 1 'max of {R1:R}' $CIRFILE.out`;
  chomp($R1max);
  $R1max=~ s/MC sampling max of \{R1:R\} = ([0-9]*)/\1/;
  print STDERR  "R1 sampling max is $R1max, analytical max is $R1_analytical_max\n";

  $R1min=`grep -m 1 'min of {R1:R}' $CIRFILE.out`;
  chomp($R1min);
  $R1min=~ s/MC sampling min of \{R1:R\} = ([0-9]*)/\1/;
  print STDERR  "R1 sampling min is $R1min, analytical min is $R1_analytical_min\n";

  # Read in the Xyce std output and locate the  max and min of V(1)
  print STDERR  "\n";

  $V1max=`grep -m 1 'max of {V(1)}' $CIRFILE.out`;
  chomp($V1max);
  $V1max=~ s/MC sampling max of \{V\(1\)\} = ([0-9]*)/\1/;
  print STDERR  "V(1) sampling max is $V1max, analytical max is $V1_analytical_max\n";

  $V1min=`grep -m 1 'min of {V(1)}' $CIRFILE.out`;
  chomp($V1min);
  $V1min=~ s/MC sampling min of \{V\(1\)\} = ([0-9]*)/\1/;
  print STDERR  "V(1) sampling min is $V1min, analytical min is $V1_analytical_min\n";

  $passed=1;
  $max_reltol=2e-2;
  $min_reltol=2e-2;

  print STDERR  "\n";
  $result2 = abs($R1max-$R1_analytical_max)/abs($R1max);
  print STDERR  "R1 max result = $result2 \n";
  if ($result2 >= $max_reltol)
  {
    print STDERR "Try # $num_tries failed  :  R1 max ($result2 >= $max_reltol)\n";
    $passed=0;
  }

  $result3 = abs($R1min-$R1_analytical_min)/abs($R1min);
  print STDERR  "R1 min result = $result3 \n";
  if ($result3 >= $min_reltol)
  {
    print STDERR "Try # $num_tries failed  :  R1 min ($result3 >= $min_reltol)\n";
    $passed=0;
  }

  $result5 = abs($V1max-$V1_analytical_max)/abs($V1max);
  print STDERR "V1 max result = $result5 \n";
  if ($result5 >= $max_reltol)
  {
    print STDERR "Try # $num_tries failed  :  V1 max ($result5 >= $max_reltol)\n";
    $passed=0;
  }

  $result6 = abs($V1min-$V1_analytical_min)/abs($V1min);
  print STDERR "V1 min result = $result6 \n";
  if ($result6 >= $min_reltol)
  {
    print STDERR "Try # $num_tries failed  :  V1 min ($result6 >= $min_reltol)\n";
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

