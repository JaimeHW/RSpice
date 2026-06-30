#!/usr/bin/env perl

use RawFileCommon;

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
# output from comparison to go into $CIRFILE.raw.out and the STDERR output from
# comparison to go into $CIRFILE.raw.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDRAW=$ARGV[4];

$GOLDRAW =~ s/\.prn$//; # remove the .prn at the end.

# Remove the previous output files.  $CIRFILE.FD.prn file should not be made, but
# remove it if it was made during a previous run.
system("rm -f $CIRFILE.raw* $CIRFILE.err $CIRFILE.out $CIRFILE.FD.*");

# run Xyce and check for output files
$CMD="$XYCE -r $CIRFILE.raw -a $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval=system($CMD);

if ($retval != 0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n";
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIRFILE;
    exit 13;
  }
  else
  {
    print "Exit code = 10\n";
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE;
    exit 10;
  }
}

$xyceexit=0;
if ( -f "$CIRFILE.FD.prn" ) {
      print STDERR "Extra output file $CIRFILE.FD.prn\n";
      $xyceexit=2;
}

if ( !(-f "$CIRFILE.raw")) {
    print STDERR "Missing output file $CIRFILE.raw\n";
    $xyceexit=14;
}

# The AC sensitivity output file should be made
if ( !(-f "$CIRFILE.FD.SENS.prn") ) {
      print STDERR "Missing output file $CIRFILE.FD.SENS.prn\n";
      $xyceexit=2;
}

if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# verify the sensitivity output file
$retcode = 0;

$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$freqreltol=1e-6;
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;

$CMD="$XYCE_ACVERIFY $CIRFILE.FD.SENS.prn $GOLDRAW.FD.SENS.prn $absTol $relTol $zeroTol $freqreltol > $CIRFILE.FD.SENS.prn.out 2> $CIRFILE.FD.SENS.prn.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval != 0)
{
  print STDERR "Comparator exited with exit code $retval on file $CIRFILE.FD.SENS.prn\n";
  $retcode = 2;
}

# verify the AC raw output
if (system("grep -v 'Date:' $CIRFILE.raw > $CIRFILE.raw.filtered 2>$CIRFILE.raw.filtered.out") != 0)
{
  print STDERR "Date line not found in file $CIRFILE.raw.filtered, see $CIRFILE.raw.filtered.out\n";
  $retcode = 2;
}

$compareVal = RawFileCommon::compareRawFiles($XYCE_VERIFY,$CIRFILE,$GOLDRAW,$absTol,$relTol,$zeroTol);
if ($compareVal != 0)
{
  print "Verification failed on .raw file\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;

