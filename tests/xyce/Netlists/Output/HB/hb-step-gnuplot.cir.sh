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
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

system("rm -f $CIRFILE.HB.TD.* $CIRFILE.HB.FD.* $CIRFILE.hb_ic.* $CIRFILE.out $CIRFILE.err");

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$retval = system($CMD);
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

if ( !(-f "$CIRFILE.HB.TD.prn")) 
{
  print STDERR "Missing output file $CIRFILE.HB.TD.prn\n";
  $xyceexit=14;
}
if ( !(-f "$CIRFILE.HB.FD.prn")) 
{
  print STDERR "Missing output file $CIRFILE.HB.FD.prn\n";
  $xyceexit=14;
}
if ( !(-f "$CIRFILE.hb_ic.prn")) 
{
  print STDERR "Missing output file $CIRFILE.hb_ic.prn\n";
  $xyceexit=14;
}

if (defined ($xyceexit)) 
{
  print "Exit code = $xyceexit\n"; 
  exit $xyceexit;
}

$retcode = 0;
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-8;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;

$CMD="$fc $CIRFILE.HB.TD.prn $GOLDPRN.HB.TD.prn $absTol $relTol $zeroTol > $CIRFILE.HB.TD.prn.out 2> $CIRFILE.HB.TD.prn.err";
if (system($CMD) != 0) 
{
  print STDERR "Verification failed on file $CIRFILE.HB.TD.prn, see $CIRFILE.HB.TD.prn.err\n";
  $retcode = 2;
}

$CMD="$fc $CIRFILE.HB.FD.prn $GOLDPRN.HB.FD.prn $absTol $relTol $zeroTol > $CIRFILE.HB.FD.prn.out 2> $CIRFILE.HB.FD.prn.err";
if (system($CMD) != 0) 
{
  print STDERR "Verification failed on file $CIRFILE.HB.FD.prn, see $CIRFILE.HB.FD.prn.err\n";
  $retcode = 2;
}

$CMD="$fc $CIRFILE.hb_ic.prn $GOLDPRN.hb_ic.prn $absTol $relTol $zeroTol > $CIRFILE.hb_ic.prn.out 2> $CIRFILE.hb_ic.prn.err";
if (system($CMD) != 0) 
{
  print STDERR "Verification failed on file $CIRFILE.hb_ic.prn, see $CIRFILE.hb_ic.prn.err\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; 
exit $retcode;
