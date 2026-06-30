#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

if (defined($verbose)) { $Tools->setVerbose(1); }

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
$GOLDPRN=$ARGV[4];

$DASHOFILE="tranStepNumColOutput";
$TRANCONT="_findv2";

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end

# hard code the number of steps
$stepNum=2;

# Remove the previous output files, including some that are only made if the
# previous run failed.
system("rm -f $CIRFILE.prn $CIRFILE.csv $CIRFILE.err $CIRFILE.out $CIRFILE.SENS*");
system("rm -f $DASHOFILE* tranStepNumColFoo");

# run Xyce
$CMD="$XYCE -o $DASHOFILE.prn -delim COMMA $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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

# check for output files
$xyceexit=0;
if ( (-f "$CIRFILE.prn") || (-f "$CIRFILE.csv") ){
  print STDERR "Extra output file $CIRFILE.prn or $CIRFILE.csv\n";
  $xyceexit=2;
}

if ( (-f "$CIRFILE.SENS.prn") || (-f "$CIRFILE.SENS.csv")) {
  print STDERR "Extra output file $CIRFILE.SENS.prn or $CIRFILE.SENS.csv\n";
  $xyceexit=2;
}

if ( -f "tranStepNumColFoo") {
  print STDERR "Extra output file tranStepNumColFoo\n";
  $xyceexit=2;
}

if ( !(-f "$DASHOFILE.prn") ){
  print STDERR "Missing -o output file for .PRINT TRAN, $DASHOFILE.prn\n";
  $xyceexit=14;
}

if ( !(-f "$DASHOFILE.SENS.prn") ){
  print STDERR "Missing -o output file for .PRINT SENS, $DASHOFILE.SENS.prn\n";
  $xyceexit=14;
}

if ( !(-f "$DASHOFILE.res") ){
  print STDERR "Missing results file, $DASHOFILE.res\n";
  $xyceexit=2;
}

foreach $i (0 ... $stepNum-1)
{
  if ( !(-f "$DASHOFILE.mt$i") ){
    print STDERR "Missing -o output file for .MEASURE TRAN, $DASHOFILE.mt$i\n";
   $xyceexit=2;
  }

  if ( !(-f "$DASHOFILE$TRANCONT.mt$i") ){
    print STDERR "Missing -o output file for .MEASURE TRAN_CONT, $DASHOFILE$TRANCONT.mt$i\n";
    $xyceexit=2;
  }

  if ( !(-f "$DASHOFILE.fft$i") ){
    print STDERR "Missing -o output file, $DASHOFILE.fft$i\n";
    $xyceexit=2;
  }

  if ( !(-f "$DASHOFILE.four$i") ){
    print STDERR "Missing -o output file, $DASHOFILE.four$i\n";
    $xyceexit=2;
  }
}

if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# Now verify the .PRINT output files.  Use file_compare.pl since I'm also
# testing print line concatenation and that the simulation footer is present.
$retcode=0;

$XPLAT_DIFF = $XYCE_VERIFY;
$XPLAT_DIFF =~ s/xyce_verify/xplat_diff/;

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;
$abstol=1e-4;
$reltol=1e-3;
$zerotol=1e-6;

$cff=$XYCE_VERIFY;
$cff =~ s/xyce_verify/compare_fourier_files/;
$phaseabstol = $abstol;
$phasereltol = $reltol;

$CMD="$fc $DASHOFILE.prn $GOLDPRN.prn $abstol $reltol $zerotol > $DASHOFILE.prn.out 2> $DASHOFILE.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.prn, see $DASHOFILE.prn.err\n";
    $retcode = 2;
}

$CMD="$fc $DASHOFILE.SENS.prn $GOLDPRN.SENS.prn $abstol $reltol $zerotol > $DASHOFILE.SENS.prn.out 2> $DASHOFILE.SENS.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.SENS.prn, see $DASHOFILE.SENS.prn.err\n";
    $retcode = 2;
}

$CMD="$XPLAT_DIFF $DASHOFILE.res $GOLDPRN.res > $DASHOFILE.res.out 2> $DASHOFILE.res.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.res, see $DASHOFILE.res.out\n";
    $retcode = 2;
}

# also check the .MEASURE, .FFT and .FOUR output for each step iteration
$namecont = "$DASHOFILE$TRANCONT";
foreach $i (0 ... $stepNum-1)
{
  $CMD="$fc $DASHOFILE.mt$i $GOLDPRN.mt$i $abstol $reltol $zerotol > $DASHOFILE.mt$i.out 2> $DASHOFILE.mt$i.err";
  if (system($CMD) != 0) {
      print STDERR "Verification failed on file $DASHOFILE.mt$i, see $DASHOFILE.mt$i.err\n";
      $retcode = 2;
  }

  $CMD="$fc $namecont.mt$i $GOLDPRN$TRANCONT.mt$i $abstol $reltol $zerotol > namecont.mt$i.out 2> $namecont.mt$i.err";
  if (system($CMD) != 0) {
      print STDERR "Verification failed on file $namecont.mt$i, see $namecont.mt$i.err\n";
      $retcode = 2;
  }

  $CMD="$fc $DASHOFILE.fft$i $GOLDPRN.fft$i $abstol $reltol $zerotol > $DASHOFILE.fft$i.out 2> $DASHOFILE.fft$i.err";
  if (system($CMD) != 0) {
      print STDERR "Verification failed on file $DASHOFILE.fft$i, see $DASHOFILE.fft$i.err\n";
      $retcode = 2;
  }

  $CMD="$cff $DASHOFILE.four$i $GOLDPRN.four$i $abstol $reltol $phaseabstol $phasereltol $zerotol > $DASHOFILE.four$i.out 2> $DASHOFILE.four$i.err";
  if (system($CMD) != 0) {
      print STDERR "Verification failed on file $DASHOFILE.four$i, see $DASHOFILE.four$i.err\n";
      $retcode = 2;
  }
}

print "Exit code = $retcode\n"; exit $retcode;
