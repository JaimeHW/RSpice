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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;

$GSFILE="ErrorFuncTestGSfile";

# gold mt0 file will be OutputData/MEASURE/REMEASURE/
$GOLDMT0 = $GOLDPRN;
$GOLDMT0 =~ s/prn$/mt0/;
if (not -s "$GOLDMT0" )
{
  print "GOLD .mt0 file does not exist\n";
  print "Exit code = 17\n";
  exit 17;
}

# remove files from previous runs
system("rm -f $CIRFILE.mt0 $CIRFILE.out $CIRFILE.err*");

# default return code
$retval = 0;

print "Testing Re-measure for netlist $CIRFILE\n";

# here is the command to run xyce with remeasure.
$CMD="$XYCE -remeasure $CIRFILE.remeasure.prn $CIRFILE > $CIRFILE.out";
print ("$CMD\n");
if (system($CMD) != 0)
{
  print "Re-measure failed for netlist $CIRFILE\n";
  print "Exit code = 2\n"; exit 2;
}

# Did we make the measure file
if (not -s "$CIRFILE.mt0" ) { print "Exit code = 17\n"; exit 17; }

# compare the mt0 files
$CMD = "$fc $CIRFILE.mt0 $GOLDMT0 $absTol $relTol $zeroTol > $CIRFILE.mt0.out 2> $CIRFILE.mt0.err";
if (system($CMD) != 0)
{
  print "Re-measure failed for netlist $CIRFILE\n";
  print "Exit code = 2\n"; exit 2;
}

# now check the stdout
# check that .out file exists for re-measure, and open it if it does
if (not -s "$CIRFILE.out" )
{
  print "Re-measure test failed to find stdout for netlist $CIRFILE\n";
  print "Exit code = 17\n"; exit 17;
}
else
{
  open(NETLIST, "$CIRFILE.out");
  open(ERRMSG,">$CIRFILE.errmsg") or die $!;
}

# parse the .out file to find the text related to remeasure
$foundStart=0;
$foundEnd=0;
$lineCount=0;
while( $line=<NETLIST> )
{
  if ($line =~ /In OutputMgr::remeasure/) { $foundStart = 1; }
  if ($foundStart > 0 && $line =~ /Remeasure analysis complete/) { $foundEnd = 1; }

  if ($foundStart > 0 && $foundEnd < 1)
  {
    print ERRMSG $line;
  }
}

close(NETLIST);
close(ERRMSG);

# test that the values and strings in the .out file match the Gold File
# to the required tolerances
$CMD="$fc $CIRFILE.errmsg $GSFILE $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
if (system($CMD) != 0)
{
  print "Failed test of Re-measure stdout for netlist $CIRFILE\n";
  print "Exit code = 2\n"; exit 2;
}

# success if we reached here
print "Exit code = $retval\n";
exit $retval;
