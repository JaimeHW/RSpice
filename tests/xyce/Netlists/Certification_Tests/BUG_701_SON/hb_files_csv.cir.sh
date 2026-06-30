#!/usr/bin/env perl

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
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;

$CONVERT_TO_PRN=$XYCE_VERIFY;
$CONVERT_TO_PRN =~ s/xyce_verify.pl/convertToPrn2.py/;

# comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-3;  #0.1%
$zerotol=1e-8;
$freqreltol=1e-6;

# these files have various .PRINT AC and .PRINT AC_IC lines in them
@CIR;
@FD_OUT;
@TD_OUT;
@IC_OUT;
@ST_OUT;
$CIR[0]="hb_files_csv.cir";
$FD_OUT[0]="hb_bleem_fd_csv";
$TD_OUT[0]="hb_bleem_td_csv";
$IC_OUT[0]="hb_bleem_ic_csv";
$ST_OUT[0]="hb_bleem_startup_csv";

$CIR[1]="hb_files_fallback_csv.cir";
$FD_OUT[1]="hb_foo_csv.HB.FD.csv";
$TD_OUT[1]="hb_foo_csv.HB.TD.csv";
$IC_OUT[1]="hb_foo_csv.hb_ic.csv";
$ST_OUT[1]="hb_foo_csv.startup.csv";

# run the netlists
foreach $idx (0 .. 1)
{
  # Remove the previous output files.  $CIRFILE.prn file should not be made, but
  # remove it if it was made during a previous run.
  system("rm -f $CIR[$idx].err $CIR[$idx].out $FD_OUT[$idx] $TD_OUT[$idx] $FD_OUT[$idx].* $TD_OUT[$idx].*");
  system("rm -f $IC_OUT[$idx] $ST_OUT[$idx] $IC_OUT[$idx].* $ST_OUT[$idx].*");
  system("rm -f $IC_OUT[$idx]_converted.prn $ST_OUT[$idx]_converted.prn $TD_OUT[$idx]_converted.prn");

  # run Xyce
  $CMD="$XYCE $CIR[$idx] > $CIR[$idx].out 2>$CIR[$idx].err";
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
  if ( !(-f "$FD_OUT[$idx]")) {
    print STDERR "Missing output file $FD_OUT[$idx]\n";
    $xyceexit=14;
  }

  if ( !(-f "$TD_OUT[$idx]")) {
    print STDERR "Missing output file $TD_OUT[$idx]\n";
    $xyceexit=14;
  }

  if ( !(-f "$IC_OUT[$idx]")) {
    print STDERR "Missing output file $IC_OUT[$idx]\n";
    $xyceexit=14;
  }

  if ( !(-f "$ST_OUT[$idx]")) {
    print STDERR "Missing output file $ST_OUT[$idx]\n";
    $xyceexit=14;
  }

  if ( ($idx== 0) && (-f "hb_bar")) {
    print STDERR "Extra output file hb_bar made\n";
    $xyceexit=14;
  }

  if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}
}


$retcode=0;
foreach $idx (0 .. 1)
{
  $xyceexit = 0;
  # convert the various .csv files, excluding the FD one
  if (system("$CONVERT_TO_PRN $TD_OUT[$idx]") != 0){
    print STDERR "Failed to convert $TD_OUT[$idx] to prn\n";
    $xyceexit = 10;
  }

  if (system("$CONVERT_TO_PRN $ST_OUT[$idx]") != 0){
    print STDERR "Failed to convert $ST_OUT[$idx] to prn\n";
    $xyceexit = 10;
  }

  if (system("$CONVERT_TO_PRN $IC_OUT[$idx]") != 0){
    print STDERR "Failed to convert $IC_OUT[$idx] to prn\n";
    $xyceexit = 10;
  }

  if ($xyceexit !=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

  # check for absence of simulation footer text (for CSV format), excluding the FD one
  $xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $TD_OUT[$idx] 2>/dev/null`;
  if ($xyceEndCount == 0)
  {
    printf "Output file $TD_OUT[$idx] contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  }
  else
  {
    printf "Output file $TD_OUT[$idx] contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
    $retcode = 2;
  }

  $xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $IC_OUT[$idx] 2>/dev/null`;
  if ($xyceEndCount == 0)
  {
    printf "Output file $IC_OUT[$idx] contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  }
  else
  {
    printf "Output file $IC_OUT[$idx] contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
    $retcode = 2;
  }

  $xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $ST_OUT[$idx] 2>/dev/null`;
  if ($xyceEndCount == 0)
  {
    printf "Output file $ST_OUT[$idx] contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  }
  else
  {
    printf "Output file $ST_OUT[$idx] contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
    $retcode = 2;
  }

  # now check the file contents, for all four output files
  $CMD="$XYCE_VERIFY $CIR[$idx] $GOLDCSV.TD.prn $TD_OUT[$idx]_converted.prn > $TD_OUT[$idx].out 2> $TD_OUT[$idx].err";
  if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $TD_OUT[$idx], see $TD_OUT[$idx].err\n";
    $retcode = 2;
  }

  $CMD="$XYCE_ACVERIFY --gsformat=xycecsv $GOLDCSV.FD.csv $FD_OUT[$idx] $abstol $reltol $zerotol $freqreltol > $FD_OUT[$idx].out 2> $FD_OUT[$idx].err";
  $retval = system($CMD);
  $retval = $retval >> 8;
  if ($retval != 0) 
  { 
    print STDERR "Verification failed on file $FD_OUT[$idx], see $FD_OUT[$idx].err\n";
    $retcode = 2; 
  }

  $CMD="$XYCE_VERIFY $CIR[$idx] $GOLDCSV.hb_ic.prn $IC_OUT[$idx]_converted.prn > $IC_OUT[$idx].out 2> $IC_OUT[$idx].err";
  if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $IC_OUT[$idx], see $IC_OUT[$idx].err\n";
    $retcode = 2;
  }

  $CMD="$XYCE_VERIFY $CIR[$idx] $GOLDCSV.startup.prn $ST_OUT[$idx]_converted.prn > $ST_OUT[$idx].out 2> $ST_OUT[$idx].err";
  if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $ST_OUT[$idx], see $ST_OUT[$idx].err\n";
    $retcode = 2;
  }
}

print "Exit code = $retcode\n"; exit $retcode;


