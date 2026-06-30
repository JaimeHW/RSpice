#!/usr/bin/env perl

use MeasureCommon;

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

# these files have various .OPTIONS MEASURE lines in them.
@CIR;
$CIR[0]="MeasprintOptionALL.cir";
$CIR[1]="MeasprintOptionBOGO.cir";
$CIR[2]="MeasprintOptionSTDOUT.cir";
$CIR[3]="MeasprintOptionNONE.cir";

# use one GSfile and gold .mt0 file for all of the netlists
$GSFILE = "MeasprintOptionGSfile";
$REMGSFILE="MeasprintOptionRemeasureGSfile";

# gold mt0 file will be OutputData/MEASURE/MeasprintOption.cir.mt0
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
foreach $idx (0 .. 3)
{  
  system("rm -f $CIR[$idx].mt0 $CIR[$idx].out $CIR[$idx].err* $CIR[$idx].remeasure*");
}

# default return code
$retval = 0;

foreach $idx (0 .. 3)
{
  print "Testing file $CIR[$idx]\n";

  $retval=$tools->wrapXyce($XYCE,$CIR[$idx]);
  if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
  if (not -s "$CIR[$idx].prn" ) { print "Exit code = 14\n"; exit 14; }

  # Did we make a measure file for the ALL and BOGO netlists?
  # Did we not make a .mt0 file for the STDOUT and NONE netlists?
  if ($idx < 2)
  {
    if (not -s "$CIR[$idx].mt0") 
    {
      print "Failed to make .mt0 file for netlist $CIR[$idx]\n"; 
      print "Exit code = 17\n"; exit 17; 
    }
  }
  elsif (-s "$CIR[$idx].mt0") 
  {
      print "Made .mt0 file for netlist $CIR[$idx], when we should not\n"; 
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

  # stdout should have measure info for the ALL, BOGO and STDOUT netlists
  if ($idx < 3)
  {
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
  }
  else
  {
    $CMD="grep MAXV1 $CIR[$idx].errmsg";
    if ( system($CMD) == 0 )
    {
      print STDERR "Measure output found in stdout for netlist $CIR[$idx]\n";
      print "Exit code = 2\n";
      exit 2;
    }
  }

  # mt0 files should only have been made for ALL and BOGO netlists.
  # This was checked earlier.
  if ($idx < 2)
  {
    # compare gold and measured .mt0 files
    $MEASUREMT0 = "$CIR[$idx].mt0";
    $CMD="$fc $MEASUREMT0 $GOLDMT0 $absTol $relTol $zeroTol > $CIR[$idx].errmsg.out 2> $CIR[$idx].errmsg.err";
    if ( system($CMD) != 0 )
    {
      print STDERR "test failed comparison of Gold and measured .mt0 files for netlist $CIR[$idx]\n";
      print "Exit code = 2\n";
      exit 2;
    }
    else
    {
      print "Passed comparison of .mt0 files for netlist $CIR[$idx]\n";
    }
  }

  # check the warning message for the BOGO case
  @searchstrings = ("Netlist warning: Unknown option value BOGO ignored for .OPTIONS MEASURE",
                   "MEASPRINT");
  if ($idx == 1) 
  { 
    if ( $tools->checkError("$CIR[$idx].out",@searchstrings) != 0) 
    {
      print "Failed to find correct warning message for $CIR[$idx]\n"; 
      print "Exit code = 2\n"; exit 2; 
    }
  }

  # Also test remeasure if the basic measure function works
  print "Testing Re-measure for netlist $CIR[$idx]\n";
 
  # rename the .mt0 files that were made for the ALL and BOGO netlists
  use File::Copy;
  if ($idx < 2) { move("$CIR[$idx].mt0","$CIR[$idx].temp.mt0"); }

  # here is the command to run xyce with remeasure.  Use the .prn file
  # from the ALL netlist so the comparison of stdout is easier.
  $CMD="$XYCE -remeasure $CIR[0].prn $CIR[$idx] > $CIR[$idx].remeasure.out";
  if (system($CMD) != 0) 
  {
    print "Re-measure failed for netlist $CIR[$idx]\n"; 
    print "Exit code = 2\n"; exit 2; 
  }

  # Did we make the measure file
  if (not -s "$CIR[$idx].mt0" ) { print "Exit code = 17\n"; exit 17; }

  # rename the mt0 files and compare them.  Use the .mt0 file from the ALL netlist in this
  # comparison as the "gold file"
  move("$CIR[$idx].mt0","$CIR[$idx].remeasure.mt0");
  if ($idx < 2) { move("$CIR[$idx].temp.mt0","$CIR[$idx].mt0"); }

  $CMD = "$fc $CIR[$idx].remeasure.mt0 $GOLDMT0 $absTol $relTol $zeroTol > $CIR[$idx].remeasure.mt0.out 2> $CIR[$idx].remeasure.mt0.err";  
  if (system($CMD) != 0)
  {
    print "Re-measure failed for netlist $CIR[$idx]\n"; 
    print "Exit code = 2\n"; exit 2;  
  }

  # now check the stdout for remeasure also
  # check that .out file exists for re-measure, and open it if it does
  if (not -s "$CIR[$idx].remeasure.out" ) 
  { 
    print "Re-measure test failed to find stdout for netlist $CIR[$idx]\n"; 
    print "Exit code = 17\n"; exit 17; 
  }
  else
  {
    open(NETLIST, "$CIR[$idx].remeasure.out");
    open(ERRMSG,">$CIR[$idx].remeasure.errmsg") or die $!;
  }

  # parse the remeasure.out file to find the text related to remeasure
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

  # test that the values and strings in the remeasure.out file match the Gold File 
  # to the required tolerances
  $CMD="$fc $CIR[$idx].remeasure.errmsg $REMGSFILE $absTol $relTol $zeroTol > $CIR[$idx].remeasure.errmsg.out 2> $CIR[$idx].remeasure.errmsg.err";
  if (system($CMD) != 0)
  { 
    print "Failed test of Re-measure stdout for netlist $CIR[$idx]\n"; 
    print "Exit code = 2\n"; exit 2;  
  } 
}

# success if we reached here
print "Exit code = $retval\n";
exit $retval;



