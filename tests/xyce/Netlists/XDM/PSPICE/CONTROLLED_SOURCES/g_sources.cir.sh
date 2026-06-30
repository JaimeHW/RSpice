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
  "GLAPLACE bogo_node 0 LAPLACE \\{V\\(10\\)} = \\{1/\\(1\\+.001\\*s\\)}; PSpice Parser Retained \\(as a comment\\). Continuing.",
  "GFREQ bogo_node 0 FREQ \\{V\\(10\\)}=\\(0,0,0\\)\\(5kHz, 0,0\\)\\(6kHz -60, 0\\) DELAY=3.2ms; PSpice Parser Retained \\(as a comment\\). Continuing.",
  "GCHEB bogo_node 0 CHEBYSHEV \\{V\\(10\\)} = LP 800 1.2K .1dB 50dB; PSpice Parser Retained \\(as a comment\\). Continuing.",
  "Total critical issues reported 			 = 0:", 
  "Total          errors reported 			 = 0:", 
  "Total          warnings reported 			 = 3:", 
  "Total          information messages reported 	 = 0:", 
  "SUCCESS: xdm completion status flag = 0:"
);
$searchStringsPtr=\@searchstrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"pspice",$absTol,$relTol,$zeroTol,
                                $verbose,$searchStringsPtr);
