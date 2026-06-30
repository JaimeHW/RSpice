#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

@XyceCMD;
$XyceCMD[0]="$XYCE -l $CIRFILE.out0 -hspice-ext bleem";
$XyceCMD[1]="$XYCE -l $CIRFILE.out1 -hspice-ext ,all";
$XyceCMD[2]="$XYCE -l $CIRFILE.out2 -hspice-ext units,,math";
$XyceCMD[3]="$XYCE -hspice-ext -l $CIRFILE.out3";

# check various error cases
# these strings should be in the output of the various failed Xyce runs
@searchstrings0 = ("Invalid value bleem for -hspice-ext command line option");
@searchstrings1 = ("Invalid value  for -hspice-ext command line option");
@searchstrings2 = ("Invalid value  for -hspice-ext command line option");
@searchstrings3 = ("Expected option value, but found option -l");

$retcode = 0;
# remove files if they exist
system("rm -f $CIRFILE.out0");
$retval=$Tools->wrapXyce($XyceCMD[0],$CIRFILE);
if ($retval == 0)
{
  print "$XyceCMD[0] ran when it should not\n";
  $retcode=$retval;
}
$retval = $Tools->checkError("$CIRFILE.out0",@searchstrings0);
if ($retval != 0)
{
  $retcode = $retval;
}

system("rm -f $CIRFILE.out1");
$retval=$Tools->wrapXyce($XyceCMD[1],$CIRFILE);
if ($retval == 0)
{
  print "$XyceCMD[1] ran when it should not\n";
  $retcode=$retval;
}
$retval = $Tools->checkError("$CIRFILE.out1",@searchstrings1);
if ($retval != 0)
{
  $retcode = $retval;
}

system("rm -f $CIRFILE.out2");
$retval=$Tools->wrapXyce($XyceCMD[2],$CIRFILE);
if ($retval == 0)
{
  print "$XyceCMD[2] ran when it should not\n";
  $retcode=$retval;
}
$retval = $Tools->checkError("$CIRFILE.out2",@searchstrings2);
if ($retval != 0)
{
  $retcode = $retval;
}

# for this run, the output will be in $CIRFILE.out
system("rm -f $CIRFILE.out");
$retval=$Tools->wrapXyce($XyceCMD[3],$CIRFILE);
if ($retval == 0)
{
  print "$XyceCMD[3] ran when it should not\n";
  $retcode=$retval;
}
$retval = $Tools->checkError("$CIRFILE.out",@searchstrings3);
if ($retval != 0)
{
  $retcode = $retval;
}

print "Exit code = $retcode\n"; exit $retcode;

