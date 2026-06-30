#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_VERIFY=~ s/xyce_verify/ACComparator/;
$CIRFILE=$ARGV[3];

# comparison tolerances for ACComparator.pl
$abstol=1e-8;
$reltol=1e-3;  #0.1%
$zerotol=1e-14;
$freqreltol=1e-6;


# We actually run these two, and use CIR1's output as gold standard.
# CIR1 puts six bjts in parallel, CIR2 uses M=6
$CIR1="bsim_4p61_noise2_baseline.net";
$CIR2="bsim_4p61_noise2_m6.net";
# remove previous output files
system("rm -f $CIRFILE.out $CIRFILE.err $CIRFILE.prn");

# run .cir file, and check that it worked
$CMD="$XYCE $CIR1 > $CIRFILE.out 2> $CIRFILE.err";
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

$CMD="$XYCE $CIR2 >> $CIRFILE.out 2>> $CIRFILE.err";
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

@cirlist=($CIR1, $CIR2);
foreach $cirname (@cirlist)
{
    $PRNFILE=$cirname.".NOISE.prn";
    if (not -s $PRNFILE ) 
    {
        print STDERR "$PRNFILE file missing \n"; 
        print "Exit code = 14\n"; 
        exit 14; 
    }
}

$retcode=0;
$CMD="$XYCE_VERIFY $CIR1.NOISE.prn $CIR2.NOISE.prn  $abstol $reltol $zerotol $freqreltol > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIR2.prn, see $CIRFILE.prn.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;


