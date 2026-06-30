#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce the appropriate output files, then we return exit code 14.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.csv.out and the STDERR output from
# comparison to go into $CIRFILE.csv.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;

$GOLDFFT = $GOLDPRN;
$GOLDFFT =~ s/\.prn$//;

system("rm -f $CIRFILE.fft*");

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) {print "Exit code = $retval\n"; exit $retval;}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
    {
        print "Exit code = 2 \n";
        exit 2;
    }
    else
    {
        print "Exit code = 0 \n";
        exit 0;
    }
}

if ( !(-f "$CIRFILE.fft0")) {
    print STDERR "Missing output file $CIRFILE.fft0\n";
    print "Exit code = 2\n";
    exit 2;
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
  if ($line =~ /FFT Analyses/) { $foundStart = 1; }

  if ($foundStart > 0 && $line =~ /Total Simulation/) { $foundEnd = 1; }

  if ($foundStart > 0 && $foundEnd < 1)
  {
    print ERRMSG $line;
  }
}
close(NETLIST);
close(ERRMSG);

# test that the values and strings in the .out file match to the required
# tolerances
my $GSFILE="fft_accurateOptionZeroGSfile";
my $absTol=1e-3;
my $relTol=1e-3;
my $zeroTol=1e-10;

$CMD="$fc $CIRFILE.errmsg $GSFILE $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
$retval=system($CMD);
$retval= $retval >> 8;

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

# check .fft0 file
$retcode=0;
$CMD="$fc $CIRFILE.fft0 $GOLDFFT.fft0 $absTol $relTol $zeroTol > $CIRFILE.fft0.out 2> $CIRFILE.fft0.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $CIRFILE.fft0\n";
  $retcode = 2;
}

print "Exit code = $retcode\n";
exit $retcode;
