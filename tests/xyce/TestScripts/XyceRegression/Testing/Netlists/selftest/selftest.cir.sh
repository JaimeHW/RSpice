#!/usr/bin/env perl

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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

# 07/18/07 tscoffe:  This function, checkTags, assumes the following:
# If the name of this test file is foo.cir then the gold standard must be:
# foo_list.gs
# And the output of the run_xyce_regression command will go into:
# foo_list.out
# And the output of the differences in the testlists will go into:
# foo_list.tmp1 and foo_list.tmp2

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CKT=$ARGV[3];
$GOLD_PRN=$ARGV[4];

$XYCE_TEST=$XYCE_VERIFY;
$XYCE_TEST =~ s/TestScripts\/xyce_verify.pl//;

$retval = system("$XYCE_TEST/TestScripts/run_xyce_regression --selftest");
# run_xyce_regression --selftest prints its own exit code to output
exit($retval);

