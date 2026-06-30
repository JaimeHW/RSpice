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

# Comparison tolerances
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/compare_csd_files/;

`rm -f $CIRFILE.out $CIRFILE.err`;
# Clean up from any previous runs
foreach $idx (1 .. 16)
{
  `rm -f $CIRFILE$idx.csd`; 
}

# create the output files
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval=system($CMD);
if ($retval != 0) 
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

# check each output file
foreach $idx (1 .. 16)
{
  # file names
  $testFileName="$CIRFILE$idx.csd";
  $goldFileName="$CIRFILE$idx.GSfile";

  if ( ! -e $testFileName)
  {
    print "Failed to produce output file \n";
    print "Exit code = 14\n";
    exit 14;
  }

  # open files and check that the files have the same number of lines
  if (not -s "$goldFileName" ) 
  { 
    print STDERR "Missing Gold Standard file: $goldFileName\nExit code = 2\n"; 
    $exitCode=2; 
    print "test Failed!\n";
    print "Exit code = $exitCode\n";
    exit $exitCode;
  }
  if (not -s "$testFileName" ) 
  { 
    print STDERR "Missing Test file: $testFileName\nExit code = 14\n"; 
    $exitCode=14; 
    print "test Failed!\n";
    print "Exit code = $exitCode\n";
    exit $exitCode;
  }

  # compare the test and gold file
  #print "$fc $CIRFILE.errmsg $GSFILE $absTol $relTol $zeroTol\n";
  $CMD="$fc $testFileName $goldFileName $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
  $exitCode = system("$CMD");

  # check the return values
  if ( $exitCode != 0 )
  {
    print "test Failed!\n";
    print "Exit code = 2\n";
    exit 2;
  }
  else
  {
    print "test passed for $idx variables on .PRINT line\n";
  }
}

# final check on the return values
if ( $exitCode != 0 )
{
  print "test Failed!\n";
  print "Exit code = $exitCode\n";
  exit $exitCode;
}
else
{
  print "test passed!\n";
  print "Exit code = $exitCode\n";
  exit $exitCode;
}
