#!/usr/bin/env perl

use XyceRegression::Tools;
use MeasureCommon;

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

use File::Copy;

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$fc=$XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;

# remove previous output files
system("rm -f $CIRFILE.csd $CIRFILE.out $CIRFILE.err $CIRFILE.mt* $CIRFILE.remeasure.*");

# run Xyce and check for the proper files
$CMD="$XYCE $CIRFILE > $CIRFILE.out";
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

# Did we make the output file
if (not -s "$CIRFILE.prn" ) {print "Exit code = 14\n"; exit 14; }

# Did we make a Measure file
if (not -s "$CIRFILE.mt0" ) { print "Exit code = 17\n"; exit 17; }

# move the .mt0 file from the original Measure run to a temp file.  
# It will be moved back later.
move("$CIRFILE.mt0","$CIRFILE.temp.mt0");

# now do a re-measure
$CMD="$XYCE -remeasure $CIRFILE.prn $CIRFILE > $CIRFILE.remeasure.out";
$retval=system($CMD)>>8;
  
# Did we make the measure file for re-measure
if (not -s "$CIRFILE.mt0" ) { print "Exit code = 17\n"; exit 17; }

# rename the .mt0 files, so it's obvious which one comes from the
# original Measure run, and which one comes from the re-measure run. 
move("$CIRFILE.mt0","$CIRFILE.remeasure.mt0");
move("$CIRFILE.temp.mt0","$CIRFILE.mt0");

# check that .out file exists for re-measure, and open it if it does
if (not -s "$CIRFILE.remeasure.out" ) 
{ 
  print "Exit code = 17\n"; 
  exit 17; 
}
else
{
  open(NETLIST, "$CIRFILE.remeasure.out");
  open(ERRMSG,">$CIRFILE.remeasure.errmsg") or die $!;
}

# parse the remeasure.out file to find the text related to remeasure
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

# test that the values and strings in the .out file match the Gold File 
# to the required tolerances
$GSFILE="RemeasureStdOutTestGSfile";
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;

# use file_compare 
$CMD="$fc $CIRFILE.remeasure.errmsg $GSFILE $absTol $relTol $zeroTol > $CIRFILE.remeasure.errmsg.out 2> $CIRFILE.remeasure.errmsg.err";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval == 0) 
{ 
  $retcode = 0; 
}
else 
{ 
  print STDERR "Comparator exited with exit code $retval during remeasure test\n";
  $retcode = 2; 
}  

print "Exit code = $retcode\n";
exit $retcode;

