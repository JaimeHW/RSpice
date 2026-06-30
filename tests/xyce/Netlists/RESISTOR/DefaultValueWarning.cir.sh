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
$CIRFILE=$ARGV[3];
$GOLDFILE=$ARGV[4];

if (defined($verbose)) { $Tools->setVerbose(1); }

# Clean up droppings from any previous run in this directory.
`rm -f $CIRFILE.prn $CIRFILE.prn.out $CIRFILE.prn.err $CIRFILE.err $CIRFILE.out $CIRFILE.res`;

# First run and check for the presence of the warning messages
@searchstrings = (["Netlist warning in file DefaultValueWarning.cir at or near line 12",
     "Expected value field for device R1, continuing with value of 0"],
    ["Netlist warning in file DefaultValueWarning.cir at or near line 12",
     "Device instance R1: Resistance is set to 0, setting to the default, 1000 ohms"],
    ["Netlist warning in file DefaultValueWarning.cir at or near line 20",
     "Device instance R2: Resistance is set to 0, setting to the default, 1000 ohms"],
    ["Netlist warning in file DefaultValueWarning.cir at or near line 25",
     "Device instance R3: Resistance is set to 0, setting to the default, 1000 ohms"]);

$retval = $Tools->runAndCheckGroupedWarning($CIRFILE,$XYCE,@searchstrings);

# These are the tolerances used in comparing the Gold and test .prn file.  
$absTol = 1.0e-6;
$relTol = 1e-4;
$zeroTol = 1.0e-12;

if ($retval !=0) 
{     
  print STDERR "Failed to run $CIRFILE or Warning Messages incorrect.\n";     
  print "Exit code = $retval\n";     
  exit $retval; 
}
else
{
  # now check that the .prn file is actually correct for the default values for
  # each resistor.  Use file_compare since xyce verify would re-run the .CIR file.
    my $dirname = `dirname $XYCE_VERIFY`;
    chomp $dirname;
    my $fc = "$dirname/file_compare.pl";
    `$fc $CIRFILE.prn $GOLDFILE $absTol $relTol $zeroTol > $CIRFILE.prn.out 2> $CIRFILE.prn.err`;  
    $retval=$? >> 8;
}

if ($retval !=0)
{
  print STDERR "Warning measure was correct, but .PRN file failed compare\n";
}
else
{
  print ".PRN file is correct\n";
}

print "Exit code = $retval\n";
exit $retval;

