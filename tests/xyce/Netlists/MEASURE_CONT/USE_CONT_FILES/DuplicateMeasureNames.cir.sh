#!/usr/bin/env perl

use XyceRegression::Tools;
use MeasureCommon;
use File::Copy;


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

# remove files from previous runs
system("rm -f $CIRFILE.mt0 $CIRFILE.out $CIRFILE.err* $CIRFILE.remeasure*");
system("rm -f $CIRFILE.prn.errmsg.* $CIRFILE\_*.mt0");

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

# We should not make the measure file in this case
if (-s "$CIRFILE.mt0" )
{
  print "$CIRFILE.mt0 file made when it should not\n";
  print "Exit code = 2\n";
  exit 2;
}

#
# Steps common to all of the measure tests are in the Perl module
# MeasureCommon.pm.

# measure names must be in lower case
@contMeasureNames;
$contMeasureNames[0] = "m1";
$contMeasureNames[1] = "m2";

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

# check the warning messages
@searchstrings = ("Netlist warning: Measure \"M1\" redefined, ignoring any previous definitions",
                  "Netlist warning: Measure \"M2\" redefined, ignoring any previous definitions"
);
if( $Tools->checkError("$CIRFILE.out",@searchstrings) != 0)
{
  print "Failed to find correct warning messages\n";
  print "Exit code = 2\n"; exit 2;
}


# The next three blocks of code are used to compare the .MEASURE output
# to stdout to the "gold" stdout in the $GSFILE (DuplicateMeasureNamesGSfile).

# check that .out file exists, and open it if it does
if (! -s "$CIRFILE.out" ) {
  print "Exit code = 7\n";
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
my $GSFILE="DuplicateMeasureNamesGSfile";
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

$exitcode=0;
# check all of the continuous mode measure output files.
foreach $i (0 .. 1)
{
  $CONTMT0 = "$CIRFILE\_$contMeasureNames[$i].mt0";
  $GOLDCONTMT0 = $GOLDPRN;
  $GOLDCONTMT0 =~ s/.prn$//;
  $GOLDCONTMT0 = "$GOLDCONTMT0\_$contMeasureNames[$i].mt0";

  if (not -s "$CONTMT0" )
  {
    print "$CONTMT0 file does not exist\n";
    print "Exit code = 17\n";
    exit 17;
  }

  if (not -s "$GOLDCONTMT0" )
  {
    print "Gold $CONTMT0 file does not exist\n";
    print "Exit code = 17\n";
    exit 17;
  }

  $CMD="$fc $CONTMT0 $GOLDCONTMT0 $absTol $relTol $zeroTol >> $CIRFILE.errmsg.out 2>> $CIRFILE.errmsg.err";
  $retval=system($CMD);
  $retval= $retval >> 8;

  if ( $retval != 0 )
  {
    print STDERR "test failed comparison of Gold and measured .mt0 files for $CONTMT0 with exit code $retval\n";
    $exitcode=2;
  }
  else
  {
    print "Passed comparison of $CONTMT0 file\n";
  }
}

if ($exitcode !=0)
{
  print "Exit code = $exitcode\n";
  exit $exitcode
}

print "Exit code = $retval\n";
exit $retval;
