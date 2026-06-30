#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

# We actually run these two, and use CIR1's output as gold standard.
# CIR1 and CIR2 should NOT match.  CIR1 uses BSIM 4.6.1 and CIR2 uses 4.7
# in a way that is not backward compatible.
$CIR1="inverter_transient_xyce.cir";
$CIR2="inverter_transient_xyce_4.7.net";

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
    $PRNFILE=$cirname.".prn";
    if (not -s $PRNFILE ) 
    {
        print STDERR "$PRNFILE file missing \n"; 
        print "Exit code = 14\n"; 
        exit 14; 
    }
}

$retcode=2;
$CMD="$XYCE_VERIFY $CIR1 $CIR1.prn $CIR2.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
if (system($CMD) != 0) {
    $retcode = 0;
}
else
{
    print STDERR "Xyce produced matching results for $CIR1.prn $CIR2.prn, which is wrong.\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;


