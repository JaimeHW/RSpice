#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This test case runs two HB netlists, one with normal Newton solve, and
# one with natural parameter continuation applied to a global parameter
# during the HB solve phase.

# The test is that both runs produce the same solution (in frequency and
# time domains) at the end.

# in Xyce 6.2 and earlier, Xyce would exit with an error
# when trying to apply the continuation.  During part of the development 
# of bug 573, but after fixing it partially, the solution would be wrong
# because the DC source would not have been swept properly.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
#$GOLDPRN=$ARGV[4];  # There is no gold standard.

$CIRFILE =~ s/.cir/_hb.cir/;
$CIRFILE2 = $CIRFILE;
$CIRFILE2 =~ s/_hb/_hb_contin/;

$AC_COMPARE=$ARGV[1];
$AC_COMPARE =~ s/xyce_verify/ACComparator/;

# comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-3;  #0.1%
$zerotol=1e-8;
$freqreltol=1e-6;

# remove previous output files
system("rm -f $CIRFILE.prn* $CIRFILE.out $CIRFILE.err $CIRFILE.HB.*");
system("rm -f $CIRFILE2.prn $CIRFILE2.out $CIRFILE2.err $CIRFILE2.HB.*");

# Now run Xyce on the netlists
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
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

$CMD="$XYCE $CIRFILE2 > $CIRFILE2.out 2> $CIRFILE2.err";
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

# Now compare the two .HB.TD files with xyce_verify
$CMD="$XYCE_VERIFY $CIRFILE $CIRFILE.HB.TD.prn $CIRFILE2.HB.TD.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
if (system($CMD) != 0)
{
    print "Time domain compare failed \n";
    print "Exit code = 2\n";
    exit 2;
}

$CMD="$AC_COMPARE $CIRFILE.HB.FD.prn $CIRFILE2.HB.FD.prn $abstol $reltol $zerotol $freqreltol >> $CIRFILE.prn.out >> $CIRFILE.prn.err";
if (system($CMD) != 0)
{
    print "Frequency domain compare failed \n";
    print "Exit code = 2\n";
    exit 2;
}
else
{
    if (-z "$CIRFILE.prn.err" ) {`rm -f $CIRFILE.prn.err`;}
}

print "Exit code = 0\n";
exit 0;
