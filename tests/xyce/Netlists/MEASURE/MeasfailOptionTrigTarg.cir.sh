#!/usr/bin/env perl

use MeasureCommon;
use File::Basename;

use XyceRegression::Tools;
$tools = XyceRegression::Tools->new();

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

$xpd=$XYCE_VERIFY;
$xpd=~s/xyce_verify/xplat_diff/;

# these files have various .OPTIONS MEASURE lines in them.
@CIR;
$CIR[0]="MeasfailOptionZeroTrigTarg.cir";
$CIR[1]="MeasfailOptionOneTrigTarg.cir";

# use one GSfile for all of the netlists
$GSFILE = "MeasfailOptionTrigTargGSfile";

# default return code
$retval = 0;

# We do need different mt0 files though in OutputData/MEASURE/ for the
# Zero and One cases.
$dirname = dirname($GOLDPRN);
@GOLDMT0;
foreach $idx (0 ..1)
{
  $GOLDMT0[$idx] = "$dirname/$CIR[$idx].mt0";
  #if (not -s "$GOLDMT0[$idx]" ) 
  #{ 
  #  print "$GOLDMT0[$idx] file does not exist\n";
  #  $retval = 17; 
  #}
}

if ($retval != 0)
{
  print "Exit code = $retval\n";
  exit $retval;
}

# remove files from previous runs
system("rm -f $CIRFILE.mt0 $CIRFILE.out $CIRFILE.err*");
foreach $idx (0 .. 1)
{  
  system("rm -f $CIR[$idx].mt0 $CIR[$idx].out $CIR[$idx].err*");
}

foreach $idx (0 .. 1)
{
  print "Testing file $CIR[$idx]\n";

  $retval=$tools->wrapXyce($XYCE,$CIR[$idx]);
  if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
  if (not -s "$CIR[$idx].prn" ) { print "Exit code = 14\n"; exit 14; }

  # Did we make a measure file 
  if (not -s "$CIR[$idx].mt0") 
  {
    print "Failed to make .mt0 file for netlist $CIR[$idx]\n"; 
    print "Exit code = 17\n"; exit 17; 
  }

  # check that .out file exists, and open it if it does
  if (not -s "$CIR[$idx].out" ) 
  { 
    print "Exit code = 17\n"; 
    exit 17; 
  }
  else
  {
    open(NETLIST, "$CIR[$idx].out");
    open(ERRMSG,">$CIR[$idx].errmsg") or die $!;
  }

  # parse the .out file to find the text related to .MEASURE
  $foundStart=0;
  $foundEnd=0;
  @outLine;
  $lineCount=0;
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
  # tolerances.  Using common GSFile for all of the netlists.
  $CMD="$fc $CIR[$idx].errmsg $GSFILE $absTol $relTol $zeroTol > $CIR[$idx].errmsg.out 2> $CIR[$idx].errmsg.err";
  if ( system($CMD) != 0 )
  {
    print STDERR "test failed comparison of stdout vs. GSfile for netlist $CIR[$idx]\n";
    print "Exit code = 2\n";
    exit 2;
  }
  else
  {
    print "Passed comparison of stdout info for netlist $CIR[$idx]\n";
  }

  # compare gold and measured .mt0 files
  $MEASUREMT0 = "$CIR[$idx].mt0";
  if ($idx == 0)
  {
    # Use file_compare since the .mt0 file for the Zero case has numbers (0) in it.
    $CMD="$fc $MEASUREMT0 $GOLDMT0[$idx] $absTol $relTol $zeroTol > $CIR[$idx].errmsg.out 2> $CIR[$idx].errmsg.err";
    if ( system($CMD) != 0 )
    {
      print STDERR "test failed comparison of Gold and measured .mt0 files for netlist $CIR[$idx]\n";
      print "Exit code = 2\n";
      exit 2;
    }
  }
  else
  {
    # .mt0 files for the One and Bogo cases should exactly match the gold file for the
    # One case, since they both contain only text.  Also the Bogo case defaults to 
    # the One case.
    $CMD="$xpd $MEASUREMT0 $GOLDMT0[1]";
    if ( system($CMD) != 0 )
    {
      print STDERR "test failed comparison of Gold and measured .mt0 files for netlist $CIR[$idx]\n";
      print "Exit code = 2\n";
      exit 2;
    }
  }
  print "Passed comparison of .mt0 files for netlist $CIR[$idx]\n";
}

# success if we reached here
print "Exit code = $retval\n";
exit $retval;
