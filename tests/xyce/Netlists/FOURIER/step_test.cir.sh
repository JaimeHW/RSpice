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
#$GOLDPRN=$ARGV[4];

# number of steps in the netlist CIRFILE
$numSteps=2;

$retval=0;
# make basename (CB)
$CB = $CIRFILE;
$CB =~ s/.cir$//;

# Clean up from any previous runs
system("rm -f $CIRFILE.prn $CIRFILE.res $CIRFILE.out $CIRFILE.err $CIRFILE.four*");
system("rm -f $CB.s*.cir.prn $CB.s*.cir.out $CB.s*.cir.err $CB.s*.cir.four0");

# create the output files for the Step netlist
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0)
{
  print "Xyce crashed running Step netlist $CIRFILE\n";
  print "See $CIRFILE.out and $CIRFILE.err\n";
  print "Exit code = $retval \n";
  exit $retval;
}

# Did we make a Fourier file for the step netlist
if (not -s "$CIRFILE.four0" ) { return 17; }

# Make the non-step output files and check each non-step .four0 output file against the
# corresponding fourX output from the step netlist
foreach my $idx (0 .. $numSteps-1)
{
  my $NSF="$CB.s$idx.cir"; # name of the non-step file
  $retval=$Tools->wrapXyce($XYCE,"$NSF");
  if ($retval != 0)
  {
    print "Xyce crashed running non-step netlist $NSF for Step $idx\n";
    print "See $NSF.out and $NSF.err\n";
    print "Exit code = $retval \n";
    exit $retval
  }

  # Did we make a Fourier file for the non-step netlist
  if (not -s "$NSF.four0" ) { return 17; }

  my $CMD="diff $CIRFILE.four$idx $NSF.four0 > $CIRFILE.four$idx.out 2> $CIRFILE.four$idx.err";
  $retval = system($CMD);
  $retval = $retval >> 8;

  # check the return value
  if ( $retval != 0 )
  {
    print "Diff Failed for Step $idx. See $CIRFILE.four$idx.out and $CIRFILE.four$idx.err\n";
    print "Exit code = 2\n";
    exit 2;
  }
}

print "Exit code = $retval \n";
exit $retval;
