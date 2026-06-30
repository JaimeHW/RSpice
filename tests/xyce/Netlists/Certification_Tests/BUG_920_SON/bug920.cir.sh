#!/usr/bin/env perl

use XyceRegression::Tools;
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

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.  

use File::Basename;

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

@Netlist;
$Netlist[0]="homotopy-no-output.cir";
$Netlist[1]="homotopy-tran-order1.cir";
$Netlist[2]="homotopy-tran-order2.cir";
$Netlist[3]="homotopy-multiple-lines.cir";

# paths to Gold Files and file_compare.pl script
$goldDirName=dirname($GOLDPRN);
$dirname = dirname($XYCE_VERIFY);
$fc = "$dirname/file_compare.pl";

# tolerances for checking the .HOMOTOPY.prn files
my $absTol=1e-5;
my $relTol=1e-3;
my $zeroTol=1e-10;

# remove old files if they exist
foreach $i (0 .. 3)
{
  `rm -f $Netlist[$i].HOMOTOPY.prn $Netlist[$i].prn`;
  `rm -f $Netlist[$i].homotopy.out $Netlist[$i].homotopy.err`;
}

# run Xyce on homotopy-no-print.cir.  Exit if the .prn file is incorrect
$retval = 0;
$GOLDPRN1="$goldDirName/$Netlist[0].prn";
$retval1 = $Tools->wrapXyceAndVerify($XYCE,$XYCE_VERIFY,$GOLDPRN1,$Netlist[0],"$Netlist[0].prn");
if ($retval1 != 0) 
{ 
  print "Error: Failed to run $Netlist[0]\n"; 
  $retval = $retval1;
}
elsif (-s "$Netlist[0].HOMOTOPY.prn" )
{
  # Error has occurred if the homotopy-no-print.cir.HOMOTOPY.prn file WAS made.  
  # It shouldn't have been made since there is no .PRINT HOMOTOPY line in 
  # the homotopy-no-output.cir netlist.
  print "Error: $Netlist[0].HOMOTOPY.prn was made, when it should not\n"; 
  $retval=2; 
}
else
{
  print "Passed test of $Netlist[0]\n";
}

# run Xyce on homotopy-tran-order1.cir, homotopy-tran-order2.cir and
# homotopy-multiple-lines.cir.  Use the same gold file for these three
# tests, since they should produce identical output.
$GOLDFILE1 = "$goldDirName/$Netlist[1].HOMOTOPY.prn";
foreach $i (1 .. 3)
{
  $retval2=$Tools->wrapXyce($XYCE,"$Netlist[$i]");
  if ($retval2 != 0) 
  { 
    print "Error: Failed to run $Netlist[$i]\n"; 
    $retval = $retval2; 
  }
  elsif (not -s "$Netlist[$i].HOMOTOPY.prn" )
  {
    # Error if the HOMOTOPY.prn file was not made 
    print "$CIRFILE$i.HOMOTOPY.prn file is missing\n"; 
    $retval = 14;
  }
  else
  {
    $CMD="$fc $Netlist[$i].HOMOTOPY.prn $GOLDFILE1 $absTol $relTol $zeroTol > $Netlist[$i].homotopy.out 2> $Netlist[$i].homotopy.err";
    $retval2=system($CMD);

    if ( $retval2 != 0 )
    {
      print "test Failed comparison of $Netlist[$i].HOMOTOPY.prn file vs. gold HOMOTOPY.prn file!\n";
      $retval = 2;
    }
    else
    {
      print "Passed comparison of $Netlist[$i].HOMOTOPY.prn files\n";
      $retval = 0;
    }
  }
}

# produce overall exit code.
if ($retval != 0)
{
  print "Test failed\n";
}
else
{
  print "Test passed\n";
}
print "Exit code = $retval\n";
exit $retval;

