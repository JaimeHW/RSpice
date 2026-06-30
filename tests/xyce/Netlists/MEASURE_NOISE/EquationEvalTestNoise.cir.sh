#!/usr/bin/env perl

use MeasureCommon;
use File::Copy;

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

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;

# remove files from previous runs
system("rm -f $CIRFILE.ma0 $CIRFILE.out $CIRFILE.err*");

#
# Steps common to all of the measure tests are in the Perl module
# MeasureCommon.pm.  This file assumes the analysis type was .NOISE
#
MeasureCommon::checkNoiseFilesExist($XYCE,$CIRFILE);

# If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh", instead of the rest of this .sh file, and then exit
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
  print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
  if (system("$XYCE_VERIFY $CIRFILE junk $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
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

# The next three blocks of code are used to compare the .MEASURE output
# to stdout to the "gold" stdout in the $GSFILE (EquationEvalTestNoiseGSfile).

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

# parse the .out file to find the text related to .MEASURE
my $foundStart=0;
my $foundEnd=0;
my @outLine;
my $lineCount=0;
while( $line=<NETLIST> )
{
  if ($line =~ /Measure Functions/) { $foundStart = 1; }
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
my $GSFILE="EquationEvalTestNoiseGSfile";
my $absTol=1e-5;
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

# The next two blocks of code are used to compare the measured .ma0 file
# with the "Gold" .ma0 file, which is in OutputData/MEASURE_NOISE/
# Check that the Gold .ma0 file exists
$GOLDMA0 = "$GOLDPRN.ma0";
#print "GOLDMA0 file = $GOLDMA0\n";
if (not -s "$GOLDMA0" )
{
  print "GOLD .ma0 file does not exist\n";
  print "Exit code = 17\n";
  exit 17;
}

# compare gold and measured .ma0 files
$MEASUREMA0 = "$CIRFILE.ma0";
$CMD="$fc $MEASUREMA0 $GOLDMA0 $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
$retval=system($CMD);
$retval= $retval >> 8;

if ( $retval != 0 )
{
  print STDERR "test failed comparison of Gold and measured .ma0 files with exit code $retval\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Passed comparison of .ma0 files\n";
}

# also compare output .NOISE.prn file. Comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-3;  #0.1%
$zerotol=1e-8;
$freqreltol=1e-6;

# compare gold and test files
$retcode = 0;
$CMD="$XYCE_ACVERIFY $GOLDPRN.NOISE.prn $CIRFILE.NOISE.prn $abstol $reltol $zerotol $freqreltol";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited on file $CIRFILE.NOISE.prn with exit code $retval\n";
  $retcode = 2;
}

print "Exit code = $retval\n";
exit $retval;
