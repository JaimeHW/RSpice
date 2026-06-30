#!/usr/bin/env perl

use MeasureCommon;

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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# these are the tolerances used in comparing the measures calculated by this 
# script to those calculated by Xyce.  Zerotol is needed because some measured
# values should be zero (and reltol won't make sense in that case).

# phase is output in degrees and for fourier components with very small magnitude, 
# it can be have several degrees of scatter.  Thus it gets its own abstol
$absTol = 0.5;
$relTol = 0.01;
$phaseAbsTol = 1.0;
$phaseRelTol = 0.01;
$zeroTol = 1.0e-8;

# remove previous output files
system("rm -f $CIRFILE.csd $CIRFILE.out $CIRFILE.err $CIRFILE.mt* $CIRFILE.remeasure.*");

# run Xyce and check for the proper files
$CMD="$XYCE $CIRFILE > $CIRFILE.out";
$retval = system($CMD);

if ($retval !=0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n";
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIRFILE;
    exit 13;
  }
  else
  {
    print "Exit code = 10\n";
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE;
    exit 10;
  }
}

# Did we make the output file
if (not -s "$CIRFILE.csd" ) { print "Exit code = 14\n"; exit 14; }

#
# Did we make the measure files
#
if ( (not -s "$CIRFILE.mt0") ||  (not -s "$CIRFILE.mt1") || (not -s "$CIRFILE.mt2") )
{ 
  print "Exit code = 17\n"; exit 17; 
}

# do re-measure
use File::Copy;
move("$CIRFILE.mt0","$CIRFILE.temp.mt0");
move("$CIRFILE.mt1","$CIRFILE.temp.mt1");
move("$CIRFILE.mt2","$CIRFILE.temp.mt2");

# here is the command to run xyce with remeasure
my $CMD="$XYCE -remeasure $CIRFILE.csd $CIRFILE > $CIRFILE.remeasure.out";
$retval = system($CMD);

if ($retval !=0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n";
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIRFILE;
    exit 13;
  }
  else
  {
    print "Exit code = 10\n";
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE;
    exit 10;
  }
}

# Did we make the measure files
if ( (not -s "$CIRFILE.mt0") ||  (not -s "$CIRFILE.mt1") || (not -s "$CIRFILE.mt2") )
{ 
  print "Exit code = 17\n"; exit 17; 
}

# rename the files
move("$CIRFILE.mt0","$CIRFILE.remeasure.mt0");
move("$CIRFILE.temp.mt0","$CIRFILE.mt0");
move("$CIRFILE.mt1","$CIRFILE.remeasure.mt1");
move("$CIRFILE.temp.mt1","$CIRFILE.mt1");
move("$CIRFILE.mt2","$CIRFILE.remeasure.mt2");
move("$CIRFILE.temp.mt2","$CIRFILE.mt2");

# Phase is output in degrees and for fourier components with very small magnitude.  So, it 
# can have several degrees of scatter.  Thus it gets its own phaseAbsTol and phaseRelTol.
$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/compare_fourier_files/;

# now compare the re-measured and original file contents
foreach $i (0 .. 2)
{
  $CMD = "$fc $CIRFILE.remeasure.mt$i $CIRFILE.mt$i $absTol $relTol $phaseAbsTol $phaseRelTol $zeroTol > $CIRFILE.remeasure.mt$i.out 2> $CIRFILE.remeasure.mt$i.err";
  $retval=system($CMD);
  $retval= $retval >> 8;
  if ( $retval != 0 )
  {
    print "test Failed for re-measure of step $i\n";
    print "Exit code = $retval\n"; 
    exit $retval;
  }
}
  
print "Exit code = $retval\n"; 
exit $retval;
