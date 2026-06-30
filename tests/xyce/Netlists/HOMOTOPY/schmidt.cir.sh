#!/usr/bin/env perl

# The input arguments to this script are:
# argv[0] = location of Xyce binary
# argv[1] = location of xyce_verify.pl script
# argv[2] = location of compare script
# argv[3] = location of circuit file to test
# argv[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$GOLD=$ARGV[4];

$COMPARESCRIPT=$XYCE_VERIFY;
$COMPARESCRIPT =~ s/xyce_verify.pl/compareHomotopy.py/;
$COMPARE="python $COMPARESCRIPT ";

$TRUEGOLD = $GOLD;
$TRUEGOLD =~ s/.prn/.HOMOTOPY.dat/;

#$CIR1="arclengthSimple.cir";
$CIR1=$ARGV[3];

# create the output files
$result = system("$XYCE $CIR1");
if ( ( $result != 0 ) ) 
{
  print "Crashed trying to run standard format \n";
  print "Exit code = 13\n";
  exit 13;
}
if ( ! -e "$CIR1.HOMOTOPY.dat")
{
  print "Failed to produce tecplot format \n";
  print "Exit code = 14\n";
  exit 14;
}

$result = system( "$COMPARE $CIR1 $TRUEGOLD  $CIR1.HOMOTOPY.dat" );
if ( $result != 0 )
{
  print $result;
  print "\n";
  print "Failed compare\n";
  print "Exit code = 2\n";
  exit 2;
}

# output final result
print "Exit code = $result\n";
exit $result;

