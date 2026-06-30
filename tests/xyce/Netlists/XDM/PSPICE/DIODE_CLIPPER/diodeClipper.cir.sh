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

# this is a bit of a hack but DIODE_CLIPPER tests that there is no need for 
# an extra blank after the last .MODEL block in PSpice .lib files. So, the 
# last line of nom.lib should not be "blank"
open(LIBFILE,"nom.lib");
$blankLineFlag=1;
$lineCount=0;
while($lineLibFile=<LIBFILE>)
{
  # test that the line is not empty, or only has spaces or only has white-space
  if ( ($lineLibFile eq '') || ($lineLibFile =~ /^ *$/) || ($lineLibFile =~ /^\s*$/) )
  {
    $blankLineFlag=1;
  }
  else
  {
    $blankLineFlag=0;
  }
  $lineCount++;
} 
close(LIBFILE);

if ( ($blankLineFlag > 0) && ($lineCount > 0) )
{
  print "Last line of nom.lib was empty\n";
  print "Exit code = 2 \n";
  exit 2;
} 
else
{
  print "nom.lib is correct to test SRN Bug 1989\n";  
}

# Strings to search for in stdout, based on xdm warning messages.  Need to pass
# a pointer into verifyXDMtranslations since the number of search strings is variable.
@searchstrings = ("Could not accept .OPTIONS",
 "Total critical issues reported 			 = 0:", 
 "Total          errors reported 			 = 0:", 
 "Total          warnings reported 			 = 2:", 
 "Total          information messages reported 	 = 1:", 
 "SUCCESS: xdm completion status flag = 0:"
);
$searchStringsPtr=\@searchstrings;

XdmCommon::verifyXDMtranslation($XYCE,$XYCE_VERIFY,$CIRFILE,"pspice",$absTol,$relTol,$zeroTol,
                                $verbose,$searchStringsPtr);
