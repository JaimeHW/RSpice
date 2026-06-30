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
$GOLDPRN=$ARGV[4];

# python file
$PYFILE=$CIRFILE;
$PYFILE =~s/cir/py/;

# find root of Xyce build directory
$XYCE_BASE = $XYCE;

$retval=0;

# remove files from previous runs
system("rm -f $CIRFILE.out $CIRFILE.prn");

$XYCE_LIB_DIR="";

# set the environment variables
$LD_LIBRARY_PATH=$ENV{'LD_LIBRARY_PATH'};

if($XYCE_BASE =~ s/\/src\/Xyce$// )
{ # uninstalled copy of Xyce.  Look for extra libraries in utils/XyceCInterface directory
  $XYCE_BASE =~ s/\/src\/Xyce$//;
  $LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$XYCE_BASE/utils/XyceCInterface/.libs";
  $XYCE_LIB_DIR="$XYCE_BASE/utils/XyceCInterface/.libs";
}
if($XYCE_BASE =~ s/\/bin\/Xyce$// )
{
  # installed copy of Xyce look for libraries in lib directory
  $XYCE_BASE =~ s/\/bin\/Xyce$//;
  $LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$XYCE_BASE/lib";
  $XYCE_LIB_DIR="$XYCE_BASE/lib";
}
$ENV{'LD_LIBRARY_PATH'} = $LD_LIBRARY_PATH;
print "setting ld library path to $LD_LIBRARY_PATH\n";

$pythonRetcode=0;
# run the netlist via the Python version of XyceCInterface
# -u makes python's stdout and stderr unbuffered which is needed
# keep Xyce and python output in a consistent order
$pythonRetcode = system("python -u $PYFILE $XYCE_LIB_DIR > $CIRFILE.out");
$pythonRetcode = $pythonRetcode>>8;
print ("Python return code was: $pythonRetcode\n");

if ($pythonRetcode == 0)
{
  print "Netlist ran via Python-based XyceCInterface to completion, when it shouldn't\n";
  print "Exit code = 2\n"; exit 2;
}
else
{

  # In this case the output .prn file should not be made
  if (!-s "$CIRFILE.prn" )
  {
    print "$CIRFILE.prn file is missing\n";
    $retval=2;
  }

  # check for error messages in Xyce stdout.
  @searchstrings = ("return value from initialize is 1",
    "Calling simulateUntil for requested_time = 0.500",
    "simulateUntil status = 1 and actual_time = 0.500",
    "Calling simulateUntil for backwards requested_time = 0.400",
    "Netlist error: requestedUntilTime <= current simulation time in",
    "simulateUntil\\(\\) call.  Simulation will abort.",
    "Simulation aborted due to error.  There are 0 MSG_FATAL errors and 1 MSG_ERROR",
    "Xyce Abort"
  );
  if ( $Tools->checkError("$CIRFILE.out",@searchstrings) != 0)
  {
    print "Failed to find all of the correct Xyce error messages in stdout\n";
    $retval=2;
  }
}

# check that python return code is 1
if ($pythonRetcode != 1)
{
  print "Python script exited with unexpected code\n";
  $retval=2;
}

print "Exit code = $retval\n"; exit $retval;

