#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10
# Otherwise we return the exit code of compare or xyce_verify.pl

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$LET=$CIRFILE;
$LET =~ s/subckt_([a-z])\.cir/$1/;

$cirfiles = "linear_simple_param.cir linear_simple_global.cir";
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
if (defined($xyceexit)) { print "Exit code = 10\n"; exit 10; }

$PARAMCIR=$cirlist[0];
$GLBLCIR=$cirlist[1];

$CMD = "diff $PARAMCIR.prn $GLBLCIR.prn > $GLBLCIR.prn.out 2> $GLBLCIR.prn.err";
if ( system("$CMD") != 0 ) 
{ 
    $CMD = "$XYCE_VERIFY $PARAMCIR $PARAMCIR.prn $GLBLCIR.prn >> $GLBLCIR.prn.out 2>> $GLBLCIR.prn.err";
    `echo "diff failed, now trying xyce_verify" >> $GLBLCIR.prn.err`;
    `echo "$CMD" >> $GLBLCIR.prn.err`;
    if ( system("$CMD") != 0 )
    {
        `echo "Test Failed!" >> $GLBLCIR.prn.out`;
        $testfailed = 1;
    }
}
else
{
    if ( -z "$GLBLCIR.prn.out" ) { `rm -f $GLBLCIR.prn.out`; }
    if ( -z "$GLBLCIR.prn.err" ) { `rm -f $GLBLCIR.prn.err`; }
}

if (defined($testfailed)) 
{ 
    `echo "Test Failed!" >> subckt_$LET.cir.prn.out`;
    print "Exit code = 2\n"; exit 2; 
}

if ( -z "$GLBLCIR.prn.err" ) { `rm -f $GLBLCIR.prn.err`; }
`echo "Test Passed!" >> $GLBLCIR.prn.out`;
print "Exit code = 0\n"; exit 0;



