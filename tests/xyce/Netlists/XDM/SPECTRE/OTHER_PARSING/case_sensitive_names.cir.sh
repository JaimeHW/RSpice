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

use Getopt::Long;
&GetOptions( "verbose!" => \$verbose );
$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
#$GOLDPRN=$ARGV[4];

# Comparison tolerances
$absTol = 1.0e-5;
$relTol = 1.0e-3;
$zeroTol = 1.0e-10;

# Note: must escape [ ] ( ) and * with \\ to get the CheckError() function to work.
# Do not have to escape { } or _.
@xyceSearchstrings = ("Netlist error in file case_sensitive_names.cir at or near line 19",
                      "Duplicate device RR0");
$translatedXyceNetlistSearchStringsPtr=\@xyceSearchstrings;

@xdmSearchstrings = ("xdm Detected multiple devices with the same case-insensitive names: \\['RR0', 'Rr0'\\]",
  "Total critical issues reported 			 = 0:", 
  "Total          errors reported 			 = 0:", 
  "Total          warnings reported 			 = 14:", 
  "Total          information messages reported 	 = 1:", 
  "SUCCESS: xdm completion status flag = 0:"     
);
$xdmSearchStringsPtr=\@xdmSearchstrings;

XdmCommon::translateAndCheckXyceMessages($XYCE,$XYCE_VERIFY,$CIRFILE,"spectre",$verbose,
                                         "ERROR",$translatedXyceNetlistSearchStringsPtr,
                                          $xdmSearchStringsPtr);
