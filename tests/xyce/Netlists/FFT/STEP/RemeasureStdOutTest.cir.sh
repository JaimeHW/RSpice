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

# name of the file with the data to remeasure
$RMF= $CIRFILE;
$RMF =~ s/cir$/csv/;

# remove files from previous runs
system("rm -f $CIRFILE.mt0 $CIRFILE.fft0 $CIRFILE.out $CIRFILE.err*");

# run Xyce with -remeasure and check for the proper outputfiles
$CMD="$XYCE -remeasure $RMF $CIRFILE > $CIRFILE.out";
$retval = system($CMD);

if ($retval !=0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n";
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIRFILE;
    exit 13;
  }
  else
  {
    print "Exit code = 10\n";
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE;
    exit 10;
  }
}

if (not -s "$CIRFILE.fft0")
{
  print "$CIRFILE.fft0 file not made during -remeasure operation\n";
  print "Exit code = 2\n"; exit 2;
}

if (-s "$CIRFILE.mt0")
{
  print "$CIRFILE.mt0 file made during -remeasure operation, when it should not\n";
  print "Exit code = 2\n"; exit 2;
}

# check that .out file exists, and open it if it does
if (not -s "$CIRFILE.out" )
{
  print "Exit code = 17\n";
  exit 17;
}
else
{
  open(NETLIST, "$CIRFILE.out");
  open(ERRMSG,">$CIRFILE.errmsg") or die $!;
}

# parse the .out file to find the text related to remeasure
my $foundStart=0;
my $foundEnd=0;
my @outLine;
my $lineCount=0;
while( $line=<NETLIST> )
{
  if ($line =~ /In OutputMgr::remeasure/) { $foundStart = 1; }

  if ($foundStart > 0 && $foundEnd < 1)
  {
    print ERRMSG $line;
  }

  # include the "complete" line in the test
  if ($foundStart > 0 && $line =~ /Remeasure analysis complete/) { $foundEnd = 1; }
}
close(NETLIST);
close(ERRMSG);

# test that the values and strings in the .out file match to the required tolerances
my $GSFILE="RemeasureStdOutTestGSfile";
my $absTol=1e-5;
my $relTol=1e-3;
my $zeroTol=1e-10;

$CMD="$fc $CIRFILE.errmsg $GSFILE $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
$retval=system($CMD);
$retval = $retval >> 8;

if ( $retval != 0 )
{
  print STDERR "test failed comparison of stdout vs. GSfile with exit code $retval\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Passed comparison of stdout info\n";
}

print "Exit code = $retval\n";
exit $retval;
