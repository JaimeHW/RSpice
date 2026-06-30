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

# Note: the base netlist ($CIRFILE) has a .PRINT HB line in it, and produces all 
# four output files.  These following netlists have various combos of .PRINT HB_FD 
# and .PRINT HB_TD line, and should only produce the requested files.
@CIR;
$CIR[0]="hb-files-fd-td.cir"; 
$CIR[1]="hb-files-fd.cir"; 
$CIR[2]="hb-files-td.cir";

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;
# comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-3;  #0.1%
$zerotol=1e-10;
$freqreltol=1e-6;

# remove previous output files
system("rm -f $CIRFILE.HB.TD.* $CIRFILE.HB.FD.* $CIRFILE.hb_ic.* $CIRFILE.startup.* $CIRFILE.out $CIRFILE.err");
foreach $idx (0 ..2)
{
  system("rm -f $CIR[$idx].HB.TD.* $CIR[$idx].HB.FD.* $CIR[$idx].hb_ic.* $CIR[$idx].startup.* $CIR[$idx].out $CIR[$idx].err");
}

# run Xyce on the "base netlist" with fallback print lines
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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

# Run Xyce on the other netlists with various combos of .PRINT HB_FD and .PRINT HB_TD lines.
# See SON Bugs 941 and 1011 for more details.
foreach $idx (0 ..2)
{
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

# check for output files.  All four files should be made for the hb-files.cir netlist
# which has a .PRINT HB line
if ( !(-f "$CIRFILE.HB.TD.prn")) {
    print STDERR "Missing output file $CIRFILE.HB.TD.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIRFILE.HB.FD.prn")) {
    print STDERR "Missing output file $CIRFILE.HB.FD.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIRFILE.hb_ic.prn")) {
    print STDERR "Missing output file $CIRFILE.hb_ic.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIRFILE.startup.prn")) {
    print STDERR "Missing output file $CIRFILE.startup.prn\n";
    $xyceexit=14;
}

# only the hb-files-fd-td.cir.HB.TD.prn and hb-files-fd-td.cir.HB.FD.prn files should be made
# for the netlist hb-files-fd-td.cir, since it only has .PRINT HB_FD and .PRINT HB_TD lines.
if ( !(-f "$CIR[0].HB.TD.prn")) {
    print STDERR "Missing output file $CIR[0].HB.TD.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR[0].HB.FD.prn")) {
    print STDERR "Missing output file $CIR[0].HB.FD.prn\n";
    $xyceexit=14;
}
if ( (-f "$CIR[0].hb_ic.prn")) {
    print STDERR "Output file $CIR[0].hb_ic.prn made when it shouldn't be\n";
    $xyceexit=2;
}
if ( (-f "$CIR[0].startup.prn")) {
    print STDERR "Output file $CIR[0].startup.prn made when it shouldn't be\n";
    $xyceexit=2;
}

# only the hb-files-fd.cir.HB.FD.prn file should be made for the
# netlist hb-files-fd.cir, since it only has a .PRINT HB_FD line.
if ( (-f "$CIR[1].HB.TD.prn")) {
    print STDERR "Output file $CIR[1].HB.TD.prn made when it shouldn't be\n";
    $xyceexit=14;
}
if ( !(-f "$CIR[1].HB.FD.prn")) {
    print STDERR "Missing output file $CIR[1].HB.FD.prn\n";
    $xyceexit=14;
}
if ( (-f "$CIR[1].hb_ic.prn")) {
    print STDERR "Output file $CIR[1].hb_ic.prn made when it shouldn't be\n";
    $xyceexit=2;
}
if ( (-f "$CIR[1].startup.prn")) {
    print STDERR "Output file $CIR[1].startup.prn made when it shouldn't be\n";
    $xyceexit=2;
}

