#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script (NEVER USED)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This script is designed to test BUG_870_SON.
# The bug was that BSIM CMG (and possibly other ADMS-generated devices
# had improperly placed code that set a "type" variable based on the type
# of the model card (NPN, PNP, NMOS, PMOS).
#
# If the Verilog-A used a parameter default for the type variable that depended 
# on some other parameter (really done only in BSIM-CMG, where TYPE's default was DEVTYPE)
# then processParams would reset the value of TYPE determined by PMOS model card type
# back to NMOS, UNLESS one had also specified TYPE in the model card explicitly.
#
# The BSIM group's sample model card DID set TYPE explicitly, and so this bug was hidden.
#
# This test runs two netlists that differ only by the presence or absence of
# an explicit TYPE=0 in the model card.
#
# They should produce identical output.  Prior to the fix of bug 870, they would not.
#

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

$BASE_FILENAME=$CIRFILE;
$BASE_FILENAME =~ s/.cir//;

$cirfiles = "$BASE_FILENAME.cir $BASE_FILENAME"."_notypeval.cir";
@cirlist = split(" ",$cirfiles);

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

# diff of the "notypeval circuit against the baseline:
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
