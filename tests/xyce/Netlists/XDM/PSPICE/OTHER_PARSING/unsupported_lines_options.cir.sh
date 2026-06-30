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
# Must escape a double quote " with a single backslash \
# Do not have to escape { } or _.
# Strings to test for in the diagnostic output from xdm.
@xdmOutputSearchStrings = (
  "at line\\:\\[16\\]. Could not accept .OPTIONS \"ADVCONV\". Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[17\\]. Could not accept .OPTIONS \"CHGTOL\". Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[18\\]. Could not accept .OPTIONS \"ITL2\". Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[19\\]. Could not accept .OPTIONS \"SPEED_LEVEL\". Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[12\\]. Unsupported type\\: .AUTOCONVERGE. Retained \\(as a comment\\). Continuing.",
  "Total critical issues reported 			 = 0:", 
  "Total          errors reported 			 = 0:", 
  "Total          warnings reported 			 = 5:", 
  "Total          information messages reported 	 = 0:", 
  "SUCCESS: xdm completion status flag = 0:"
);
$xdmOutputSearchStringsPtr=\@xdmOutputSearchStrings;

#Strings to test for in the translated Xyce netlist
@translatedXyceNetlistSearchStrings = (
  "\\.OPTIONS TIMEINT METHOD=TRAP"
);
$translatedXyceNetlistSearchStringsPtr=\@translatedXyceNetlistSearchStrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"pspice",$absTol,$relTol,$zeroTol,
                                $verbose,$xdmOutputSearchStringsPtr,
                                $translatedXyceNetlistSearchStringsPtr);
