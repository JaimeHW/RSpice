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

# Note: must escape ( ) and * with \\ to get the CheckError() function to work.
# Do not have to escape { } or _.
# Strings to test for in the diagnostic output from xdm.
@searchStrings = ("Param removed. No param defined internally in XML: DEV",
  "MODEL D1N3940_VER2 D\\(\\(BV=600 DEV=1%\\) IS=4E-10 RS=.105",
  "Total critical issues reported 			 = 0:",
  "Total          errors reported 			 = 0:", 
  "Total          warnings reported 			 = 2:", 
  "Total          information messages reported 	 = 1:", 
  "SUCCESS: xdm completion status flag = 0:"
);
$searchStringsPtr=\@searchStrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"pspice",$absTol,$relTol,$zeroTol,
                                $verbose,$searchStringsPtr);
