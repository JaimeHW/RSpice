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
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# python files
@PYFILE;
$PYFILE[0]="DCOPfailuresRunSimulation.py";
$PYFILE[1]="DCOPfailuresSimulateUntil.py";

# log files
@LOGFILE;
$LOGFILE[0]="DCOPfailuresRunSimulationLog";
$LOGFILE[1]="DCOPfailuresSimulateUntilLog";

# remove logfiles from previous runs
system("rm -f $LOGFILE[0] $LOGFILE[1]");

# strings to search for in LOGFILEs
@searchstrings0 = ("DC Operating Point Failed.  Exiting transient loop",
                   "return value from runSimulation is 0",
                   "calling close",
                   "MAXV1 = FAILED");

@searchstrings1 = ("Calling simulateUntil for requested_time = 0.100",
                   "DC Operating Point Failed.  Exiting transient loop",
                   "simulateUntil status = 0 and actual_time = 0.000",
                   "calling close",
                   "MAXV1 = FAILED");

# find root of Xyce build directory
$XYCE_BASE = $XYCE;

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

foreach $idx (0 .. 1)
{ 
  print "Testing with logfile = $LOGFILE[$idx]\n";

  # remove files from previous runs
  system("rm -f $CIRFILE.prn $CIRFILE.mt0");

  # run the netlist via the Python version of XyceCInterface
  # -u makes python's stdout and stderr unbuffered which is needed
  # keep Xyce and python output in a consistent order
  $retval = system("python -u $PYFILE[$idx] $XYCE_LIB_DIR > $LOGFILE[$idx]");
  if ($retval != 0)
  {
    print "Netlist failed to run via Python-based XyceCInterface for logfile $LOGFILE\n";
    print "Exit code = 2\n"; exit 2;
  }
  else
  {
    # check that output .prn file was not made
    if ( -s "$CIRFILE.prn" )
    { 
      print ".prn file made when it should not be\n";
      print "Exit code = 2\n"; exit 2; 
    }
  }

  # check for XyceCInterface return codes in stdout
  $ceVal=0;
  if ($idx == 0)
  {
    $ceVal = $Tools->checkError("$LOGFILE[$idx]",@searchstrings0)
  }
  else
  {
    $ceVal = $Tools->checkError("$LOGFILE[$idx]",@searchstrings1)
  }
                   
  if ( $ceVal != 0) 
  {
   print "Failed to find all of the correct XyceCInterface information in $LOGFILE[$idx]\n"; 
   print "Exit code = 2\n"; exit 2; 
  }
}

# success if we reached here
print "Exit code = 0\n"; exit 0;

