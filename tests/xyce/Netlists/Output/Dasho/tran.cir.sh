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

$DASHOFILE="tranOutput";
$TRANCONT="_findv2";

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end

# Remove the previous output files, including some that are only made if the
# previous run failed. 
system("rm -f $CIRFILE.prn $CIRFILE.csv $CIRFILE.err $CIRFILE.out $CIRFILE.SENS*");
system("rm -f $DASHOFILE* tranGrepOutput tranFoo");

# run Xyce
$CMD="$XYCE -o $DASHOFILE -delim COMMA $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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

if ( -f "tranFoo") {
  print STDERR "Extra output file tranFoo\n";
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

if ( !(-f "$DASHOFILE.mt0") ){
  print STDERR "Missing -o output file for .MEASURE TRAN, $DASHOFILE.mt0\n";
  $xyceexit=2;
}

if ( !(-f "$DASHOFILE$TRANCONT.mt0") ){
  print STDERR "Missing -o output file for .MEASURE TRAN_CONT, $DASHOFILE$TRANCONT.mt0\n";
  $xyceexit=2;
}

if ( !(-f "$DASHOFILE.fft0") ){
  print STDERR "Missing -o output file for .FFT, $DASHOFILE.fft0\n";
  $xyceexit=2;
}

if ( !(-f "$DASHOFILE.four0") ){
  print STDERR "Missing -o output file for .FOUR, $DASHOFILE.four0\n";
  $xyceexit=2;
}

if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# Now verify the output .PRINT files.  Use file_compare.pl since I'm also
# testing print line concatenation and that the simulation footer is present.
$retcode=0;

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;
$abstol=1e-4;
$reltol=1e-3;
$zerotol=1e-6;

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

# output files for .PRINT lines should not have any commas in them
if ( system("grep ',' $DASHOFILE.prn > tranGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

if ( system("grep ',' $DASHOFILE.SENS.prn > tranGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.SENS.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

# also check the .MEASURE, .FFT and .FOUR output
print "Now testing .MEASURE, .FFT and .FOUR output\n";
$CMD="$fc $DASHOFILE.mt0 $GOLDPRN.mt0 $abstol $reltol $zerotol > $DASHOFILE.mt0.out 2> $DASHOFILE.mt0.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.mt0, see $DASHOFILE.mt0.err\n";
    $retcode = 2;
}

$namecont = "$DASHOFILE$TRANCONT";
$CMD="$fc $namecont.mt0 $GOLDPRN$TRANCONT.mt0 $abstol $reltol $zerotol > namecont.mt0.out 2> $namecont.mt0.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $namecont.mt0, see $namecont.mt0.err\n";
    $retcode = 2;
}

$CMD="$fc $DASHOFILE.fft0 $GOLDPRN.fft0 $abstol $reltol $zerotol > $DASHOFILE.fft0.out 2> $DASHOFILE.fft0.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.fft0, see $DASHOFILE.fft0.err\n";
    $retcode = 2;
}

$cff=$XYCE_VERIFY;
$cff =~ s/xyce_verify/compare_fourier_files/;
$phaseabstol = $abstol;
$phasereltol = $reltol;

$CMD="$cff $DASHOFILE.four0 $GOLDPRN.four0 $abstol $reltol $phaseabstol $phasereltol $zerotol > $DASHOFILE.four0.out 2> $DASHOFILE.four0.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.four0, see $DASHOFILE.four0.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
