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

$DASHOFILE="esOutput";
$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

# Remove the previous output files, including some that are only made if the
# previous run failed.
system("rm -f $CIRFILE.prn* $CIRFILE.err $CIRFILE.out $CIRFILE.ES.*");
system("rm -f $DASHOFILE* esGrepOutput esFoo");

# run Xyce
$CMD="$XYCE -o $DASHOFILE.ES.prn -delim COMMA $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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
if ( (-f "$CIRFILE.prn") || (-f "$CIRFILE.ES.csv") ){
  print STDERR "Extra output file $CIRFILE.prn or $CIRFILE.ES.csv\n";
  $xyceexit=2;
}

if ( -f "esFoo") {
  print STDERR "Extra output file esFoo\n";
  $xyceexit=2;
}

if ( !(-f "$DASHOFILE.ES.prn") ){
  print STDERR "Missing -o output file for .PRINT ES, $DASHOFILE.ES.prn\n";
  $xyceexit=14;
}

if ( !(-f "$DASHOFILE.prn") ){
  print STDERR "Missing -o output file for .PRINT DC, $DASHOFILE.prn\n";
  $xyceexit=14;
}

if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# Now verify the output files
$retcode = 0;
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-6;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;

$CMD="$fc $DASHOFILE.ES.prn $GOLDPRN.ES.prn $absTol $relTol $zeroTol > $DASHOFILE.ES.prn.out 2> $DASHOFILE.ES.prn.err";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $DASHOFILE.ES.prn\n";
  $retcode = 2;
}

$CMD="$fc $DASHOFILE.prn $GOLDPRN.prn $absTol $relTol $zeroTol > $DASHOFILE.prn.out 2> $DASHOFILE.prn.err";
$retval = system($CMD);
$retval = $retval >> 8;
if ($retval != 0){
  print STDERR "Comparator exited with exit code $retval on file $DASHOFILE.prn\n";
  $retcode = 2;
}

# output files should not have any commas in them
if ( system("grep ',' esOutput.ES.prn > esGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.ES.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

if ( system("grep ',' esOutput.prn > esGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
