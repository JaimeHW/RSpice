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

# remove files from previous runs
system("rm -f $CIRFILE.out $CIRFILE.prn $CIRFILE.gidArrays");

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

$retval=0;
# run the netlist via the Python version of XyceCInterface
# -u makes python's stdout and stderr unbuffered which is needed
# keep Xyce and python output in a consistent order
$retval = system("python -u $PYFILE $XYCE_LIB_DIR > $CIRFILE.out");
if ($retval != 0)
{
  print "Netlist failed to run via Python-based XyceCInterface\n";
  print "Exit code = 2\n"; exit 2;
}

# check for XyceCInterface return codes in stdout.
@searchstrings = ("return value and numAdjNodes for V1 is 1 2",
                  "return value for getAdjGIDsForDevice for V1 is 1",
                  "return value and numAdjNodes is for R1 1 2",
                  "return value for getAdjGIDsForDevice for R1 is 1",
                  "return value and numAdjNodes for X1:R1 is 1 2",
                  "return value for getAdjGIDsForDevice for X1:R1 is 1",
                  "return value and numAdjNodes for r3 is 1 2",
                  "return value for getAdjGIDsForDevice for r3 is 1",
                  "return value and numAdjNodes for YADC!adc1 is 1 2",
                  "return value for getAdjGIDsForDevice for YADC!adc1 is 1",
                  "Netlist warning: Device RBOGO not found",
                  "return value and numAdjNodes for RBOGO is 0 0",
                  "Netlist warning: Device RBOGO not found",
                  "return value for getAdjGIDsForDevice for RBOGO is 0",
                  "\\[\\]"
);
if ( $Tools->checkError("$CIRFILE.out",@searchstrings) != 0)
{
  print "Failed to find all of the correct XyceCInterface return codes in stdout\n";
  $retval = 2;
}

# now check that the GIDs are sensible
print "Checking GID values for consistency\n";
$CMD = "grep \"\\[\" $CIRFILE.out > $CIRFILE.gidArrays";
$retcode = system($CMD);
if ($retcode != 0)
{
  print "Error grepping for GIDs in $CIRFILE.out\n";
  $retval = 2;
}
else
{
  if (not -s "$CIRFILE.gidArrays" )
  {
    print STDERR "Missing Test file: $CIRFILE.gidArrays\n";
    $retval = 2;
  }
  else
  {
    open(TESTFILE,"$CIRFILE.gidArrays");
    $lineCount=0;
    $prevLine="";
    $prevGID = -2 ; # this GID number is invalid
    while ( ($line=<TESTFILE>) && ($lineCount < 5) )
    {
      $lineCount++;
      chop $line;
      # Remove leading spaces on lineGS, otherwise the spaces become
      # element 0 of "@gsData" instead of the first column of data.
      $line =~ s/^\s*//;
      @lineData = (split(/[\s,]+/, $line));

      if ($lineCount == 1)
      {
        if ( ($lineData[0] ne "\[-1") || ($#lineData !=1) )
        {
          print "Invalid GIDs for line 1 in $CIRFILE.gidArrays\n";
          $retval = 2;
        }
      }
      elsif ( $lineCount < 5)
      {
        $tmpStr = $lineData[0];
        $tmpStr =~ s/^.//s;
        $currGID = $tmpStr;
        #print "currGID and prevGID =  $currGID $prevGID\n";
        if ( ($currGID ne $prevGID) || ($#lineData !=1) )
        {
          print "Invalid GIDs for line $lineCount and previous line in $CIRFILE.gidArrays\n";
          $retval = 2;
        }
      }
      else
      {
        if ($line ne $prevLine)
        {
          print "Invalid GIDs for line $lineCount and previous line in $CIRFILE.gidArrays\n";
          $retval = 2;
        }
      }

      if ($#lineData ==1)
      {
        $tmpStr = $lineData[1];
        $tmpStr =~ s/.$//s;
        $prevGID = $tmpStr;
        $prevLine = $line;
      }
    }

    close(TESTFILE);
  }
}

print "Exit code = $retval\n"; exit $retval;
