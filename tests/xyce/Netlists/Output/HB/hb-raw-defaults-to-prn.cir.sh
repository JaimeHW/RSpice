#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

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
#$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$CIR1="hb-raw-defaults-to-prn.cir";
$CIR2="hb-raw-fallback-defaults-to-prn.cir";

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;

# comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-3;  #0.1%
$zerotol=1e-10;
$freqreltol=1e-6;

# remove previous output files
system("rm -f $CIR1.HB.TD.* $CIR1.HB.FD.* $CIR1.hb_ic.* $CIR1.startup.* $CIR1.out $CIR1.err");
system("rm -f $CIR2.HB.TD.* $CIR2.HB.FD.* $CIR2.hb_ic.* $CIR2.startup.* $CIR2.out $CIR2.err");

# run Xyce on both netlists
$CMD="$XYCE $CIR1 > $CIR1.out 2>$CIR1.err";
$retval=system($CMD);

if ($retval != 0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n"; 
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIR1; 
    exit 13;
  }
  else
  {
    print "Exit code = 10\n"; 
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIR1; 
    exit 10;
  }
}

$CMD="$XYCE $CIR2 > $CIR2.out 2>$CIR2.err";
$retval=system($CMD);

if ($retval != 0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n"; 
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIR2; 
    exit 13;
  }
  else
  {
    print "Exit code = 10\n"; 
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIR2; 
    exit 10;
  }
}

# check for output files from both netlists
if ( !(-f "$CIR1.HB.TD.prn")) {
    print STDERR "Missing output file $CIR1.HB.TD.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR1.HB.FD.prn")) {
    print STDERR "Missing output file $CIR1.HB.FD.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR1.hb_ic.prn")) {
    print STDERR "Missing output file $CIR1.hb_ic.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR1.startup.prn")) {
    print STDERR "Missing output file $CIR1.startup.prn\n";
    $xyceexit=14;
}

if ( !(-f "$CIR2.HB.TD.prn")) {
    print STDERR "Missing output file $CIR2.HB.TD.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR2.HB.FD.prn")) {
    print STDERR "Missing output file $CIR2.HB.FD.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR2.hb_ic.prn")) {
    print STDERR "Missing output file $CIR2.hb_ic.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR2.startup.prn")) {
    print STDERR "Missing output file $CIR2.startup.prn\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# these strings should be in the output of a successful Xyce run for both CIR1 and CIR2
@searchstrings = (["Netlist warning: HB_FD output cannot be written in PROBE, RAW or Touchstone",
                   "format, using standard format instead"],
   ["Netlist warning: HB_TD output cannot be written in PROBE, RAW or Touchstone",
     "format, using standard format instead"],
   ["Netlist warning: HB_IC output cannot be written in PROBE, RAW or Touchstone",
    "format, using standard format instead"],
   ["Netlist warning: HB_STARTUP output cannot be written in PROBE, RAW or",
    "Touchstone format, using standard format instead"]
);

$retval = $Tools->checkGroupedError("$CIR1.out",@searchstrings);
if ($retval != 0)
{
  print "Check on warning message failed for $CIR1\n";
  print "Exit code = $retval\n"; exit $retval; 
} 

$retval = $Tools->checkGroupedError("$CIR2.out",@searchstrings);
if ($retval != 0)
{
  print "Check on warning message failed for $CIR2\n";
  print "Exit code = $retval\n"; exit $retval; 
} 


$retcode = 0;
# Test the outputs from $CIR1, which has explicit .PRINT lines for each format type
$CMD="$XYCE_VERIFY $CIR1 $GOLDPRN.HB.TD.prn $CIR1.HB.TD.prn > $CIR1.HB.TD.prn.out 2> $CIR1.HB.TD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR1.HB.TD.prn, see $CIR1.HB.TD.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_ACVERIFY $GOLDPRN.HB.FD.prn $CIR1.HB.FD.prn $abstol $reltol $zerotol $freqreltol > $CIR1.HB.FD.prn.out 2> $CIR1.HB.FD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR1.HB.FD.prn, see $CIR1.HB.FD.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY $CIR1 $GOLDPRN.hb_ic.prn $CIR1.hb_ic.prn > $CIR1.hb_ic.prn.out 2> $CIR1.hb_ic.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR1.hb_ic.prn, see $CIR1.hb_ic.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY $CIR1 $GOLDPRN.startup.prn $CIR1.startup.prn > $CIR1.startup.prn.out 2> $CIR1.startup.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR1.startup.prn, see $CIR1.startup.prn.err\n";
    $retcode = 2;
}

# Test the outputs from $CIR2, which only has a .PRINT HB line.  The $CIR1 and $CIR2 outputs
# should be identical
$CMD="diff $CIR1.HB.TD.prn $CIR2.HB.TD.prn > $CIR2.HB.TD.prn.out 2> $CIR2.HB.TD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR2.HB.TD.prn, see $CIR2.HB.TD.prn.out\n";
    $retcode = 2;
}

$CMD="diff $CIR1.HB.FD.prn $CIR2.HB.FD.prn > $CIR2.HB.FD.prn.out 2> $CIR2.HB.FD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR2.HB.FD.prn, see $CIR2.HB.FD.prn.out\n";
    $retcode = 2;
}

$CMD="diff $CIR1.hb_ic.prn $CIR2.hb_ic.prn > $CIR2.hc_ib.prn.out 2> $CIR2.hb_ic.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR2.hb_ic.prn, see $CIR2.hb_ic.prn.out\n";
    $retcode = 2;
}

$CMD="diff $CIR1.startup.prn $CIR2.startup.prn > $CIR2.startup.prn.out 2> $CIR2.startup.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR2.startup.prn, see $CIR2.startup.prn.out\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
