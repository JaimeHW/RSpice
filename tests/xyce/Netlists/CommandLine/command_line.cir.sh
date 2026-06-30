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
$BADCIRFILE="bad_$CIRFILE";

$failed = 0;
$CMD="$XYCE -h > help.out 2> help.err";
$system_code = system($CMD);
if ($system_code != 0) {
  $exit_code = $system_code >> 8;
  $signal_id = $system_code & 127;
  print STDERR "Xyce -h exited with exit code $exit_code signal $signal_id\n";
  ++$failed;
}

# Remove the lines with -plugin since it might not always be present.
# Also lines with either dakota are only present in a build with --enable-dakota
# while lines containing the string debug are only present for debug builds.
# Also, ignore differences in the number of blank lines, since their number/position 
# may vary depending on the build. This is done with the -B option for grep, in the second 
# grep. Finally, the help test associated with totalview is also not present in all builds.
$CMD="grep -vi -E \"plugin|dakota|debug|totalview\" help.out > helpTrimmed.out 2> helpTrimmed.err";
$system_code = system($CMD);
# this second grep ignores blank lines
$CMD="diff --strip-trailing-cr -biB helpTrimmed.out goldHelpTxt > helpGrep.out 2> helpGrep.err";
$system_code = system($CMD);
if ($system_code != 0) {
  print STDERR "Xyce -h output text did not match gold standard\n";
  ++$failed;
}

$CMD="$XYCE -v > version.out 2> version.err";
$system_code = system($CMD);
if ($system_code != 0) {
  $exit_code = $system_code >> 8;
  $signal_id = $system_code & 127;
  print STDERR "Xyce -v exited with exit code $exit_code signal $signal_id\n";
  ++$failed;
}

$CMD="$XYCE -capabilities > capabilities.out 2> capabilities.err";
$system_code = system($CMD);
if ($system_code != 0) {
  $exit_code = $system_code >> 8;
  $signal_id = $system_code & 127;
  print STDERR "Xyce -capabilities exited with exit code $exit_code signal $signal_id\n";
  ++$failed;
}

$CMD="$XYCE -license > license.out 2> license.err";
$system_code = system($CMD);
if ($system_code != 0) {
  $exit_code = $system_code >> 8;
  $signal_id = $system_code & 127;
  print STDERR "Xyce -license exited with exit code $exit_code signal $signal_id\n";
  ++$failed;
}

$CMD="$XYCE -syntax $CIRFILE > syntax.out 2> syntax.err";
$system_code = system($CMD);
if ($system_code != 0) {
  $exit_code = $system_code >> 8;
  $signal_id = $system_code & 127;
  print STDERR "Xyce -syntax exited with exit code $exit_code signal $signal_id\n";
  ++$failed;
}

$CMD="$XYCE -syntax $BADCIRFILE > syntax.out 2> syntax.err";
$system_code = system($CMD);
if ($system_code == 0) {
  $exit_code = $system_code >> 8;
  $signal_id = $system_code & 127;
  print STDERR "Xyce -syntax exited with exit code $exit_code signal $signal_id\n";
  ++$failed;
}

$CMD="$XYCE -norun > norun.out 2> norun.err";
$system_code = system($CMD);
if ($system_code != 0) {
  $exit_code = $system_code >> 8;
  $signal_id = $system_code & 127;
  print STDERR "Xyce -norun exited with exit code $exit_code signal $signal_id\n";
  ++$failed;
}

$CMD="$XYCE -count $CIRFILE > count.out 2> count.err";
$system_code = system($CMD);
if ($system_code != 0) {
  $exit_code = $system_code >> 8;
  $signal_id = $system_code & 127;
  print STDERR "Xyce -count exited with exit code $exit_code signal $signal_id\n";
  ++$failed;
}

$CMD="$XYCE -junk > junk.out 2> junk.err";
$system_code = system($CMD);
if ($system_code == 0) {
  $exit_code = $system_code >> 8;
  $signal_id = $system_code & 127;
  print STDERR "Xyce -junk exited with exit code $exit_code signal $signal_id\n";
  ++$failed;
}

$CMD="$XYCE -quiet $CIRFILE > quiet.out 2> quiet.err";
$system_code = system($CMD);
if ($system_code != 0) {
  $exit_code = $system_code >> 8;
  $signal_id = $system_code & 127;
  print STDERR "Xyce -quiet exited with exit code $exit_code signal $signal_id\n";
  ++$failed;
}

$CMD="grep -i -E \"Percent complete|Current system time|Estimated time to completion\" quiet.out > quietGrep.out 2> quietGrep.err";
$system_code = system($CMD);
$quiet_exit_code = $system_code >> 8;
if ($quiet_exit_code != 1) {
  $signal_id = $system_code & 127;
  print STDERR "Xyce -quiet output contained progress info. Grep Exit code $quiet_exit_code signal $signal_id\n";
  ++$failed;
}


$retcode = 0;
if ($failed != 0) {
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
