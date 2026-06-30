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
$XYCE_VERIFY=$ARGV[1];

$TRANSLATESCRIPT = $XYCE_VERIFY;
$TRANSLATESCRIPT =~ s/xyce_verify.pl/convertToPrn.py/;

$CIRFILE=$ARGV[3];

#Clean up from any previous runs
`rm -f *.raw *.out *.err`;

# Now repeat for binary and ascii
for ($i = 0; $i<2; $i++)
{
    $ASCII="BINARY";
    $MINUSA="";
    if ($i == 1)
    {
        $ASCII="ASCII";
        $MINUSA="-a";
    }

# Run Xyce and create a rawfile
    $result=system("$XYCE $MINUSA $CIRFILE > $CIRFILE.out 2> $CIRFILE.err");
    if ($result != 0)
    {
        print STDERR "Xyce crashed trying to run $ASCII rawfile\n";
        print "Exit code = 14 \n";
        exit 14;
    }
    if ( ! -e "$CIRFILE.raw" )
    {
        print STDERR "Xyce did not produce a rawfile\n";
        print "Exit code = 14\n";
        exit 14;
    }
    
# Convert this to .prn format
    $result=system("python $TRANSLATESCRIPT $CIRFILE.raw");
    if ($result != 0)
    {
        print STDERR "translator crashed trying to convert $ASCII rawfile to prn\n";
        print "Exit code = 14 \n";
        exit 14;
    }
    $PRNFILE="test_".$CIRFILE.".raw";
    if (! -e $PRNFILE)
    {
        print STDERR "Conversion of $ASCII rawfile did not produce prn file\n";
        print "Exit code = 14\n";
        exit 14;
    }

    # Now read in the prn file and check that columns 3 and 4 agree
    open(PRN,"<$PRNFILE") || die "Could not open prnfile\n";
    while (<PRN>)
    {
        s/^\s*//;
        s/\s+/ /g;
        @cols=split(/\s/);
        next if ($cols[0] eq "Index");
        if ($cols[2] != $cols[3])
        {
            print STDERR "Columns 3 and 4 of $ASCII file disagree\n";
            print STDERR;
            print STDERR "Column 3: $cols[2], Column 4: $cols[3]\n";
            print "Exit code = 2\n";
            exit 2;
        }
    }
    print STDERR "Everything in $ASCII file agrees!\n";
    close(PRN);
}

print "Exit code = 0\n";
exit 0;
