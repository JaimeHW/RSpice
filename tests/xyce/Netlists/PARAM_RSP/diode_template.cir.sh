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
my @paramsFiles = ( "diode_template_params1.in",  "diode_template_params2.in", "diode_template_params3.in" );
my @responseFiles = ("diode_template_results1.out", "diode_template_results2.out", "diode_template_results3.out");

for $i (0..2)
{
  # have to call Xyce manually here to stick in extra args
  $CMD="$XYCE -prf $paramsFiles[$i] -rsf $responseFiles[$i] $CIRFILE > $CIRFILE.$i.out";
  $retval=system($CMD)>>8;
  if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
  if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }
  if (not -s "$CIRFILE.mt0" ) { print "Exit code = 17\n"; exit 17; }
}

# the response values from the three $responseFile[123] should all 
# be different as the underlying model should have changed at each 
# simulation.
#
# accepted values are:
#  Diode Model              average current from 0.4ms to 0.5ms
#    1                         IVMON = 0.000900086 
#    2                         IVMON = 0.000851004  
#    3                         IVMON = 0.000837878 
#

# I could parse these out of the Xyce output but then if Xyce changed because something 
# got broken, then I wouldn't detect it here.  By storing the accepted results here
# they are under revision control.  Could be more obvious if I stored them in the 
# gold standard directory. 
my @ivMonValues = (0.000900086, 0.000851004, 0.000837878);
#

for $i (0..2)
{
  open(RESULT_FILE, $responseFiles[$i]);
  my $absError = 0.0;
  my $relError = 0.0;
  my $value = 0.0;
  while( $line=<RESULT_FILE> )
  {
    chop $line;
    # Remove leading spaces on line, otherwise the spaces become element 0 of "@lineOfDataFromXyce" instead of the first column of data.
    $line =~ s/^\s*//;
    #    print STDERR "Line of data = \'$line\'\n";
    ($value, $sep, $name) = (split(/\s+/, $line));
  }
  close(RESULT_FILE);
  $absError = $ivMonValues[$i] - $value;
  $relError = $absError / $value;
  if( (abs( $absError ) > $absTol) || (abs( $relError ) > $relTol) ) 
  {
    print "\nSimulation $i is out of tolerance with absErr = $absError and relErr = $relError : $value != $ivMonValues[$i] \n";
    $retval = 2;
  }
}


if ( $retval != 0 )
{
  print "\nExit code = $retval\n"; 
  exit $retval;
}

print "\nExit code = $retval\n"; 
exit $retval;

