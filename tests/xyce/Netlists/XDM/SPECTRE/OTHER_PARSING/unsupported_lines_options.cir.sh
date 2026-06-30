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

# Note: must escape ( ) and [ ] and * and . and : with \\ to get the CheckError() 
# function to work.
# Must escape a double quote " with a single backslash \
# Do not have to escape { } or _.
# Strings to test for in the diagnostic output from xdm.
@xdmOutputSearchStrings = (
  "Unsupported parameter in spectre tran statement\\: errpreset Line\\(s\\)\\: \\[21, 22\\]",
  "Unsupported parameter in spectre tran statement\\: write Line\\(s\\)\\: \\[21, 22\\]",
  "Unsupported parameter in spectre tran statement\\: writefinal Line\\(s\\)\\: \\[21, 22\\]",
  "Unsupported parameter in spectre tran statement\\: annotate Line\\(s\\)\\: \\[21, 22\\]",
  "Unsupported parameter in spectre tran statement\\: maxiters Line\\(s\\)\\: \\[21, 22\\]",
  "at line\\:\\[17, 18, 19, 20\\]. Unsupported type\\: simulatorOptions. Retained \\(as a comment\\). Continuing", 
  "at line\\:\\[23\\]. Unsupported type\\: finalTimeOP. Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[24\\]. Unsupported type\\: modelParameter. Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[25\\]. Unsupported type\\: element. Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[26\\]. Unsupported type\\: outputParameter. Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[27\\]. Unsupported type\\: designParamVals. Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[28\\]. Unsupported type\\: primitives. Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[29\\]. Unsupported type\\: subckts. Retained \\(as a comment\\). Continuing.",
  "at line\\:\\[31\\]. Unsupported type\\: saveOptions. Retained \\(as a comment\\). Continuing.",
  "Total critical issues reported 			 = 0:", 
  "Total          errors reported 			 = 0:", 
  "Total          warnings reported 			 = 14:", 
  "Total          information messages reported 	 = 1:",
  "SUCCESS: xdm completion status flag = 0:"
);
$xdmOutputSearchStringsPtr=\@xdmOutputSearchStrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"spectre",$absTol,$relTol,$zeroTol,
                                $verbose,$xdmOutputSearchStringsPtr);
