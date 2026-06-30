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
#$CIRFILE=$ARGV[3];
$GOLDCSV=$ARGV[4];

$GOLDCSV =~ s/\.prn$//; # remove the .prn at the end.

# base file, with .PRINT SENS, for generating the known good .raw file
$BASECIR="tran-nosens.cir"; 

# these files have various .PRINT SENS lines in them.
@CIR;
$CIR[0]="tran-sens.cir";
$CIR[1]="tran-sens-multifile.cir";
$CIR[2]="tran-sens-nooverwrite.cir";

$DASHR[0]="$CIR[0].raw";
$DASHR[1]="$CIR[1].raw";
$DASHR[2]=$CIR[2];

$CSV[0]="$CIR[0].SENS.csv"; 
$CSV[1]="tran-sens-multifile.SENS.csv";
$CSV[2]="$CIR[2].SENS.csv";

# Make the .raw file for the base case.  This assumes that the tests
# in Output/TRAN work.  Also, the base netlist should not make a .SENS.csv
# file, but remove it if it was made by a previous run.
system("rm -f $BASECIR.raw* $BASECIR.err $BASECIR.out $BASECIR.prn $BASECIR.SENS.csv");

$CMD="$XYCE -r $BASECIR.raw -a $BASECIR > $BASECIR.out 2>$BASECIR.err";
$retval=system($CMD);

if ($retval != 0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n"; 
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIR[$idx]; 
    exit 13;
  }
  else
  {
    print "Exit code = 10\n"; 
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIR[$idx]; 
    exit 10;
  }

}
  
if ( -f "$BASECIR.prn" ) {
  print STDERR "Extra output file $BASECIR.prn\n";
  $xyceexit=2;
}

if ( -f "$BASECIR.SENS.csv" ) {
  print STDERR "Extra output file $BASECIR.SENS.csv\n";
  $xyceexit=2;
}

if ( !(-f "$BASECIR.raw")) {
  print STDERR "Missing output file $BASECIR.raw\n";
  $xyceexit=14;
}

if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}


# now run the netlists with .PRINT SENS lines in them
foreach $idx (0 .. 2)
{
  # Remove the previous output files.  $CIRFILE.prn file should not be made, but
  # remove it if it was made during a previous run.
  system("rm -f $CIR[$idx].raw* $CIR[$idx].err $CIR[$idx].out $CIR[$idx].prn $CSV[$idx]*");

  # run Xyce
  $CMD="$XYCE -r $CIR[$idx].raw -a $CIR[$idx] > $CIR[$idx].out 2>$CIR[$idx].err";
  $retval=system($CMD);

  if ($retval != 0)
  {
    if ($retval & 127)
    {
      print "Exit code = 13\n"; 
      printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIR[$idx]; 
      exit 13;
    }
    else
    {
      print "Exit code = 10\n"; 
      printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIR[$idx]; 
    exit 10;
    }

  }
  
  $xyceexit=0;
  if ( -f "$CIR[$idx].prn" ) {
    print STDERR "Extra output file $CIR[$idx].prn\n";
    $xyceexit=2;
  }

  if ( !(-f "$CIR[$idx].raw")) {
    print STDERR "Missing output file $CIR[$idx].raw\n";
    $xyceexit=14;
  }

  if ( !(-f "$CSV[$idx]")) {
    print STDERR "Missing output file $CSV[$idx]\n";
    $xyceexit=14;
  }

  if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}
}

# Assume that the base case works, based on tests in Output/TRAN.
# The "filtered" file does not have the Date line.  Use file_compare.pl 
# against a gold standard for the .SENS.csv files.
$retcode=0;

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;
$abstol=1e-6;
$reltol=1e-3;
$zerotol=1e-8;

if (system("grep -v 'Date:' $BASECIR.raw > $BASECIR.raw.filtered 2>$BASECIR.raw.filtered.out") != 0)
{
  print STDERR "Date line not found in file $BASECIR.raw.filtered, see $BASECIR.raw.filtered.out\n";
  $retcode = 2;
}

foreach $idx (0 .. 2)
{
  print "testing raw output for $CIR[$idx]\n";
  $compareVal = RawFileCommon::compareRawFiles($XYCE_VERIFY,$CIR[$idx],$BASECIR,$abstol,$reltol,$zerotol);
  if ($compareVal != 0)
  {
    print "Verification failed on .raw file\n";
    $retcode = 2;
  }

  $CMD="$fc $CSV[$idx] $GOLDCSV.SENS.csv $abstol $reltol $zerotol > $CSV[$idx].out 2> $CSV[$idx].err";
  if (system($CMD) != 0) {
      print STDERR "Verification failed on file $CSV[$idx], see $CSV[$idx].out\n";
      $retcode = 2;
  }
}

print "Exit code = $retcode\n"; exit $retcode;


