#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

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
# expects the STDERR output from Xyce to go into $CIRFILE.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

system("rm -f $CIRFILE.prn* $CIRFILE.ts2* $CIRFILE.err $CIRFILE.out");

$CMD="$XYCE -quiet $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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
        printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE; exit 10;
    }
}

if ( !(-f "$CIRFILE.prn")) {
    print STDERR "Missing output file $CIRFILE.prn\n";
    $xyceexit=14;
}

if ( !(-f "$CIRFILE.ts2")) {
    print STDERR "Missing output file $CIRFILE.ts2\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# these strings should be in the output of this successful Xyce run
@searchstrings = ("Netlist warning: Transient output cannot be written in Touchstone format,",
                  "using standard format",
                  "Netlist warning: Transient output cannot be written in Touchstone format,",
                  "using standard format"
);

$retval = $Tools->checkError("$CIRFILE.out",@searchstrings);
if ($retval != 0)
{
  print "Check on warning message failed\n";
  print "Exit code = $retval\n"; exit $retval;
}

$retcode = 0;

$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN.prn $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.prn, see $CIRFILE.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN.ts2 $CIRFILE.ts2 > $CIRFILE.ts2.out 2> $CIRFILE.ts2.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.ts2, see $CIRFILE.ts2.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
