#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

# The input arguments to this script are set up in
# Xyce_Test/TestScripts/tsc_run_test_suite and are as follows:
# $1 = location of Xyce binary
# $2 = location of xyce_verify.pl script
# $3 = location of compare script
# $4 = location of circuit file to test
# $5 = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# Notes:  ring51_top.cir and ring51_top_l.cir should produce identical
# results - they are both 2-level Newton runs.  ring51_top_l.cir uses LOCA
# for the DC sweep, and ring51_top.cir does a conventional DC sweep.
# ring51.cir is the same as ring51_top.cir, but is a conventional, 1-newton
# solve.

$CIR1="ring51.cir";
$CIR2="ring51_loca.cir";
$CIR3="ring51_loca_defaults.cir";
$CIR4="ring51_loca_arc.cir";

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIR1);
if ($retval != 0) { print STDERR "Xyce of $CIR1 failed\n"; print "Exit code = $retval\n"; exit $retval; }

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIR2);
if ($retval != 0) { print STDERR "Xyce of $CIR2 failed\n"; print "Exit code = $retval\n"; exit $retval; }

print STDERR "test1\n";

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIR3);
if ($retval != 0) { print STDERR "Xyce of $CIR3 failed\n"; print "Exit code = $retval\n"; exit $retval; }

print STDERR "test2\n";

# Compare ring51.cir result to ring51_top.cir result with xyce_verify.
#$retval = -1;
#$retval = system("$XYCE_VERIFY $CIR1 $CIR2.prn $CIR1.prn $CIR1.plotfile > $CIR1.prn.out 2> $CIR1.prn.err");
#if ($retval == 0) { print "Exit code = 0\n"; exit 0; } else { print "Exit code = 2\n"; exit 2; }

print STDERR "test\n";

# Compare ring51.cir to ring51_loca.cir result using diff.
if ( not -s "$CIR1.prn" ) { print STDERR "Missing $CIR1.prn\n"; print "Exit code = 14\n"; exit 14; }
`grep -v "Index" $CIR1.prn | grep -v "End" > $CIR1.prn2`;
if ( not -s "$CIR2.HOMOTOPY.prn" ) { print STDERR "Missing $CIR2.HOMOTOPY.prn\n"; print "Exit code = 14\n"; exit 14; }
`grep -v "Index" $CIR2.HOMOTOPY.prn | grep -v "End" > $CIR2.prn2`;

$retval = system("diff $CIR1.prn2 $CIR2.prn2 > $CIR2.prn.out 2> $CIR2.prn.err");
if ($retval == 0) { print "Exit code = 0\n"; exit 0; } else { print STDERR "Diff of $CIR1.prn2 to $CIR2.prn2 failed\n"; print "Exit code = 2\n"; exit 2; }

# Compare ring51_loca.cir to ring51_loca_defaults.cir result using diff.
if ( not -s "$CIR2.prn" ) { print STDERR "Missing $CIR2.prn\n"; print "Exit code = 14\n"; exit 14; }
`grep -v "Index" $CIR2.prn | grep -v "End" > $CIR2.prn2`;
if ( not -s "$CIR3.HOMOTOPY.prn" ) { print STDERR "Missing $CIR3.HOMOTOPY.prn\n"; print "Exit code = 14\n"; exit 14; }
`grep -v "Index" $CIR3.HOMOTOPY.prn | grep -v "End" > $CIR3.prn2`;

$retval = system("diff $CIR2.prn2 $CIR3.prn2 > $CIR3.prn.out 2> $CIR3.prn.err");
if ($retval == 0) { print "Exit code = 0\n"; exit 0; } else { print STDERR "Diff of $CIR2.prn2 to $CIR3.prn2 failed\n"; print "Exit code = 2\n"; exit 2; }
