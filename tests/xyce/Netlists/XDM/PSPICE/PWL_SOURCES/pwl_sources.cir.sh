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
@searchstrings = (
 "V_V5 1 0 PWL REPEAT FOREVER \\(0,0\\) \\(5e-4,1\\) \\(1e-3,0\\)  ENDREPEAT; PSpice Parser Retained \\(as a comment\\). Continuing.",
 "V_V6 2 0 PWL REPEAT FOR 5 \\(0,0\\) \\(5e-4,1\\) \\(1e-3,0\\) ENDREPEAT; PSpice Parser Retained \\(as a comment\\). Continuing.",
 "V_V7 3 0 PWL TIME_SCALE_FACTOR=0.1 \\(0,0\\) \\(0.5,1\\) \\(1,0\\); PSpice Parser Retained \\(as a comment\\). Continuing.",
 "V_V8 4 0 PWL VALUE_SCALE_FACTOR=0.1 \\(0,0\\) \\(0.5,1\\) \\(1,0\\); PSpice Parser Retained \\(as a comment\\). Continuing.",
 "Total critical issues reported 			 = 0:", 
 "Total          errors reported 			 = 0:", 
 "Total          warnings reported 			 = 4:",
 "Total          information messages reported 	 = 0:", 
 "SUCCESS: xdm completion status flag = 0:"
);
$searchStringsPtr=\@searchstrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"pspice",$absTol,$relTol,$zeroTol,
                                $verbose,$searchStringsPtr);

