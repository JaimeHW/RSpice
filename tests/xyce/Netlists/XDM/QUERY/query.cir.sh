#!/usr/bin/env perl

use XyceRegression::Tools;
use XdmCommon;

#$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);

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

$Tools = XyceRegression::Tools->new();

# set Xyce XML and xdm_wrap versions
$XYCEXML=XdmCommon::getXyceXMLVersion();
$XDMWRAP=XdmCommon::getXdmWrapVersion(); 

# execute xdm queries for various options.  assume success ($retval=0)
$retval=0;

# for -q R
$CMD="$XDMWRAP -s $XYCEXML -l WARN -q R $CIRFILE > $CIRFILE.xdm.R.out";
if (system($CMD) != 0)
{
  print "XDM query for R exited with errors, or failed to run\n";
  $retval=2;
}
else
{
  # Compare against a gold standard
  $CMD="diff $CIRFILE.xdm.R.out $CIRFILE.xdm.R.gs.out";
  if (system($CMD) != 0)
  {
    print "XDM query for R failed comparison with gold file\n";
    $retval=2;
  }
  else
  {
    print "XDM query for R succeeded\n";
  }
}

# for -q C
$CMD="$XDMWRAP -s $XYCEXML -l WARN -q C $CIRFILE > $CIRFILE.xdm.C.out";
if (system($CMD) != 0)
{
  print "XDM query for C exited with errors, or failed to run\n";
  $retval=2;
}
else
{
  # Compare against a gold standard
  $CMD="diff $CIRFILE.xdm.C.out $CIRFILE.xdm.C.gs.out";
  if (system($CMD) != 0)
  {
    print "XDM query for C failed comparison with gold file\n";
    $retval=2;
  }
  else
  {
    print "XDM query for C succeeded\n";
  }
}

# for -q L
$CMD="$XDMWRAP -s $XYCEXML -l WARN -q L $CIRFILE > $CIRFILE.xdm.L.out";
if (system($CMD) != 0)
{
  print "XDM query for L exited with errors, or failed to run\n";
  $retval=2;
}
else
{
  # Compare against a gold standard
  $CMD="diff $CIRFILE.xdm.L.out $CIRFILE.xdm.L.gs.out";
  if (system($CMD) != 0)
  {
    print "XDM query for L failed comparison with gold file\n";
    $retval=2;
  }
  else
  {
    print "XDM query for L succeeded\n";
  }
}

# for -q ALL
$CMD="$XDMWRAP -s $XYCEXML -l WARN -q ALL $CIRFILE > $CIRFILE.xdm.ALL.out";
if (system($CMD) != 0)
{
  print "XDM query for ALL exited with errors, or failed to run\n";
  $retval=2;
}
else
{
  # Compare against a gold standard
  $CMD="diff $CIRFILE.xdm.ALL.out $CIRFILE.xdm.ALL.gs.out";
  if (system($CMD) != 0)
  {
    print "XDM query for ALL failed comparison with gold file\n";
    $retval=2;
  }
  else
  {
    print "XDM query for ALL succeeded\n";
  }
}

print "Exit code = $retval\n"; 
exit $retval;  





