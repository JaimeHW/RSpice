#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
$GOLDPRN=$ARGV[4];

# set overall return value
$retval=0;

# test FILE= on .PRINT line
print "Testing FILE= on .PRINT line\n";
# this string should be in the output of this failed Xyce run
@searchstrings = ("Failure opening bogo_dir/output.prn");
$retval1 = $Tools->runAndCheckError($CIRFILE,$XYCE,@searchstrings);

if ($retval1 != 0)
{
  print "Test failed for FILE= on .PRINT line\n";
  $retval=$retval1;
}

# now do the test with the -r command line option
print "Testing -r command line option\n";
$CMD="$XYCE -r bogo_dir/output.raw $CIRFILE > $CIRFILE.raw.out 2>$CIRFILE.raw.err";
$retval2=system($CMD)>>8;

if ($retval2 == 0)
{
  print "Xyce ran (for -r) when it should have failed\n";
  $retval=2;
}
else
{
  # this string should be in the output of this failed Xyce run  
  @searchstrings = ("Failure opening bogo_dir/output.raw");

  $retval2 = $Tools->checkError("$CIRFILE.raw.out",@searchstrings);

  if ($retval2 != 0)
  {
    print "Test failed for -r command line option\n";
    $retval=$retval2;
  }
}

# If we get here and $retval is not equal to zero, then the 
# return exit code will be from the last test failure.  However, 
# the .sh file will print out a list of which tests failed.
print "Exit code = $retval\n"; exit $retval; 
