#!/usr/bin/env perl

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
$XYCE_VERIFY=~ s/xyce_verify/ACComparator/;
$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$CIR1="RC_simple_baseline.cir";
$CIR2="RC_simple_data.cir";

# comparison tolerances for ACComparator.pl
$abstol=6e-5;
$reltol=1e-4;  
$zerotol=1e-6;
$freqreltol=1e-6;

# remove previous output files
system("rm -f $CIR1.FD.prn $CIR1.out $CIR1.err");
system("rm -f $CIR2.FD.prn $CIR2.out $CIR2.err");

# run Xyce on both netlists
$CMD="$XYCE $CIR1 > $CIR1.out 2> $CIR1.err";
$retval = system($CMD);

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

$CMD="$XYCE $CIR2 > $CIR2.out 2> $CIR2.err";
$retval = system($CMD);

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

$retcode=0;

# do a "diff" comparison first
$CMD="diff $CIR1.FD.prn $CIR2.FD.prn > $CIR1.FD.prn.out 2> $CIR1.FD.prn.err";
$retval = system($CMD);
if ($retval != 0)
{
  # if "diff" fails, then do a Xyce verify comparison
  $CMD="$XYCE_VERIFY --gsformat=xyceprn $CIR1.FD.prn $CIR2.FD.prn $abstol $reltol $zerotol $freqreltol";
  $retval = system($CMD);
  $retval = $retval >> 8;
  if ($retval == 0) 
  { 
    $retcode = 0; 
  }
  else 
  {
    print STDERR "Comparator exited with exit code $retval\n"; 
    $retcode = 2; 
  }
}

print "Exit code = $retcode\n"; exit $retcode;

