#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
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
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;

# comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-4;  #0.01%
$zerotol=1e-11;
$freqreltol=1e-6;

# remove previous output files
system("rm -f $CIRFILE.out $CIRFILE.err $CIRFILE.HB.* $CIRFILE.hb_ic.* $CIRFILE.startup.*");

#run Xyce
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

# check for output files
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

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE $GOLDCSV $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
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

# verify output files
$retcode = 0;
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

$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN.startup.prn $CIRFILE.startup.prn > $CIRFILE.startup.prn.out 2> $CIRFILE.startup.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIRFILE.startup.prn, see $CIRFILE.startup.prn.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
