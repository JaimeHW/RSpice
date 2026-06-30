#!/usr/bin/env perl

# This script generates a Xyce netlist that creates a large number of 
# normally distributed random numbers, then runs Xyce on it.  It then 
# reads the output and checks that the random numbers actually have the
# right mean and standard deviation.

#use XyceRegression::Tools;

#$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CIRFILE="testagauss.cir";

$mean_requested = 100;
$abs_dev = 10;   # agauss returns random deviates around mean +/- abs_dev, not
                 # +/- infinity.
$sigma_unit = 10; # agauss uses abs_dev/sigma_unit as the standard
                 # deviation of the gaussian distribution.  I.e. abs_dev is
                 # sigma_unit standard deviations of the distribution.
$sigma=$abs_dev/$sigma_unit;

$NUM_RANDS=10000;  # number of samples to generate

open (CIROUT, ">$CIRFILE") || die "Cannout open circuit file for output.";

print CIROUT "*TEST of gaussian random deviates\n";
print CIROUT "I1 1 0 DC -1\n";

for ($i=1;$i<=$NUM_RANDS;$i++)
{
    print CIROUT "R$i 1 0 {agauss($mean_requested,$abs_dev,$sigma_unit)}\n";
}

print CIROUT "\n.DC I1 -1 -1 -.1\n";
print CIROUT ".print DC FORMAT=NOINDEX PRECISION=19 ";
for ($i=1;$i<=$NUM_RANDS;$i++)
{
    print CIROUT "R$i:R ";
}
print CIROUT "\n.end\n";
close (CIROUT);

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

# Now read in the Xyce output and compute the mean and standard deviation
# of the resistances we generated.

    open (CIRPRN,"<$CIRPRN") || die "cannot open $CIRPRN for read";
    $firstline=1;
    while (<CIRPRN>)
    {
        if ($firstline==1) { $firstline=0; next; }
        (@resistances)=split(" ");
        
        if ($resistances[0] == "End") {next;}
        
        $sum=0;
        $sumsq=0;
        $sumcub=0;
        $sum4=0;
        
        for ($i=0;$i<=$#resistances; $i++)
        {
            $sum += $resistances[$i];
            $sumsq += $resistances[$i]* $resistances[$i];
            $sumcub += $resistances[$i]* $resistances[$i]*$resistances[$i];
            $sum4 += $resistances[$i]* $resistances[$i]*$resistances[$i]*$resistances[$i];
        }
        
        $num_samples=$#resistances+1;
        
        print STDERR "There are $num_samples samples.  Sum of samples=$sum, sum of squares=$sumsq\n";
        
        $mean=$sum/$num_samples;
        $mean_sq=$sumsq/$num_samples;
        $mean_cub=$sumcub/$num_samples;
        $mean_4=$sum4/$num_samples;
        $std_dev=sqrt($mean_sq-$mean*$mean);
        $mu3=$mean_cub-3*$mean*$mean_sq+2*$mean*$mean*$mean;
        $mu4=$mean_4-4*$mean*$mean_cub+6*$mean*$mean*$mean_sq-3*$mean*$mean*$mean*$mean;
        
        print STDERR "Mean: $mean.   Sigma=$std_dev \n";
        print STDERR "Expected mean= $mean_requested.  Expected stddev=$sigma\n";
        print STDERR "Error in mean:  "; print STDERR abs($mean-$mean_requested); print "\n";
        print STDERR "Error in sigma:  "; print STDERR abs($std_dev-$sigma); print "\n";
        print STDERR "Skew (should be near zero): " ; print STDERR $mu3/($std_dev*$std_dev*$std_dev); print STDERR " \n"; 
        print STDERR "Kurtosis (should be near 3.0): " ; print STDERR $mu4/($std_dev*$std_dev*$std_dev*$std_dev); print STDERR " \n"; 

        if (abs($mean-$mean_requested) < 5e-2 && abs($std_dev-$sigma) < 5e-2)
        {
            $passed=1;
            $finished=1;
        }
        else
        {
            # failed this time.  Perhaps try again
            # It is unusual for the first try to get a set of random
            # numbers that fail the test, but it does happen.  It is 
            # *extremely* unusual for it to happen twice in a row.
            $passed=0;
        }
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
    print STDERR "Ran $num_tries, passed the last one.\n";
    print "Exit code = 0\n";
    exit 0;
}
else
{
    print STDERR "Ran $num_tries times, failed all of them.\n";
    print "Exit code = 2\n";
    exit 2;
}

