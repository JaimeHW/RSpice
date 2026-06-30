#!/usr/bin/env perl
# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];
$PRNOUT=$CIRFILE.".prn";

# remove old output files
system("rm -f $CIRFILE\_faked*");

# This is the list of fields that must be in the output.
# Use of unordered maps in Xyce means they might not come out in the
# same order on different platforms.
@expectedOutputs=("Index", "V\\(1\\)", "V\\(X2:4B\\)", "V\\(X1:4A\\)", "V\\(X1:4B\\)",
                  "V\\(X2:4A\\)", "V\\(X1:X3:E\\)", "V\\(X2:X3:E\\)");

# Now run the main netlist, which has the V() wildcard print line in it.
$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$PRNOUT" ) { print "Exit code = 14\n"; exit 14; }

# pull the header line out of the file and check it for the presence of all
# required data:

open(PRNFILE,"<$PRNOUT");
$headerline=<PRNFILE>;
close(PRNFILE);

chomp($headerline);
@headerfields=split(' ',$headerline);

$retval=0;
$numMatch=0;
foreach $field (@expectedOutputs)
{
    if ( $headerline =~ /$field/ )
    {
        ++$numMatch;
    }
    else
    {
        print STDERR "Could not find field $field in primary output file.\n";
        $retval=2;
    }
}

if ($#headerfields+1 != $#expectedOutputs+1)
{
    print STDERR "Incorrect number of entries on header line in primary output file.\n";
    $retval=2;
}
elsif ($numMatch != ($#expectedOutputs + 1))
{
    print STDERR "Insufficient number of matches found on header line in primary output file.\n";
    $retval=2;
}

# only if we have all the expected outputs should we proceed.
if ($retval==0)
{
    @headerfields=split(' ',$headerline);
    # Get rid of Index
    shift(@headerfields);

    open(CIRFILE,"<$CIRFILE");
    $CIRFILE2=$CIRFILE."_faked";
    open(CIRFILE2,">$CIRFILE2");
    while(<CIRFILE>)
    {
        if (/.print/i)
        {
            print CIRFILE2 ".print dc";
            foreach $field (@headerfields)
            {
                print CIRFILE2 " $field";
            }
            print CIRFILE2 "\n";
        }
        else
        {
            print CIRFILE2 $_;
        }
    }
    close(CIRFILE);
    close(CIRFILE2);

    # we have now created a new circuit file that should have a .print line that matches
    # what the V() wildcard version did
    $retval=$Tools->wrapXyce($XYCE,$CIRFILE2);
    if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
    if (not -s "$CIRFILE2.prn" ) { print "Exit code = 14\n"; exit 14; }

    # Have to use the faked cirfile here so that xyce_verify gets the right header expectations
    $CMD="$XYCE_VERIFY $CIRFILE2 $PRNOUT $CIRFILE2.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
    $retcode=system($CMD);
    $retval=2 if $retcode != 0;
}

print "Exit code = $retval\n";
exit $retval;
