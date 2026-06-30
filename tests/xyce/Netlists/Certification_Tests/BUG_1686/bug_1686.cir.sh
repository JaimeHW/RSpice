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

@cirlist = ("bug_1686a.cir", "bug_1686_dup.cir");

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

#remove the "0" circuit from the cirlist
$baseline=shift @cirlist;

#so this loop is over everything *but* the 0 element
foreach $cirname (@cirlist)
{
  $CMD = "diff $baseline.prn $cirname.prn > $cirname.prn.out 2> $cirname.prn.err";
  if ( system("$CMD") != 0 ) 
  { 
    $CMD = "$XYCE_VERIFY $baseline.cir $baseline.cir.prn $cirname.prn >> $cirname.prn.out 2>> $cirname.prn.err";
    `echo "diff failed, now trying xyce_verify" >> $cirname.prn.err`;
    `echo "$CMD" >> $cirname.prn.err`;
    if ( system("$CMD") != 0 )
    {
      `echo "Test Failed!" >> $cirname.prn.out`;
      $testfailed = 1;
    }
  }
  else
  {
    if ( -z "$cirname.prn.out" ) { `rm -f $cirname.prn.out`; }
    if ( -z "$cirname.prn.err" ) { `rm -f $cirname.prn.err`; }
  }
}
if (defined($testfailed)) 
{ 
  `echo "Test Failed!" >> $CIRFILE.prn.out`;
  print "Exit code = 2\n"; exit 2; 
}

`echo "Test Passed!" >> $CIRFILE.prn.out`;
print "Exit code = 0\n"; exit 0;



