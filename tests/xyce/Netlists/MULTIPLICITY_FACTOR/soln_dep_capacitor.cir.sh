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
$GOLDPRN=$ARGV[4];

$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;

# Comparison tolerances
$absTol = 1.0e-5;
$relTol = 1.0e-3;
$zeroTol = 1.0e-10;

# remove previous output files
system("rm -f $CIRFILE.out $CIRFILE.err $CIRFILE.prn $CIRFILE.errmsg.*");

# run .cir file, and check that it worked
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
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

$PRNFILE="$CIRFILE.prn";
if (not -s $PRNFILE ) 
{
  print STDERR ".prn file missing \n"; 
  print "Exit code = 14\n"; 
  exit 14; 
}

$retcode=0;
#print "$fc $PRNFILE $GOLDPRN $absTol $relTol $zeroTol\n";
$CMD="$fc $PRNFILE $GOLDPRN $absTol $relTol $zeroTol > $CIRFILE.errmsg.out 2> $CIRFILE.errmsg.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $CIRFILE.prn, see $CIRFILE.errmsg.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;