# only the hb-files-td.cir.HB.TD.prn file should be made for the
# netlist hb-files-td.cir, since it only has a .PRINT HB_TD line.
if ( !(-f "$CIR[2].HB.TD.prn")) {
    print STDERR "Missing output file $CIR[2].HB.TD.prn\n";
    $xyceexit=14;
}
if ( (-f "$CIR[2].HB.FD.prn")) {
    print STDERR "Output file $CIR[2].HB.FD.prn made when it shouldn't be\n";
    $xyceexit=14;
}
if ( (-f "$CIR[2].hb_ic.prn")) {
    print STDERR "Output file $CIR[2].hb_ic.prn made when it shouldn't be\n";
    $xyceexit=2;
}
if ( (-f "$CIR[2].startup.prn")) {
    print STDERR "Output file $CIR[2].startup.prn made when it shouldn't be\n";
    $xyceexit=2;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

$retcode = 0;

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE $GOLDRAW $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
    {
        print "Exit code = 2 \n";
        exit 2;
    }
    else
    {
        print "Exit code = 0 \n";
        exit 0;
    }
}

# check all four output files from the hb-files.cir netlist, which has a .PRINT HB line
$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN.HB.TD.prn $CIRFILE.HB.TD.prn > $CIRFILE.HB.TD.prn.out 2> $CIRFILE.HB.TD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIRFILE.HB.TD.prn, see $CIRFILE.HB.TD.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_ACVERIFY $GOLDPRN.HB.FD.prn $CIRFILE.HB.FD.prn $abstol $reltol $zerotol $freqreltol > $CIRFILE.HB.FD.prn.out 2> $CIRFILE.HB.FD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIRFILE.HB.FD.prn, see $CIRFILE.HB.FD.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN.hb_ic.prn $CIRFILE.hb_ic.prn > $CIRFILE.hb_ic.prn.out 2> $CIRFILE.hb_ic.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIRFILE.hb_ic.prn, see $CIRFILE.hb_ic.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN.startup.prn $CIRFILE.startup.prn > $CIRFILE.startup.prn.out 2> $CIRFILE.startup.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIRFILE.startup.prn, see $CIRFILE.startup.prn.err\n";
    $retcode = 2;
}

# <netlistname>.HB.TD.prn files should be identical for the base netlist and the 
# hb-files-fd-td.cir netlist.  However, they sometimes differ slightly (to within the
# machine precision) when run in parallel.  So, use xyce_verify rather than diff here.
$CMD="$XYCE_VERIFY $CIRFILE $CIRFILE.HB.TD.prn $CIR[0].HB.TD.prn > $CIR[0].HB.TD.prn.out 2> $CIR[0].HB.TD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR[0].HB.TD.prn, see $CIR[0].HB.TD.prn.out\n";
    $retcode = 2;
}

# <netlistname>.HB.FD.prn files should be identical for the base netlist and 
# the hb-files-fd-td.cir netlist
$CMD="diff $CIRFILE.HB.FD.prn $CIR[0].HB.FD.prn > $CIR[0].HB.FD.prn.out 2> $CIR[0].HB.FD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR[0].HB.FD.prn, see $CIR[0].HB.FD.prn.out\n";
    $retcode = 2;
}

# <netlistname>.HB.FD.prn files should be identical for the base netlist and 
# the hb-files-fd.cir netlist
$CMD="diff $CIRFILE.HB.FD.prn $CIR[1].HB.FD.prn > $CIR[1].HB.FD.prn.out 2> $CIR[1].HB.FD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR[1].HB.FD.prn, see $CIR[1].HB.FD.prn.out\n";
    $retcode = 2;
}

# <netlistname>.HB.TD.prn files should be identical for the base netlist and 
# the hb-files-td.cir netlist.  However, they sometimes differ slightly (to within the
# machine precision) when run in parallel.  So, use xyce_verify rather than diff here.
$CMD="$XYCE_VERIFY $CIRFILE $CIRFILE.HB.TD.prn $CIR[2].HB.TD.prn > $CIR[2].HB.TD.prn.out 2> $CIR[2].HB.TD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR[2].HB.TD.prn, see $CIR[2].HB.TD.prn.out\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
