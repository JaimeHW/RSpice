#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];

$CIRFILE="nand8.cir";
$OUTFILE="nand8.cir.out";
$NAMESFILE="namesOnly.txt";

$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
# check to make sure Xyce crashed (as it should for this bug)
$retcode = 0;
if ($retval != 13) 
{ 
  print "Exit code = $retval\n"; 
  $retcode = 1;
  exit $retcode;
}

# now use grep to look for the phrase "Transient failure history":
if (system("grep -A 27 \"Transient failure history:\" $OUTFILE | grep -v history | grep -v node | grep -v name > $OUTFILE.history") )
{
  print "Exit code = 2\n";
  exit 2;
}
else
{
  if (system("grep -f $NAMESFILE -i $OUTFILE.history > finalGrepResult"))
  {
    print "Exit code = 2\n";
    exit 2;
  }
  else
  {
    system("wc -l finalGrepResult > numberOfLines");
    if (system("grep 25  numberOfLines"))
    {
      print "Exit code = 2\n";
      exit 2;
    }
  }
}


print "Exit code = 0\n";
exit 0;

