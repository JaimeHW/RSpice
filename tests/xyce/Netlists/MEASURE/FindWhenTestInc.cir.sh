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

# these are the tolerances used in comparing the measures calculated by this 
# script to those calculated by Xyce.  Zerotol is needed because some measured
# values should be zero (and reltol won't make sense in that case).

my $absTol = 3.0e-3;
my $relTol = 0.02;
my $zeroTol = 1.0e-5;

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

#
# Did we make a measure file
#
if (not -s "$CIRFILE.mt0" ) { print "Exit code = 17\n"; exit 17; }

#
# Now we need to count the number of .MEASURE statemets in the netlist
# and make sure we have some output for each one in the output file
#
open(NETLIST, "$CIRFILE");
$numMeasures=0;
while( $line=<NETLIST> )
{
  if( $line =~ /^\.measure/i)
  {  
    $numMeasures++;
  }
}
close(NETLIST);

#
# Now look for the measure output file and count how many measurements are in it
# Also save the name and value calculated by Xyce.
#
open(RESULTS, "$CIRFILE.mt0");
$numMeasuresFound=0;
$failWarn=0;
my @measureNames;
my @measureVals;
while( $line=<RESULTS> )
{
  if( $line =~ /=/)
  {  
    # parse out NAME = Value 
    chop $line;
    ($name,$sep,$value) = split(/ /, $line);
    print "name = $name value=$value\n";
    push @measureNames, $name;
    push @measureVals, $value;
    $numMeasuresFound++;
  }
}
close(RESULTS);

if( $numMeasures != $numMeasuresFound )
{
  print "Number of requested measures did not match number of printed measures.\n";
  print "Exit code = 2\n"; exit 2; 
}

# in this test case Xyce shouldn't find the allowed window to calculate a 
# result.  Thus the measure's should have values of -1 and -100 which 
# are the default values and user specified default values respectively.

#
# Make sure the measurement was not output multiple times in parallel
# in either the output file or the measure file.
for (my $i=0; $i < $numMeasures; $i++)
{
   $numMeasOut = `grep -ic $measureNames[ $i ] $CIRFILE.out`;
   $failWarn = `grep -icw \"$measureNames[ $i ] failed\" $CIRFILE.out`;
   if ( $numMeasOut != 1 )
   {
     if ( $numMeasOut >= 1 && $failWarn != $numMeasOut-1 )
     {
       print "Xyce printed out $measureNames[ $i ] measure to standard output multiple times.\n";
       $retval = 2;
     }
   }
   $numMeasMT0 = `grep -ic $measureNames[ $i ] $CIRFILE.mt0`;
   if ( $numMeasMT0 != 1 )
   {
     print "Xyce printed out $measureNames[ $i ] measure to file multiple times.\n";
     $retval = 2;
   }
}

if ( $retval != 0 )
{
  print "Exit code = $retval\n";
  exit $retval;
}

$retval = 0;

if( $measureVals[0] != -1 )
{
  print "Measure value 0 was not -1.\n";
  $retval=2;
}

if( $measureVals[1] != -100 )
{
  print "Measure value 1 was not -100.\n";
  $retval=2;
}

if ( $retval != 0 )
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

print "Exit code = $retval\n"; 
exit $retval;

