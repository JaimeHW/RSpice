#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are: 
# $1 = location of Xyce binary
# $2 = location of xyce_verify.pl script
# $3 = location of compare script 
# $4 = location of circuit file to test
# $5 = location of gold standard prn file

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
$paramsFile = "params.in.1";
$responseFile = "results.out.1";
# have to call Xyce manually here to stick in extra args
$CMD="$XYCE -prf $paramsFile -rsf $responseFile $CIRFILE > $CIRFILE.out";
$retval=system($CMD)>>8;

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
  print "Numer of requested measures did not match number of printed measures.\n";
  print "Exit code = 2\n"; exit 2; 
}

# Now look at the requested results file and ensure
# that the calculated measure is reported there 
open(XYCE_RESULTS, "$responseFile");
my @reportedMeasureVals;
my @reportedMeasureNames;

while( $line=<XYCE_RESULTS> )
{
  # save var values 
  chop $line;
  # Remove leading spaces on line, otherwise the spaces become element 0 of "@lineOfDataFromXyce" instead of the first column of data.
  $line =~ s/^\s*//;
  #    print STDERR "Line of data = \'$line\'\n";
  ($value, $sep, $name) = (split(/\s+/, $line));
  push @reportedMeasureNames, $name;
  push @reportedMeasureVals, $value;
}
close(XYCE_RESULTS);


my @absMeasureError;
my @relMeasureError;

# Form the average = integral/total time period, and absolute vs. relative errors.
for $i (0 .. 1 )
{
  $absMeasureError[$i] = abs( $measureVals[$i] - $reportedMeasureVals[$i] );
  if( abs( $measureVals[$i] ) != 0.0 )
  {
    $relMeasureError[$i] = abs($absMeasureError[$i] / $measureVals[$i]);
  }
}

$retval = 0;
foreach $i (0 .. 1)
{
  print "For measure $measureNames[$i] calculated value = $measureVals[$i]: abserror = $absMeasureError[$i] relerror=$relMeasureError[$i]\n";
  if( (abs($absMeasureError[$i]) > $absTol) || ((abs($measureVals[$i]) > $zeroTol) && (abs($relMeasureError[$i]) > $relTol) ))
  {
    print "Out of tolerance with abstol = $absTol and reltol = $reltol \n";
    $retval = 2;
  }
}


if ( $retval != 0 )
{
  print "Exit code = $retval\n"; 
  exit $retval;
}
  
print "Exit code = $retval\n"; 
exit $retval;

