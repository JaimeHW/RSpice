#!/usr/bin/env perl

sub dirname($) {my $file = shift; $file =~ s!/?[^/]*/*$!!; return $file; }

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

$GOLDDIR = dirname($GOLDPRN);
$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

# use file_compare rather than ACComparator since this is
# not frequency domain data
$fc=$XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;

# comparison tolerances
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-8;

# remove the previous output files
system("rm -f $CIRFILE.prn $CIRFILE.out $CIRFILE.err $CIRFILE.prn.out $CIRFILE.prn.err");
system("rm -f $CIRFILE.SENS.prn $CIRFILE.SENS.prn.out $CIRFILE.SENS.prn.err");

# run Xyce
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

if ( !(-f "$CIRFILE.prn")) {
    print STDERR "Missing output file $CIRFILE.prn\n";
    $xyceexit=14;
}

if ( !(-f "$CIRFILE.SENS.prn")) {
    print STDERR "Missing output file $CIRFILE.SENS.prn\n";
    $xyceexit=14;
}

if ( !(-f "$CIRFILE.res")) {
    print STDERR "Missing output file $CIRFILE.res\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

$retcode = 0;

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
# We have to do this special casing because this script does more than just
# one call to $XYCE_VERIFY.
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
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

$CMD="$fc $CIRFILE.prn $GOLDPRN.prn $absTol $relTol $zeroTol > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.prn, see $CIRFILE.prn.err\n";
    $retcode = 2;
}

$CMD="$fc $CIRFILE.SENS.prn $GOLDPRN.SENS.prn $absTol $relTol $zeroTol > $CIRFILE.SENS.prn.out 2> $CIRFILE.SENS.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIRFILE.SENS.prn, see $CIRFILE.SENS.prn.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
