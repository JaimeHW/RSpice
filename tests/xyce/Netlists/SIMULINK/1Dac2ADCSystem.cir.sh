#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();
use Cwd;

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This script will attempt to run Simulink through Matlab
# on a Simulink problem that will call Xyce.  To do this
# is must determine based on the path to Xyce the location 
# of the Xyce-Simulink interface library and the 
# location of the Xyce-Simulink interface file that Matlab reads
# as well as the actual problem.
#
# if we are testing from an installed copy of Xyce
# /path/to/XyceInstall/bin/Xyce 
# /path/toXyceInstall/lib             <---- location of needed shared libraries 
# /path/toXyceInstall/share/simulink  <---- locaiton of needed matlab files.
#


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


# Simulink file
$SimulinkFile="Driver1_1DAC_2ADC.slx";

# matlab command to run the simulink file 
# Note don't include the ".m" extension as this will be run from within matlab.
$matlabCommand="Run1Dac2ADCSystem";

# make name of gold .mt0 file
$GOLDMT0 = $GOLDPRN;
$GOLDMT0 =~ s/prn/mt0/;

# find root of Xyce build directory
$XYCE_BASE = $XYCE;

# remove files from previous runs
#system("rm -f $CIRFILE.out $CIRFILE.prn $CIRFILE.mt0");

$XYCE_LIB_DIR="";
$XYCE_SIMULINK_DIR="";

if($XYCE_BASE =~ s/\/src\/Xyce$// )
{ # uninstalled copy of Xyce.  Look for extra libraries in utils/SimulinkInterface directory
  # and use current test directory for XyceBlocks.slx
  $XYCE_BASE =~ s/\/src\/Xyce$//;
  $XYCE_LIB_DIR="$XYCE_BASE/utils/SimulinkInterface";
  $XYCE_SIMULINK_DIR = Cwd::cwd();
}
if($XYCE_BASE =~ s/\/bin\/Xyce$// )
{
  # installed copy of Xyce look for libraries in lib directory
  $XYCE_BASE =~ s/\/bin\/Xyce$//;
  $XYCE_LIB_DIR="$XYCE_BASE/lib";
  $XYCE_SIMULINK_DIR = "$XYCE_BASE/share/simulink";
}

# write the needed matlab command to a file 
open(MF, '>', "$matlabCommand.m") or die $!;
$matlabString=qq{
try
  addpath('$XYCE_LIB_DIR','$XYCE_SIMULINK_DIR');
  sim('$SimulinkFile');
catch
  exit;
end
exit;
};
print MF $matlabString;
close(MF);

$retval=0;

# run the netlist via the Python version of XyceCInterface
$CMD="matlab -nodisplay -nosplash -nodesktop -batch $matlabCommand > $CIRFILE.out 2> $CIRFILE.err";
$retval = system($CMD);
if ($retval != 0)
{
  print "Netlist failed to run via Simulink.\n";
  print "Exit code = 2\n"; exit 2;
}
else
{
  # check output files
  if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }
  #if (not -s "$CIRFILE.mt0" ) { print "Exit code = 17\n"; exit 17; }
}

# Verify that the .prn and .mt0 files are correct.
$retval=0;
#$fc = $XYCE_VERIFY;
#$fc=~ s/xyce_verify/file_compare/;
#$absTol=1e-3;
#$relTol=1e-2;
#$zeroTol=1e-10;

# check .prn file
$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn";
if (system($CMD) != 0) 
{
  print STDERR "Verification failed on file $CIRFILE.prn";
  $retval = 2;
}

#check .mt0 file
#$CMD="$fc $CIRFILE.mt0 $GOLDMT0 $absTol $relTol $zeroTol";
#if (system($CMD) != 0) 
#{
#  print STDERR "Verification failed on file $CIRFILE.mt0";
#  $retval = 2;
#}


print "Exit code = $retval\n"; exit $retval;

