#!/usr/bin/env perl

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

# Comparison tolerances
$absTol = 1.0e-5;
$relTol = 1.0e-3;
$zeroTol = 1.0e-10;

# Note: must escape ( ) [ ] and * with \\ to get the CheckError() function to work.
# Do not have to escape { } or _.
@xdmOutputSearchStrings = (
  "Line\\(s\\):\\[26\\]. Unsupported Output Variable in Xyce: I\\(\\*\\)",
  "Line\\(s\\):\\[26\\]. Unsupported Output Variable in Xyce: W\\(\\*\\)",
  "Line\\(s\\):\\[26\\]. Unsupported Output Variable in Xyce: D\\(\\*\\)",
  "Line\\(s\\):\\[26\\]. Unsupported Output Variable in Xyce: NOISE\\(\\*\\)",
  "Line\\(s\\):\\[28\\]. Unsupported Output Variable in Xyce: I\\(\\*\\)",
  "Line\\(s\\):\\[29\\]. Unsupported Output Variable in Xyce: W\\(\\*\\)",
  "Line\\(s\\):\\[30\\]. Unsupported Output Variable in Xyce: D\\(\\*\\)",
  "Line\\(s\\):\\[31\\]. Unsupported Output Variable in Xyce: NOISE\\(\\*\\)",
  "Total critical issues reported 			 = 0:",
  "Total          errors reported 			 = 0:", 
  "Total          warnings reported 			 = 8:", 
  "Total          information messages reported 	 = 0:", 
  "SUCCESS: xdm completion status flag = 0:"
);
$xdmOutputSearchStringsPtr=\@xdmOutputSearchStrings;

#Strings to test for in the translated Xyce netlist
@translatedXyceNetlistSearchStrings = (
  ".PRINT DC FORMAT=PROBE V\\(1\\) V\\(2a\\) V\\(2b\\) I\\(R1\\) W\\(R1\\) I\\(R3\\) W\\(R3\\) V\\(\\*\\)"
);
$translatedXyceNetlistSearchStringsPtr=\@translatedXyceNetlistSearchStrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"pspice",$absTol,$relTol,$zeroTol,
                                $verbose,$xdmOutputSearchStringsPtr,
                                $translatedXyceNetlistSearchStringsPtr);
