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
$OUTPUTFILE=$CIRFILE . ".out";
print "OUTPUT file is $OUTPUTFILE\n";

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if($retval!=0)
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

# now verify that each warning is NOT there
@warningList1=("Netlist warning: Voltage Node \\\(7\\\) does not have a DC path to ground");
@warningList2=("Netlist warning: Voltage Node \\\(6\\\) does not have a DC path to ground");
@warningList3=("Netlist warning: Voltage Node \\\(2\\\) does not have a DC path to ground");
@warningList4=("Netlist warning: Voltage Node \\\(1\\\) does not have a DC path to ground");
@warningList5=("Netlist warning: Voltage Node \\\(8\\\) does not have a DC path to ground");
@warningList6=("Netlist warning: Voltage Node \\\(3\\\) does not have a DC path to ground");

$retval=$Tools->checkError($OUTPUTFILE, @warningList1);
# the above should return 2 as the warning listed should no longer be there.
if ($retval == 0)
{
  print "This test did not pass as the above line should not have been there anymore.\n";
  print "Exit code = 2\n"; exit 2;
}

$retval=$Tools->checkError($OUTPUTFILE, @warningList2);
# the above should return 2 as the warning listed should no longer be there.
if ($retval == 0)
{
  print "This test did not pass as the above line should not have been there anymore.\n";
  print "Exit code = 2\n"; exit 2;
}

$retval=$Tools->checkError($OUTPUTFILE, @warningList3);
# the above should return 2 as the warning listed should no longer be there.
if ($retval == 0)
{
  print "This test did not pass as the above line should not have been there anymore.\n";
  print "Exit code = 2\n"; exit 2;
}

$retval=$Tools->checkError($OUTPUTFILE, @warningList4);
# the above should return 2 as the warning listed should no longer be there.
if ($retval == 0)
{
  print "This test did not pass as the above line should not have been there anymore.\n";
  print "Exit code = 2\n"; exit 2;
}

$retval=$Tools->checkError($OUTPUTFILE, @warningList5);
# the above should return 2 as the warning listed should no longer be there.
if ($retval == 0)
{
  print "This test did not pass as the above line should not have been there anymore.\n";
  print "Exit code = 2\n"; exit 2;
}

$retval=$Tools->checkError($OUTPUTFILE, @warningList6);
# the above should return 2 as the warning listed should no longer be there.
if ($retval == 0)
{
  print "This test did not pass as the above line should not have been there anymore.\n";
  print "Exit code = 2\n"; exit 2;
}

$retval=0;
print "Exit code = $retval\n"; 
exit $retval;

