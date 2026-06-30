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

# Note: must escape ( ) [ ] and * and . with \\ to get the CheckError() function to work.
# Must escape a double quote " with a single backslash \
# Do not have to escape { } or _.

# Strings to test for in the diagnostic output from xdm.
@xdmOutputSearchStrings = ("statistics \\{; Spectre statistics block Retained \\(as a comment\\). Continuing.",
   "SUCCESS: xdm completion status flag = 0");
$xdmOutputSearchStringsPtr=\@xdmOutputSearchStrings;

# Strings to test for in the translated Xyce netlist
# Make sure that the Spectre statistics lines (that start with *) are correctly 
# commented out in the translated Xyce netlist.  See SRN Bugs 2088 and 2089
# for more details.
@translatedXyceNetlistSearchStrings = ("\\* process",
   "\\* vary d1",   
   "\\* correlate param = \\[d2 d4\\]",
   "\\* mismatch");
$translatedXyceNetlistSearchStringsPtr=\@translatedXyceNetlistSearchStrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"spectre",$absTol,$relTol,$zeroTol,
                                $verbose,$xdmOutputSearchStringsPtr,
                                $translatedXyceNetlistSearchStringsPtr);
