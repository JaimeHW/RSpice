#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10
# Otherwise we return the exit code of compare or xyce_verify.pl

# This tests that two specific circuits produce xyce_verify-equivalent
# output.  Prior to the fix of bug 1040, they did not.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

@cirlist = ("rc_discharge_diode_op.net","rc_discharge_diode_noop.net");

$xyceexit = 0;

foreach $cirname (@cirlist)
{
  `rm -f $cirname.prn > /dev/null 2>&1`;
  `rm -f $cirname.err > /dev/null 2>&1`;
  $CMD = "$XYCE $cirname > $cirname.out 2> $cirname.err";
  if ( system("$CMD") != 0 )
  {
    `echo "Xyce EXITED WITH ERROR! on $cirname" >> $cirname.err`;
    print STDERR "Xyce EXITED WITH ERROR! on $cirname";
    # Append the output to the file that actually gets uploaded
    # to CDash on failure, so we might have some hope of knowing what
    # failed and why
    `cat $cirname.out >> $CIRFILE.out`;
    `cat $cirname.err >> $CIRFILE.err`;
    $xyceexit = 1;
  }
  else
  {
    if ( -z "$cirname.err" ) { `rm -f $cirname.err`; }
  }
}

# Exit script with appropriate error code if either of the previous runs
# failed
if ($xyceexit == 1)
{
    print "Exit code = 10\n";
    exit 10;
}

# we have now run both circuits.  They should produce curves that pass xyce_verify
$CMD = "$XYCE_VERIFY rc_discharge_diode_op.net rc_discharge_diode_op.net.prn rc_discharge_diode_noop.net.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
if ( system("$CMD") != 0 )
{
    print "Exit code = 2\n"; exit 2;
}
else
{
    if (-z "$CIRFILE.prn.out") { `rm -f $CIRFILE.prn.out`;}
    if (-z "$CIRFILE.prn.err") { `rm -f $CIRFILE.prn.err`;}
    print "Exit code = 0\n"; exit 0;
}
