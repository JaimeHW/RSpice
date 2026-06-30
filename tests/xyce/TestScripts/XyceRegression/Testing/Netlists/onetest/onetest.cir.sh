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

@filelist = ( "run_xyce_tags", "run_xyce_tags_regression", "Certification_Tests", "Certification_Tests_serial_weekly" );
foreach $file (@filelist) {
  system("rm -fv $file $file.diff");
}

$defaultopts = "--xyce_test=$XYCE_TEST --skipmake --notest --printlist";
$CMD = "$XYCE_TEST/TestScripts/run_xyce_regression $defaultopts --onetest=run_xyce_tags --writelist=run_xyce_tags $XYCE";
print "Running run_xyce_tags... >>$CMD<<\n";
system("$CMD");
$CMD = "$XYCE_TEST/TestScripts/run_xyce_regression $defaultopts --onetest=run_xyce_tags --writelist=run_xyce_tags_regression --taglist=\"+regression\" $XYCE";
print "Running run_xyce_tags_regression... >>$CMD<<\n";
system("$CMD");
$CMD = "$XYCE_TEST/TestScripts/run_xyce_regression $defaultopts --onetest=Certification_Tests --writelist=Certification_Tests $XYCE";
print "Running Certification_Tests... >>$CMD<<\n";
system("$CMD");
$CMD = "$XYCE_TEST/TestScripts/run_xyce_regression $defaultopts --onetest=Certification_Tests --writelist=Certification_Tests_serial_weekly --taglist=\"+serial+weekly\" $XYCE";
print "Running Certification_Tests_serial_weekly... >>$CMD<<\n";
system("$CMD");

$regex = 's/^[#].*$//';
system("perl -pi -e \"$regex\" run_xyce_tags");
system("perl -pi -e \"$regex\" run_xyce_tags_regression");
system("perl -pi -e \"$regex\" Certification_Tests");
system("perl -pi -e \"$regex\" Certification_Tests_serial_weekly");

$correct = 0;
$status = system("diff run_xyce_tags run_xyce_tags.gs > run_xyce_tags.diff");
if ($status == 0) { $correct++; };
$status = system("diff run_xyce_tags_regression run_xyce_tags_regression.gs > run_xyce_tags_regression.diff");
if ($status == 0) { $correct++; };
$status = system("diff Certification_Tests Certification_Tests.gs > Certification_Tests.diff");
if ($status == 0) { $correct++; };
$status = system("diff Certification_Tests_serial_weekly Certification_Tests_serial_weekly.gs > Certification_Tests_serial_weekly.diff");
if ($status == 0) { $correct++; };

$retval = 2;
if ($correct == 4) {
  $retval = 0;
}

print "Exit code = $retval\n"; exit $retval;
