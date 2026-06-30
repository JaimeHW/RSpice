#!/usr/bin/env perl

use lib '.';
use XyceRegression::Tools;
use MeasureCommon;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

use File::Basename;

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
$GOLDPRN=$ARGV[4];

# check various error cases
# this string should be in the warning messages of this Xyce run  
@searchstrings = ( "Netlist warning in file bad_dot_measure_line6.cir at or near line 20",
                   ".MEASURE lines do not support DC modes.  Measure line will be ignored."
                 );

print "Checking warning messages for DC measure line\n";
$retval = $Tools->runAndCheckWarning($CIRFILE,$XYCE,@searchstrings);

if ($retval != 0)
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

#
# steps common to all of the measure tests are in the module
# commonCode.pl   This file assume the analysis type was .tran
#
print "Checking stdout and .mt0 file for valid TRAN measure\n";
MeasureCommon::checkTranFilesExist($XYCE,$CIRFILE);

# Now check that the TRAN measure is correct
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
my $GSFILE_STDOUT="bad_dot_measure_line6_GSFile";
my $GSFILE_MT0="bad_dot_measure_line6_Mt0GSFile";
my $absTol=1e-5;
my $relTol=1e-3;
my $zeroTol=1e-10;

# check the info in stdout
my $dirname = dirname($XYCE_VERIFY);
my $fc = "$dirname/file_compare.pl";
`$fc $CIRFILE.errmsg $GSFILE_STDOUT $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err`;
my $retval=$? >> 8;

if ( $retval != 0 )
{
  print "test Failed comparison vs. GSfile!\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# check the info in the .mt0 file
`$fc $CIRFILE.mt0 $GSFILE_MT0 $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err`;
my $retval=$? >> 8;

if ( $retval != 0 )
{
  print "test Failed comparison vs. GSfile!\n";
  print "Exit code = $retval\n";
  exit $retval;
}

# do final checks on return value 
if ( $retval != 0 )
{
  print "test Failed!\n";
  print "Exit code = $retval\n";
  exit $retval;
}
else
{
  print "test passed!\n";
  print "Exit code = $retval\n";
  exit $retval;
}


