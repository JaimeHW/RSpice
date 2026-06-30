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
#$GOLDRAW=$ARGV[4];

@CIR;
$CIR[0]="tran.cir";
$CIR[1]="tran-multifile.cir";
$CIR[2]="tran-nooverwrite.cir";

$DASHR[0]="$CIR[0].raw";
$DASHR[1]="$CIR[1].raw";
$DASHR[2]=$CIR[2];

foreach $idx (0 .. 2)
{
  # Remove the previous output files.  $CIRFILE.prn file should not be made, but
  # remove it if it was made during a previous run.
  system("rm -f $CIR[$idx].raw* $CIR[$idx].err $CIR[$idx].out $CIR[$idx].prn");

  # run Xyce
  $CMD="$XYCE -r $DASHR[$idx] -a $CIR[$idx] > $CIR[$idx].out 2>$CIR[$idx].err";
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

  if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}
}

# Assume that the base case works, based on tests in Output/TRAN.
# The "filtered" files do not have their Date lines.
$retcode=0;

$abstol=1e-6;
$reltol=1e-3;
$zerotol=1e-8;

if (system("grep -v 'Date:' $CIR[0].raw > $CIR[0].raw.filtered 2>$CIR[0].raw.filtered.out") != 0)
{
  print STDERR "Date line not found in file $CIR[0].raw.filtered, see $CIR[0].raw.filtered.out\n";
  $retcode = 2;
}

foreach $idx (1 .. 2)
{
  print "testing raw output for $CIR[$idx]\n";
  $compareVal = RawFileCommon::compareRawFiles($XYCE_VERIFY,$CIR[$idx],$CIR[0],$abstol,$reltol,$zerotol);
  if ($compareVal != 0)
  {
    print "Verification failed on .raw file\n";
    $retcode = 2;
  }
}

print "Exit code = $retcode\n"; exit $retcode;


