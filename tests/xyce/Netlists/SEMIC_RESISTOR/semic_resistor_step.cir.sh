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

# Comparison tolerances
$absTol = 1.0e-5;
$relTol = 1.0e-3;
$zeroTol = 1.0e-10;

# remove old files if they exist
`rm -f $CIRFILE.prn $CIRFILE.out $CIRFILE.err`;
`rm -f $CIRFILE.prn.out $CIRFILE.prn.err`;

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

$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
#print "$fc $PRNFILE $GOLDPRN $absTol $relTol $zeroTol\n";
$CMD="$fc $PRNFILE $GOLDPRN $absTol $relTol $zeroTol > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
$retval=system($CMD);
if ( $retval != 0 )
{
  print "test Failed comparison of .prn files\n";
  print "Exit code = 2";
  exit 2;
}
else
{
  print "test passed!\n";
  print "Exit code = 0\n";
  exit 0;
}



