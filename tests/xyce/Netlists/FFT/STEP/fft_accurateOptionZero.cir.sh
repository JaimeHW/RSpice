#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce the appropriate output files, then we return exit code 14.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.csv.out and the STDERR output from
# comparison to go into $CIRFILE.csv.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$CB = $CIRFILE;
$CB =~ s/.cir$//;

system("rm -f $CIRFILE.fft*");
system("rm -f $CB.s*.cir.prn $CB.s*.cir.out $CB.s*.cir.fft0 $CB.s*.cir.err");

# create the output files for the Step netlist
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) {print "Exit code = $retval\n"; exit $retval;}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
    {
        print "Exit code = 2 \n";
        exit 2;
    }
    else
    {
        print "Exit code = 0 \n";
        exit 0;
    }
}

# Did we make both fft files for the step netlist
if ( !(-f "$CIRFILE.fft0")) {
    print STDERR "Missing output file $CIRFILE.fft0\n";
    print "Exit code = 14\n";
    exit 14;
}

if ( !(-f "$CIRFILE.fft1")) {
    print STDERR "Missing output file $CIRFILE.fft1\n";
    print "Exit code = 14\n";
    exit 14;
}

$retcode = 0;

# Make the non-step output files and check each non-step .fft0 output file against the
# corresponding .fftX output from the step netlist
foreach my $idx (0 .. 1)
{
  $NSF="$CB.s$idx.cir"; # name of the non-step file
  $retval=$Tools->wrapXyce($XYCE,"$NSF");
  if ($retval != 0)
  {
    print "Xyce crashed running non-step netlist $NSF for Step $idx\n";
    print "See $NSF.out and $NSF.err\n";
    print "Exit code = $retval\n";
    exit $retval;
  }

  # Did we make a .fft0 file for the non-step netlist
  if (not -s "$NSF.fft0" )
  {
    print "Failed to make file $NSF.fft0";
    $retcode = 17;
  }

  $CMD="diff $CIRFILE.fft$idx $NSF.fft0 > $CIRFILE.fft$idx.out 2> $CIRFILE.fft$idx.err";
  $retval = system($CMD);
  $retval = $retval >> 8;

  # check the return value
  if ( $retval != 0 )
  {
    print "Diff Failed for Step $idx. See $CIRFILE.fft$idx.out and $CIRFILE.fft$idx.err\n";
    $retcode = 2;
  }
}

print "Exit code = $retcode\n";
exit $retcode;
