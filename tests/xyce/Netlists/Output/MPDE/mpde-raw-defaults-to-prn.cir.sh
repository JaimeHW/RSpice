#!/usr/bin/env perl

use File::Basename;
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

$CIR1="mpde-raw-defaults-to-prn.cir";
$CIR2="mpde-raw-fallback-defaults-to-prn.cir";

$TMPCIRFILE1="startup_printline_for_mpde-raw-defaults-to-prn.cir";
$TMPCIRFILE2="startup_printline_for_mpde-raw-fallback-defaults-to-prn.cir";

$GOLDDIR = dirname($GOLDPRN);
$GOLD1 = "$GOLDDIR/$CIR1";
$GOLD2 = "$GOLDDIR/$CIR2";

# remove previous output files
system("rm -f $CIR1.prn $CIR1.MPDE.* $CIR1.mpde_ic.* $CIR1.startup.* $CIR1.out $CIR1.err");
system("rm -f $CIR2.prn $CIR2.MPDE.* $CIR2.mpde_ic.* $CIR2.startup.* $CIR2.out $CIR2.err");

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
if ( !(-f "$CIR1.prn")) {
    print STDERR "Missing output file $CIR1.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR1.MPDE.prn")) {
    print STDERR "Missing output file $CIR1.MPDE.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR1.mpde_ic.prn")) {
    print STDERR "Missing output file $CIR1.mpde_ic.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR1.startup.prn")) {
    print STDERR "Missing output file $CIR1.startup.prn\n";
    $xyceexit=14;
}

# check for output files from both netlists
if ( !(-f "$CIR2.MPDE.prn")) {
    print STDERR "Missing output file $CIR2.MPDE.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR2.mpde_ic.prn")) {
    print STDERR "Missing output file $CIR2.mpde_ic.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR2.startup.prn")) {
    print STDERR "Missing output file $CIR2.startup.prn\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# these strings should be in the output of a successful Xyce run for both CIR1 and CIR2
@searchstrings = (["Netlist warning: MPDE output cannot be written in PROBE, RAW or Touchstone",
    "format, using standard format instead"],
   ["Netlist warning: MPDE_IC output cannot be written in PROBE, RAW or Touchstone",
     "format, using standard format instead"],
   ["Netlist warning: MPDE_STARTUP output cannot be written in PROBE, RAW or",
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
# check for simulation footer text
# Output from Netlist 1
$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR1.MPDE.prn 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR1.MPDE.prn contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR1.MPDE.prn contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR1.mpde_ic.prn 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR1.mpde_ic.prn contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR1.mpde_ic.prn contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR1.startup.prn 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR1.startup.prn contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR1.startup.prn contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

# output for Netlist 2
$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR2.MPDE.prn 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR2.MPDE.prn contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR2.MPDE.prn contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR2.mpde_ic.prn 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR2.mpde_ic.prn contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR2.mpde_ic.prn contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR2.startup.prn 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR2.startup.prn contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR2.startup.prn contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

# check contents of MPDE-specific output files for both netlists.
print "Checking contents of output files\n";
#$CMD="$fc $CIR1.MPDE.prn $GOLD1.MPDE.prn $abstol $reltol $zerotol > $CIR1.MPDE.prn.out 2> $CIR1.MPDE.prn.err";
#if (system("$CMD") != 0) {
#    print STDERR "Verification failed on file $CIR1.MPDE.prn, see $CIR1.MPDE.prn.err\n";
#    $retcode = 2;
#}

$CMD="$XYCE_VERIFY $CIR1 $GOLD1.mpde_ic.prn $CIR1.mpde_ic.prn > $CIR1.mpde_ic.prn.out 2> $CIR1.mpde_ic.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIR1.mpde_ic.prn, see $CIR1.mpde_ic.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY $TMPCIRFILE1 $GOLD1.startup.prn $CIR1.startup.prn > $CIR1.startup.prn.out 2> $CIR1.startup.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIR1.startup.prn, see $CIR1.startup.prn.err\n";
    $retcode = 2;
}

# netlist 2

#$CMD="$fc $CIR2.MPDE.prn $GOLD2.MPDE.prn $abstol $reltol $zerotol > $CIR2.MPDE.prn.out 2> $CIR2.MPDE.prn.err";
#if (system("$CMD") != 0) {
#    print STDERR "Verification failed on file $CIR2.MPDE.prn, see $CIR2.MPDE.prn.err\n";
#    $retcode = 2;
#}

$CMD="$XYCE_VERIFY $CIR2 $GOLD2.mpde_ic.prn $CIR2.mpde_ic.prn > $CIR2.mpde_ic.prn.out 2> $CIR2.mpde_ic.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIR2.mpde_ic.prn, see $CIR2.mpde_ic.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY $TMPCIRFILE2 $GOLD2.startup.prn $CIR2.startup.prn > $CIR2.startup.prn.out 2> $CIR2.startup.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIR2.startup.prn, see $CIR2.startup.prn.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
