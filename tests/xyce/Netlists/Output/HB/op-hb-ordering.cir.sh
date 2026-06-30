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
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDCSV=$ARGV[4];

# these files have a .OP statement (before their .HB statements)
# with the various .PRINT HB line types in them.
@CIR;
$CIR[0]="op-hb-ordering.cir";           # .PRINT HB
$CIR[1]="op-hb_fd-ordering.cir";        # .PRINT HB_FD
$CIR[2]="op-hb_td-ordering.cir";        # .PRINT HB_TD
$CIR[3]="op-hb_ic-ordering.cir";        # .PRINT HB_IC
$CIR[4]="op-hb_startup-ordering.cir";   # .PRINT HB_STARTUP

# now run the netlists
foreach $idx (0 .. 4)
{
  # Remove the previous output files. 
  system("rm -f $CIR[$idx].out $CIR[$idx].err");
  system("rm -f $CIR[$idx].HB.FD.prn $CIR[$idx].HB.TD.prn $CIR[$idx].hb_ic.prn $CIR[$idx].startup.prn");

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
}

# check for the presence of (but not the contents of) the requested output files  
$retcode=0;
if ( !(-f "$CIR[0].HB.FD.prn")) {
  print STDERR "Missing output file $CIR[0].HB.FD.prn\n";
    $retcode=14;
}

if ( !(-f "$CIR[0].HB.TD.prn")) {
  print STDERR "Missing output file $CIR[0].HB.TD.prn\n";
  $retcode=14;
}

if ( !(-f "$CIR[0].hb_ic.prn")) {
  print STDERR "Missing output file $CIR[0].hb_ic.prn\n";
  $retcode=14;
}

if ( !(-f "$CIR[0].startup.prn")) {
  print STDERR "Missing output file $CIR[0].startup.prn\n";
  $retcode=14;
}

if ( !(-f "$CIR[1].HB.FD.prn")) {
  print STDERR "Missing output file $CIR[1].HB.FD.prn\n";
    $retcode=14;
}

if ( !(-f "$CIR[2].HB.TD.prn")) {
  print STDERR "Missing output file $CIR[2].HB.TD.prn\n";
  $retcode=14;
}

if ( !(-f "$CIR[3].hb_ic.prn")) {
  print STDERR "Missing output file $CIR[3].hb_ic.prn\n";
  $retcode=14;
}

if ( !(-f "$CIR[4].startup.prn")) {
  print STDERR "Missing output file $CIR[4].startup.prn\n";
  $retcode=14;
}

print "Exit code = $retcode\n"; exit $retcode;


