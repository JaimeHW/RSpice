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

# Note: must escape ( ) and * with \\ to get the CheckError() function to work.
# Do not have to escape { } or _.
@searchstrings = ("Non-ASCII character detected in the comment within file 'ext_ascii_chars_comment_line.cir.hspice' at line number\\(s\\) \\[5\\]",
  "Total critical issues reported 			 = 0:",
  "Total          errors reported 			 = 0:",
  "Total          warnings reported 			 = 1:",
  "Total          information messages reported 	 = 0:",
  "SUCCESS: xdm completion status flag = 0:"
);

$searchStringsPtr=\@searchstrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"hspice",$absTol,$relTol,$zeroTol,
                                $verbose,$searchStringsPtr);
