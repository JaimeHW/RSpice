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

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.  

use File::Basename;

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# Exit if the .prn file was not made
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

# do a valgrind run, if that's been requested
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
  print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
  if (system("$XYCE_VERIFY $CIRFILE junk $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
  {
    #valgrind_check.sh has reported a memory error, test is a failure
    print "Exit code = 2 \n";
    exit 2;
  }
  else
  {
    #valgrind_check.sh has reported no memory errors, pass
    print "Exit code = 0 \n";
    exit 0;
  }
}

# Now check the .HOMOTOPY.PRN file
my $absTol=1e-5;
my $relTol=1e-3;
my $zeroTol=1e-10;

my $dirname = dirname($XYCE_VERIFY);
my $fc = "$dirname/file_compare.pl";
$GOLDPRN = substr($GOLDPRN,0,-3)."HOMOTOPY.prn";
`$fc $CIRFILE.HOMOTOPY.prn $GOLDPRN $absTol $relTol $zeroTol > $CIRFILE.out 2> $CIRFILE.err`;
my $retval=$? >> 8;

if ( $retval != 0 )
{
  print "test Failed comparison of HOMOTOPY.prn file vs. Gold HOMOTOPY.prn file!\n";
  print "Exit code = $retval\n";
  exit $retval;
}
else
{
  print "Passed comparison of HOMOTOPY.prn files\n";
}

print "Exit code = $retval\n";
exit $retval;

