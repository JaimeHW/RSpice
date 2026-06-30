#!/usr/bin/env perl

use XyceRegression::Tools;
use XdmCommon;

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

# for -q Q
$CMD="$XDMWRAP -s $XYCEXML -l WARN -q Q $CIRFILE > $CIRFILE.xdm.Q.out";
if (system($CMD) != 0)
{
  print "XDM query for Q exited with errors, or failed to run\n";
  $retval=2;
}
else
{
  # Compare against a gold standard
  $CMD="diff $CIRFILE.xdm.Q.out $CIRFILE.xdm.Q.gs.out";
  if (system($CMD) != 0)
  {
    print "XDM query for Q failed comparison with gold file\n";
    $retval=2;
  }
  else
  {
    print "XDM query for Q succeeded\n";
  }
}

print "Exit code = $retval\n"; 
exit $retval;  
