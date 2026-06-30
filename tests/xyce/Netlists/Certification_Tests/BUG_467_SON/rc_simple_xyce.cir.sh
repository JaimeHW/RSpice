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

use Scalar::Util qw(looks_like_number);

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDOUT=$ARGV[4];

# change ending on gold standard file from ".prn" as passed in to ".mt0" 
# which is the file storing the measure results.

#$GOLDOUT =~ s/prn$/gs\.out/;

if (system("$XYCE -syntax $CIRFILE > $CIRFILE.out 2> $CIRFILE.err"))
{
  print "Exit code = 10\n";
  exit 10;
}

#
# Now look for the output file and compare the total device counts to 
# those in a gold standard output file.
#
open(RESULTS, "$CIRFILE.out");
$device = 'Total Devices';
$R = 'R';
$C = 'C';
$numTotalDevices = 6003;
$numResistors = 3001;
$numCapacitors = 3001;
$devFound = 0;
$resFound = 0;
$capFound = 0;
$retval = -1;

while( $line=<RESULTS> )
{
  # Find the total number of devices
  if ( index($line,$device) >= 0 )
  {
    $devFound = 1;
    if ( index($line,$numTotalDevices) < 0 )
    {
      # This does not have the right number of total devices.
      print "Expected $device = $numTotalDevices, Xyce produced: $line\n";
      $retval=2;
      last;
    }
  }

  # Remove leading spaces on line, otherwise the spaces become 
  # element 0 of "@lineOfDataFromXyce" instead of the first column of data.
  $line =~ s/^\s*//;
  @lineOfDataFromXyce = (split(/[\s,]+/, $line));

  # Find the number of resistors
  if ( $lineOfDataFromXyce[0] eq $R )
  {
    $resFound = 1;
    if ( index($line,$numResistors) < 0 )
    {
      # This does not have the right number of resistors.
      print "Expected $R = $numResistors, Xyce produced: $line\n";
      $retval=2;
      last;
    }
  }
  
  # Find the number of capacitors
  if ( $lineOfDataFromXyce[0] eq $C )
  {
    $capFound = 1;
    if ( index($line,$numCapacitors) < 0 )
    {
      # This does not have the right number of capacitors.
      print "Expected $C = $numCapacitors, Xyce produced: $line\n";
      $retval=2;
      last;
    }
  }
}

close(RESULTS);

# Make sure we detected all of the device lines.
if ( $retval != 2 )
{
  if ( ($devFound==1) && ($resFound==1) && ($capFound==1) )
  { 
    $retval=0;
  }
  else
  {
    print "Script found $device = $devFound, $R = $resFound, $C = $capFound\n";
    $retval=2;
  }
}

if ( $retval != 0 )
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

print "Exit code = $retval\n"; 
exit $retval;


