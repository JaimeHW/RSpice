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
@searchstrings = ("Could not convert non-ASCII character\\(s\\) within file 'ext_ascii_chars_noncomment_line.cir.pspice' at line number\\(s\\) \\[13\\]");
$searchStringsPtr=\@searchstrings;

XdmCommon::runAndCheckXDMerror($XYCE,$XYCE_VERIFY,$CIRFILE,"pspice",$verbose,$searchStringsPtr);
