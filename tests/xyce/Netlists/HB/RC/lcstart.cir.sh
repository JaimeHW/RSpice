#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

#$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
#if ($retval != 14) { print "Exit code = $retval\n"; exit $retval; }
system("rm -f $CIRFILE.HB.TD.prn $CIRFILE.hb_ic.prn");
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";

if (system($CMD) != 0)
    {
        `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
        $xyceexit=1;
    }
else
    {
        if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
    }

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

if ((-f "$CIRFILE.HB.TD.prn") && (-f "$CIRFILE.HB.FD.prn"))
{
  $CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN.HB.TD.prn $CIRFILE.HB.TD.prn > $CIRFILE.HB.TD.prn.out 2> $CIRFILE.HB.TD.prn.err";
  $retval = system("$CMD");

  if ($retval == 0) 
  {
    $retcode = 0;
  }

  else { $retcode = 2; }
} 
else
{
  $retcode = 14;
}

print "Exit code = $retcode\n"; exit $retcode;
