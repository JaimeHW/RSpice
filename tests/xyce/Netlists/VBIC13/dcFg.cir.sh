#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script (NEVER USED)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10
# Otherwise we return the exit code of compare or xyce_verify.pl

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$BASE_FILENAME=$CIRFILE;
$BASE_FILENAME =~ s/.cir//;

$cirfiles = "$BASE_FILENAME.cir $BASE_FILENAME"."_noFlip_P.cir $BASE_FILENAME"."_m.cir";
@cirlist = sort split(" ",$cirfiles);

foreach $cirname (@cirlist)
{
  `rm -f $cirname.prn > /dev/null 2>&1`;
  `rm -f $cirname.err > /dev/null 2>&1`;
  $CMD = "$XYCE $cirname > $cirname.out 2> $cirname.err";
  if ( system("$CMD") != 0 ) 
  { 
    `echo "Xyce EXITED WITH ERROR! on $cirname" >> $cirname.err`;
    $xyceexit = 1;
  }
  else
  {
    if ( -z "$cirname.err" ) { `rm -f $cirname.err`; }
  }
}
if (defined($xyceexit)) 
{ 
    print "Exit code = 10\n";   # Xyce exited with error
    exit 10; 
}

# We have now run two netlists that should produce identical output.  Check
# the first against the gold standard
$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE".".prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
$result= system("$CMD");

if ($result != 0)
{
    print "Exit code = 2\n";
    print STDERR "Base netlist $CIRFILE failed to match gold standard.\n";
    exit 2;
}

# Now try diff of the noFlip_P and m circuits against the baseline:
# Remove baseline from list
shift @cirlist;

foreach $cirname (@cirlist)
{
    $CMD="diff $CIRFILE.prn $cirname.prn > $cirname.prn.out 2> $cirname.prn.err";
    if (system("$CMD") != 0)
    {
        `echo "diff failed, now trying xyce_verify" >> $cirname.prn.err`;
        $CMD="$XYCE_VERIFY $cirname $CIRFILE.prn $cirname.prn >> $cirname.prn.out 2>> $cirname.prn.err";
        `echo "$CMD" >> $cirname.prn.err`;
        if (system("$CMD") != 0)
        {
            `echo "Test failed!" >> $cirname.prn.out`;
            $testfailed = 1;
        }
    }
    else
    {
        if ( -z "$cirname.prn.out") { `rm -f $cirname.prn.out`;}
        if ( -z "$cirname.prn.err") { `rm -f $cirname.prn.err`;}
    }
}

if (defined($testfailed))
{
    `echo "Test Failed!" >> $CIRFILE.prn.out`;
    print "Exit code = 2\n";
    exit 2;
}

if ( -z "$CIRNAME.prn.err") { `rm -f $CIRNAME.prn.err`;}
`echo "Test passed!" >> $CIRFILE.prn.out`;
print "Exit code = 0\n";
exit 0;
