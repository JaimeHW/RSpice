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

# make name of gold .mt0 and .prn file names
$GOLD1MT0 = $GOLDPRN;
$GOLD1MT0 =~ s/\.cir\.prn/1\.cir\.mt0/;
$GOLD2MT0 = $GOLDPRN;
$GOLD2MT0 =~ s/\.cir\.prn/2\.cir\.mt0/;

$GOLD1PRN = $GOLDPRN;
$GOLD1PRN =~ s/\.cir\.prn/1\.cir\.prn/;
$GOLD2PRN = $GOLDPRN;
$GOLD2PRN =~ s/\.cir\.prn/2\.cir\.prn/;
#print "GOLD1MT0 = $GOLD1MT0\n";
#print "GOLD2MT0 = $GOLD2MT0\n";
#print "GOLD1PRN = $GOLD1PRN\n";
#print "GOLD2PRN = $GOLD2PRN\n";

# make base names of netlists
$CIR1 = $CIRFILE;
$CIR1 =~ s/\.cir/1\.cir/;
$CIR2 = $CIRFILE;
$CIR2 =~ s/\.cir/2\.cir/;
#print "CIR1 = $CIR1\n";
#print "CIR2 = $CIR2\n";

# find root of Xyce build directory
$XYCE_BASE = $XYCE;

# remove files from previous runs
system("rm -f $CIRFILE.out $CIR1.prn $CIR1.mt0 $CIR2.prn $CIR2.mt0");

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
$retval = system("python $PYFILE $XYCE_LIB_DIR > $CIRFILE.out");
if ($retval != 0)
{
  print "Netlists failed to run via Python-based XyceCInterface\n";
  print "Exit code = 2\n"; exit 2;
}
else
{
  # check output files
  if (not -s "$CIR1.prn" ) { print "Exit code = 14\n"; exit 14; }
  if (not -s "$CIR1.mt0" ) { print "Exit code = 17\n"; exit 17; }
  # check output files
  if (not -s "$CIR2.prn" ) { print "Exit code = 14\n"; exit 14; }
  if (not -s "$CIR2.mt0" ) { print "Exit code = 17\n"; exit 17; }
}

# Verify that the .prn and .mt0 files are correct.
$retval=0;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$absTol=1e-3;
$relTol=1e-2;
$zeroTol=1e-10;

# check .prn files
$CMD="$XYCE_VERIFY $CIR1 $GOLD1PRN $CIR1.prn";
if (system($CMD) != 0) 
{
  print STDERR "Verification failed on file $CIR1.prn";
  $retval = 2;
}

$CMD="$XYCE_VERIFY $CIR2 $GOLD2PRN $CIR2.prn";
if (system($CMD) != 0) 
{
  print STDERR "Verification failed on file $CIR2.prn";
  $retval = 2;
}

#check .mt0 files
$CMD="$fc $CIR1.mt0 $GOLD1MT0 $absTol $relTol $zeroTol";
if (system($CMD) != 0) 
{
  print STDERR "Verification failed on file $CIR1.mt0";
  $retval = 2;
}

#check .mt0 file
$CMD="$fc $CIR2.mt0 $GOLD2MT0 $absTol $relTol $zeroTol";
if (system($CMD) != 0) 
{
  print STDERR "Verification failed on file $CIR2.mt0";
  $retval = 2;
}

# check for XyceCInterface return codes in stdout
@searchstrings = ("calling initialize for xyceObj1 with netlist runMultipleSims1.cir",
                  "return value from initialize for xyceObj1 is 1",
                  "calling initialize for xyceObj2 with netlist runMultipleSims2.cir",
                  "return value from initialize for xyceObj2 is 1",
                  "Calling runSimulation for xyceObj1",
                  "return value from runSimulation for xyceObj1 is 1",
                  "Calling runSimulation for xyceObj2",
                  "return value from runSimulation for xyceObj2 is 1",
                  "calling close for xyceObj1",
                  "calling close for xyceObj2"
);
if ( $Tools->checkError("$CIRFILE.out",@searchstrings) != 0) 
{
 print "Failed to find all of the correct XyceCInterface return codes in stdout\n"; 
 $retval = 2; 
}

print "Exit code = $retval\n"; exit $retval;

