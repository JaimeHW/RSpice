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
$CIRFILE=$ARGV[3];

$CIRFILE1=$CIRFILE;
$CIRFILE2=$CIRFILE;
$CIRFILE1 =~ s/.cir/_vsrc.cir/;
$CIRFILE2 =~ s/.cir/_expr.cir/;

$retval=$Tools->wrapXyce($XYCE,$CIRFILE1);
if ($retval != 0) 
{ 
    print "Xyce failed on $CIRFILE1\n"; 
    print "Exit code = $retval\n"; 
    exit $retval; 
}

$retval=$Tools->wrapXyce($XYCE,$CIRFILE2);
if ($retval != 0) 
{ 
    print "Xyce failed on $CIRFILE2\n"; 
    print "Exit code = $retval\n"; 
    exit $retval; 
}


if (-f "$CIRFILE1.prn" && -f "$CIRFILE2.prn")
{
  $CMD="$XYCE_VERIFY $CIRFILE1 $CIRFILE1.prn $CIRFILE2.prn  > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
  $retval = system("$CMD");

  if ($retval == 0) 
  { 
      $retcode = 0; 
  }
  else 
  { 
      $retcode = 2; 
  }
}
else 
{ 
    $retcode = 14; 
}

print "Exit code = $retcode\n";
exit $retcode;

