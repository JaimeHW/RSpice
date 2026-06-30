#!/usr/bin/env perl

use XdmCommon;

# The input arguments to this script are set up in 
# Xyce_Test/TestScripts/tsc_run_test_suite and are as follows:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 
$GOLDPRN=$ARGV[4];

# Comparison tolerances
$absTol = 1.0e-5;
$relTol = 1.0e-3;
$zeroTol = 1.0e-10;

@searchStrings = (
  "Total critical issues reported 			 = 0:",
  "Total          errors reported 			 = 0:", 
  "Total          warnings reported 			 = 0:", 
  "Total          information messages reported 	 = 0:", 
  "SUCCESS: xdm completion status flag = 0:",
);
$searchStringsPtr=\@searchStrings;
$xyceSearchStrings = undef;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"hspice",$absTol,$relTol,$zeroTol,$verbose,$searchStringsPtr,$xyceSearchStrings,$GOLDPRN);
