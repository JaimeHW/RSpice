#!/usr/bin/env perl

# Run Xyce/Dakota sampling on resistor circuit, check stats

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file


$XYCE = $ARGV[0];
$CIRFILE=$ARGV[3];

#Clear out any previous run's droppings
`rm -f LHS*.out *.txt $CIRFILE.mt0 $CIRFILE.*.prn S4`;

# This test is just a Dakota sampling, and we will not be comparing against a 
# gold standard.  We'll run the sampling, then look at the mean and standard
# deviation of the voltage at V(B) output by the code.  The mean should be 
# near 2.5V, and the standard deviation near about 13.9%.  The voltage is 
# 5V*R/(R+10), and  R has normal distributions with 25% standard deviations.
# Therefore, by standard propagation of error formulae, the standard deviation
# of the product should be (2.5/10)*sqrt(5)/4=13.975%.  Because the sampling won't 
# necessarily nail that number, we'll check that the standard deviation is between
# 13% and 14%, and be happy if it is.  We'll be happy if the mean is between
# 2.4 and 2.6 V.

$CMD="$XYCE -dakota res.dak $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
$sysret = system($CMD);
if ($sysret != 0)
{
    $actualRet=$sysret >> 8;
    $signalRet = $sysret & 127;
    $withCore = $sysret & 128;
    print STDERR "Xyce died with return code $actualRet.\n";
    $retval=10;    # Xyce exited with error
    if ($signalRet != 0)
    {
	print STDERR "Crashed with signal $signalRet\n";
	print STDERR "Core dumped\n" if $withCore;
	$retval=13;   # Xyce crashed
    }
    print "Exit code = $retval\n";
    exit $retval;
}

# Ok, Xyce did not crash.  Check for existence of Dakota output file
if (-f 'resLHS.txt')
{
    # We got the file.  Now scan for the line
    # random_sampling//1/"Moments: Standard"
    # The actual moments appear 3 lines below that, after column and row headings, and a "Data" line
    open(RESFILE,'resLHS.txt');
    $foundheader=0;
    while (<RESFILE>)
    {
	chomp();
	if (/random_sampling.*Moments: Standard/)
	{
	    $foundheader=1;
	    $skipping=0;
	    next;
	}
	if ($foundheader && $skipping < 3)
	{
	    $skipping++;
	    next;
	}
	if ($foundheader && $skipping==3)
	{
	    $mean=$_;
	    chomp($mean);
	    $mean =~ s/ //g;
	    $stddev=<RESFILE>;
	    chomp($stddev);
	    $stddev =~ s/ //g;
	    last;
	}
    }
    $retval=0;

    if ($mean< 2.4 || $mean >= 2.6)
    {
	$retval=2; # Failed
	print STDERR "Failed because mean $mean outside range (0.99e-3,1.01e-3)\n";
    }
    if ($stddev/$mean< 0.13 || $stddev/$mean >= 0.14)
    {
	$retval=2; # Failed
	$stddev_rel = $stddev/$mean;
	print STDERR "Failed because stddev $stddev_rel outside range (13.8%,14%)\n";
    }

    print "Exit code = $retval\n";
    exit $retval;
} else {
    print STDERR "Xyce did not produce correct output file rcLHS.txt\n";
    print "Exit code = 14\n";
    exit 14;
}
