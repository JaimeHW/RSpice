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

# for -q D
$CMD="$XDMWRAP -s $XYCEXML -l WARN -q D $CIRFILE > $CIRFILE.xdm.D.out";
if (system($CMD) != 0)
{
  print "XDM query for D exited with errors, or failed to run\n";
  $retval=2;
}
else
{
  # need to compare against a gold standard, but just get the
  # query to work from within a perl script for now.
  $CMD="diff $CIRFILE.xdm.D.out $CIRFILE.xdm.D.gs.out";
  if (system($CMD) != 0)
  {
    print "XDM query for D failed comparison with gold file\n";
    $retval=2;
  }
  else
  {
    print "XDM query for D succeeded\n";
  }
}

print "Exit code = $retval\n"; 
exit $retval;  
