#!/usr/bin/env perl

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
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CIR1="abs.cir";
$CIR2="poly.cir";
$CIR3="dig_count.cir";
$CIR4="bug_138_1.cir";

# Make sure these circuits run and that they produce results.
# These tests were generating seg faults before when AztecOO was
# used as the solver, so that is all that is needed here.
# Since they are nightly tests, the correctness of the result is tested elsewhere.
#
# Added Belos solver to make sure that it doesn't seg fault too.

$CMD="$XYCE -linsolv aztecoo $CIR1 > $CIR1.aztecoo.out 2> $CIR1.aztecoo.err";
if (system("$CMD") != 0) { print "Failed: $CMD\nExit code = 10\n"; exit 10; }

$CMD="$XYCE -linsolv aztecoo $CIR2 > $CIR2.aztecoo.out 2> $CIR2.aztecoo.err";
if (system("$CMD") != 0) { print "Failed: $CMD\nExit code = 10\n"; exit 10; }

$CMD="$XYCE -linsolv aztecoo $CIR3 > $CIR3.aztecoo.out 2> $CIR3.aztecoo.err";
if (system("$CMD") != 0) { print "Failed: $CMD\nExit code = 10\n"; exit 10; }

$CMD="$XYCE -linsolv aztecoo $CIR4 > $CIR4.aztecoo.out 2> $CIR4.aztecoo.err";
if (system("$CMD") != 0) { print "Failed: $CMD\nExit code = 10\n"; exit 10; }

if ((-f "$CIR1.prn") && (-f "$CIR2.prn") && (-f "$CIR3.prn") && (-f "$CIR4.prn"))
{
  $retcode = 0; 
}
else { $retcode = 14; }

system("rm -f $CIR1.prn $CIR2.prn $CIR3.prn $CIR4.prn");

$CMD="$XYCE -linsolv belos $CIR1 > $CIR1.belos.out 2> $CIR1.belos.err";
if (system("$CMD") != 0) { print "Failed: $CMD\nExit code = 10\n"; exit 10; }

$CMD="$XYCE -linsolv belos $CIR2 > $CIR2.belos.out 2> $CIR2.belos.err";
if (system("$CMD") != 0) { print "Failed: $CMD\nExit code = 10\n"; exit 10; }

$CMD="$XYCE -linsolv belos $CIR3 > $CIR3.belos.out 2> $CIR3.belos.err";
if (system("$CMD") != 0) { print "Failed: $CMD\nExit code = 10\n"; exit 10; }

$CMD="$XYCE -linsolv belos $CIR4 > $CIR4.belos.out 2> $CIR4.belos.err";
if (system("$CMD") != 0) { print "Failed: $CMD\nExit code = 10\n"; exit 10; }

if ((-f "$CIR1.prn") && (-f "$CIR2.prn") && (-f "$CIR3.prn") && (-f "$CIR4.prn") && ($retcode==0))
{
}
else { $retcode = 14; }

print "Exit code = $retcode\n"; exit $retcode;
  
