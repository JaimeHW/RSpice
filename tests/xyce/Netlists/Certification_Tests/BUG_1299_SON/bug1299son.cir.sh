#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10
# Otherwise we return the appropriate exit code consistent with the
# status of the run.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

$basecir=$CIRFILE;
$basecir =~ s/.cir//;

@cirlist=("$CIRFILE", "${basecir}_fixedinterval.net", "${basecir}_fixedinterval_gear.net","${basecir}_lastcycle.net","${basecir}_lastcycle_gear.net", "${basecir}_lastcycle_fixedinterval.net","${basecir}_lastcycle_fixedinterval_gear.net");

foreach $cirname (@cirlist)
{
    print "Run $cirname\n";
    `rm -f $cirname.prn > /dev/null 2>&1`;
    `rm -f $cirname.err > /dev/null 2>&1`;
    $CMD = "$XYCE $cirname > $cirname.out 2> $cirname.err";
    if ( system("$CMD") != 0 )
    {
        `echo "Xyce EXITED WITH ERROR! on $cirname" >> $cirname.err`;
        print STDERR "Xyce EXITED WITH ERROR! on $cirname\n";
        $xyceexit = 1;
    }
    else
    {
        if ( -z "$cirname.err" ) { `rm -f $cirname.err`; }
    }
}
if (defined($xyceexit)) { print "Exit code = 10\n"; exit 10; }

# Now we check that the two "fixed interval" runs match the base run as far as xyce_verify cares
foreach $cirname ("${basecir}_fixedinterval.net", "${basecir}_fixedinterval_gear.net")
{
    print "Testing ${cirname} against unrestricted output...\n";
    $CMD = "$XYCE_VERIFY $CIRFILE ${CIRFILE}.prn ${cirname}.prn > ${cirname}.prn.out 2> ${cirname}.prn.err";
    if ( system("$CMD") != 0 )
    {
        `echo "Test Failed, signals do not match!" >> ${cirname}.prn.out`;
        print STDERR "Test failed, ${cirname}.prn does not match signal in ${CIRFILE}.prn\n";
        print "Exit code = 2\n";
        exit 2;
    }

    print "Testing ${cirname} has correct intervals...\n";
    # And if they're right, check that each "fixed interval" file is in fact at 0.25s fixed intervals
    open(OUTPUTFILE, "${cirname}.prn");
    $i=0;
    while ($line=<OUTPUTFILE>)
    {
        if($line =~ /^[IE]/)
        {
            # don't do anything for header or footer line of output
        }
        else
        {
            @fields=split(" ",$line);
            $t=$i*0.25;
            if ($fields[1] != $t)
            {
                print STDERR "Line $i of file ${cirname}.prn says time is $fields[1], but it should be $t\n";
                print "Exit code = 2\n";
                exit 2;
            }
            $i++;
        }
    }
    close(OUTPUTFILE);
}

# Now we check that the two "last cycle" runs match the base run as far as xyce_verify cares
# xyce_verify.pl uses the times in the test file for its interpolation, so it is OK that
# the test file is missing early data.
foreach $cirname ("${basecir}_lastcycle.net", "${basecir}_lastcycle_gear.net")
{
    print "Testing ${cirname} against unrestricted output...\n";
    $CMD = "$XYCE_VERIFY $CIRFILE ${CIRFILE}.prn ${cirname}.prn > ${cirname}.prn.out 2> ${cirname}.prn.err";
    if ( system("$CMD") != 0 )
    {
        `echo "Test Failed, signals do not match!" >> ${cirname}.prn.out`;
        print STDERR "Test failed, ${cirname}.prn does not match signal in ${CIRFILE}.prn\n";
        print "Exit code = 2\n";
        exit 2;
    }

    print "Testing ${cirname} begins at correct time...\n";
    # And if they're right, check that each "fixed interval" file is in fact at 0.25s fixed intervals
    open(OUTPUTFILE, "${cirname}.prn");
    $i=0;
    while ($line=<OUTPUTFILE>)
    {
        if($line =~ /^[IE]/)
        {
            # don't do anything for header or footer line of output
        }
        else
        {
            @fields=split(" ",$line);
            if ($i==0)
            {
                $firsttime=$fields[1];
            }
            else
            {
                $lasttime=$fields[1];
            }
            $i++;
        }
    }
    close(OUTPUTFILE);
    if ($firsttime != 4.0)
    {
        print STDERR "File ${cirname}.prn says first time is $firsttime, but it should be 4.0\n";
        print "Exit code = 2\n";
        exit 2;
    }
    if ($lasttime != 5.0)
    {
        print STDERR "File ${cirname}.prn says last time is $firsttime, but it should be 5.0\n";
        print "Exit code = 2\n";
        exit 2;
    }
}

# finally, we test that the two "lastcycle_fixedinterval" files start at correct time *and* have correct
# intervals
foreach $cirname ("${basecir}_lastcycle_fixedinterval.net", "${basecir}_lastcycle_fixedinterval_gear.net")
{
    print "Testing ${cirname} against unrestricted output...\n";
    $CMD = "$XYCE_VERIFY $CIRFILE ${CIRFILE}.prn ${cirname}.prn > ${cirname}.prn.out 2> ${cirname}.prn.err";
    if ( system("$CMD") != 0 )
    {
        `echo "Test Failed, signals do not match!" >> ${cirname}.prn.out`;
        print STDERR "Test failed, ${cirname}.prn does not match signal in ${CIRFILE}.prn\n";
        print "Exit code = 2\n";
        exit 2;
    }

    print "Testing ${cirname} has correct intervals...\n";
    # And if they're right, check that each "fixed interval" file is in fact at 0.25s fixed intervals
    open(OUTPUTFILE, "${cirname}.prn");
    $i=0;
    while ($line=<OUTPUTFILE>)
    {
        if($line =~ /^[IE]/)
        {
            # don't do anything for header or footer line of output
        }
        else
        {
            @fields=split(" ",$line);
            $t=$i*0.25+4.0;
            if ($fields[1] != $t)
            {
                print STDERR "Line $i of file ${cirname}.prn says time is $fields[1], but it should be $t\n";
                print "Exit code = 2\n";
                exit 2;
            }
            $i++;
        }
    }
    close(OUTPUTFILE);
}

print "Exit code = 0\n";
exit 0;
