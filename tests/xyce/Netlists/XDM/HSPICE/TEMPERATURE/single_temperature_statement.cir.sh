#!/usr/bin/env perl

use XyceRegression::Tools;
use XdmCommon;
use Getopt::Long;
&GetOptions( "verbose!" => \$verbose );

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIR=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$Tools = XyceRegression::Tools->new();
if (defined($verbose)) { $Tools->setVerbose(1); }

# remove old files if they exist
system("rm -f $CIR.prn*");
system("rm -f $CIR.out $CIR.err");

# run known good Xyce .cir file, and check that it worked
$CMD="$XYCE -o $CIR $CIR > $CIR.out 2> $CIR.err";
$retval = system("$CMD");
if ($retval != 0)
 {
   if ($retval & 127)
   {
     print "Exit code = 13\n";
     printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIR;
     exit 13;
  }
   else
   {
     print "Exit code = 10\n";
     printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIR;
     exit 10;
   }
 }

# check the runtime output of gold netlist vs gold standard output 
if (-s $GOLDPRN)
{
  $absTol=1e-5;
  $relTol=1e-3;
  $zeroTol=1e-10;
  $fc = $XYCE_VERIFY;
  $fc=~ s/xyce_verify/file_compare/;
  
  $retcode = 0;
  
  $CMD="$fc $CIR.s2p $GOLDPRN $absTol $relTol $zeroTol > $CIR.s2p.gold.out 2> $CIR.s2p.gold.err";
  $retval = system("$CMD");
  $retval = $retval >> 8;
  if ($retval == 0)
  {
    $retcode = 0;
  }
  else
  {
    print STDERR "Comparator exited with exit code $retval on file $CIR.s2p vs gold standard output at $GOLDPRN\n";
    $retcode = 2;
    print "Exit code = $retcode\n"; exit $retcode;
  }
}

# remove any previous tranlation directory
system("rm -f $CIR-translated");

# run xdm
my ($XDMEXECSTR,$FROMSPICEFILE,$TRANSLATEDDIR,$OUTFILETYPE) = XdmCommon::setXDMvariables("hspice",$CIR);
my $CMD=$XDMEXECSTR;

if (system($CMD) != 0)
{
  print "XDM exited with errors, or failed to run\n";
  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "XDM translation succeeded\n";
}

# rename the translated file
$CMD="mv ./$TRANSLATEDDIR/$FROMSPICEFILE ./$TRANSLATEDDIR/$CIR";
if (system($CMD) != 0)
{
  print "Rename of translated file failed.\n";
  print "Exit code = 2\n";
  exit 2;
}

# run the translated netlist and check for errors.
# need to change directories to ./translated so that the translated .lib and .NET files
# are used.
chdir "./$TRANSLATEDDIR";
$retval=system("$XYCE -o $CIR $CIR > $CIR.out 2> $CIR.err");
if ($retval != 0)
{
  print STDERR "Xyce crashed trying to run translated files\n";
  print "Exit code = 14 \n";
  exit 14;
}
else
{
  print "Translated Xyce netlist ran\n";
}

# Exit if the various output files were not made
if (not -s "$CIR.prn" )
{
  print "Translated $CIR.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

chdir "..";

if (not -s "$CIR.prn" )
{
  print "Gold $CIR.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

@searchStrings = ("Total critical issues reported 			 = 0:",
  "Total          errors reported 			 = 0:", 
  "Total          warnings reported 			 = 0:", 
  "Total          information messages reported 	 = 0:", 
  "SUCCESS: xdm completion status flag = 0:",
);
$xdmOutputSearchStringsPtr=\@searchStrings;

print "checking for xdm warning/error messages\n";
my @xdmOutputSearchStrings = @$xdmOutputSearchStringsPtr;
$retval = $Tools->checkError("$CIR.xdm.out",@xdmOutputSearchStrings);
if ($retval != 0)
{
  print "search for xdm warning/error messages failed\n";
  print "Exit code = $retval\n";
  exit $retval;
}
else
{
  print "test for xdm warning/error messages passed\n";
}

# Now check the various output files
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;

$retcode = 0;

$CMD="$fc $CIR.prn $TRANSLATEDDIR/$CIR.prn $absTol $relTol $zeroTol > $CIR.prn.out 2> $CIR.prn.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ($retval == 0)
{
  $retcode = 0;
}
else
{
  print STDERR "Comparator exited with exit code $retval on file $CIR.prn\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
