#!/usr/bin/env perl

# Run Xyce/Dakota sampling on RC circuit, check stats

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
# deviation of the time constant output by the code.  The mean should be 
# near 1ms, and the standard deviation near about 7%.  The time constant is 
# R*C, and both R and C have normal distributions with 5% standard deviations.
# Therefore, by standard propagation of error formulae, the standard deviation
# of the product should be 5%*sqrt(2) = 7.07%.  Because the sampling won't 
# necessarily nail that number, we'll check that the standard deviation is between
# 7% and 7.2%, and be happy if it is.  We'll be happy if the mean is between
# 0.99 and 1.01 milliseconds.

$CMD="$XYCE -dakota rc.dak $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
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
if (-f 'rcLHS.txt')
{
    # We got the file.  Now scan for the line
    # random_sampling//1/"Moments: Standard"
    # The actual moments appear 3 lines below that, after column and row headings, and a "Data" line
    open(RESFILE,'rcLHS.txt');
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

    if ($mean< 9.9e-4 || $mean >= 1.01e-3)
    {
	$retval=2; # Failed
	print STDERR "Failed because mean $mean outside range (0.99e-3,1.01e-3)\n";
    }
    if ($stddev/$mean< .07 || $stddev/$mean >= 0.072)
    {
	$retval=2; # Failed
	$stddev_rel = $stddev/$mean;
	print STDERR "Failed because stddev $stddev_rel outside range (7%,7.2%)\n";
    }

    print "Exit code = $retval\n";
    exit $retval;
} else {
    print STDERR "Xyce did not produce correct output file rcLHS.txt\n";
    print "Exit code = 14\n";
    exit 14;
}
