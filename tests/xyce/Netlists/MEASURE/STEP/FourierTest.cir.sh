#!/usr/bin/env perl

use MeasureCommon;

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
#$GOLDPRN=$ARGV[4];

# number of steps in the netlist CIRFILE
$numSteps=2;  

# Verify that measure works with .STEP.
$retval=MeasureCommon::checkTranStepResults($XYCE,$CIRFILE,$numSteps);

if ($retval != 0)
{
  print "Exit code = $retval\n";
  exit $retval;
}

# Verify stdout against a gold standard
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
$foundStart=0;
$foundEnd=0;
@outLine;
$lineCount=0;
while( $line=<NETLIST> )
{
  if ($line =~ /Measure Functions/) 
  { 
    $foundStart = 1;
    $foundEnd = 0; 
  }

  if ($foundStart > 0 && $line =~ /LASTMEASURE/ )
  {
    print ERRMSG $line;
    #print "Found last measure line\n";
    $foundEnd = 1; 
    $foundStart = 0;
  }
  elsif ( $foundStart > 0 && ( $line =~ /Solution Summary/ || $line =~ /Beginning DC/ ) )
  { 
    #print "Stopping on Solution Summary or Beginning DC line\n";
    $foundEnd = 1; 
    $foundStart = 0;
  }  
   
  if ($foundStart > 0 && $foundEnd < 1)
  {
    print ERRMSG $line;
  }
}

close(ERRMSG);

# test that the values and strings in the .out file match to the required tolerances
$GSFILE="FourierTestGSfile";

# phase is output in degrees and for fourier components with very small magnitude. It
# can have several degrees of scatter.  Thus it gets its own phaseabstol and phasereltol.
$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/compare_fourier_files/;
$absTol = 5.0e-2;
$relTol = 0.02;
$phaseAbsTol = 1.0e-3;
$phaseRelTol = 0.02;
$zeroTol = 1.0e-5;

$CMD="$fc $CIRFILE.errmsg $GSFILE $absTol $relTol $phaseAbsTol $phaseRelTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
$retval=system($CMD);
$retval= $retval >> 8;

if ( $retval != 0 )
{
  print STDERR "test failed comparison of stdout vs. GSfile\n";
  print "Exit code = 2\n";
  exit 2;
}

# check number format in .mt0 file
$retval = MeasureCommon::checkNumberFormatinFourOutput("$CIRFILE.mt0",7);
if ( $retval != 0 )
{
  print STDERR "Failed number format and precision test in .mt0 file\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Passed number format and precision test for .mt0 file\n";
}

# check number format in stdout
$retval = MeasureCommon::checkNumberFormatinFourOutput("$CIRFILE.errmsg",7);
if ( $retval != 0 )
{
  print STDERR "Failed number format and precision test for stdout\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Passed number format and precision test for stdout\n";
}

# -remeasure test must use tolerances, since measure and re-measure results may differ slightly
# on the same platform.
$retval = MeasureCommon::checkRemeasureFour($XYCE,$XYCE_VERIFY,$CIRFILE,$absTol,$relTol,$phaseAbsTol,$phaseRelTol,$zeroTol,"prn",$numSteps);

print "Exit code = $retval\n";
exit $retval;
