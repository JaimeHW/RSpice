#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

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
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# remove old files if they exist
system("rm -f $CIRFILE.NOISE.prn $CIRFILE.err $CIRFILE.out $CIRFILE.noise.*");

# run Xyce
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
$retval = system("$CMD");
if ($retval != 0)
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

# Exit if the NOISE.prn file was not made
if (not -s "$CIRFILE.NOISE.prn" )
{
  print "$CIRFILE.NOISE.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

# Now check the .NOISE.prn file
$retcode=0;
$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-16;
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$GOLDNOISE = substr($GOLDPRN,0,-3)."NOISE.prn";
$CMD="$fc $CIRFILE.NOISE.prn $GOLDNOISE $absTol $relTol $zeroTol > $CIRFILE.noise.out 2> $CIRFILE.noise.err";
$retval = system("$CMD");
$retval = $retval >> 8;
if ( $retval != 0 ){
  print STDERR "Comparator exited on file $CIRFILE.prn with exit code $retval\n";
  $retcode = 2;
}
else
{
  print "Passed comparison of .prn files\n";
}

# check for the warning message
@searchstrings=("Netlist warning: Total Noise Integrals will not be calculated, since",
                "frequencies in .DATA table are not monotonically increasing");
$retval = $Tools->checkError("$CIRFILE.out",@searchstrings);
if ( $retval != 0){  $retcode = $retval };

# Make sure that the Total Noise Integral values ARE NOT in the .out file.
# Check that .out file exists, and open it if it does.
if (not -s "$CIRFILE.out" )
{
  print "Exit code = 17\n";
  exit 17;
}
else
{
  open(NETLIST, "$CIRFILE.out");
  open(ERRMSG,">$CIRFILE.errmsg") or die $!;
}

# parse the .out file to find the text related to .NOISE
$foundStart=0;
$foundEnd=0;
@outLine;
$lineCount=0;
while( $line=<NETLIST> )
{
  if ($line =~ /Total Output Noise/) { $foundStart = 1; }

  if ($foundStart > 0 && $foundEnd < 1)
  {
    print ERRMSG $line;
  }

  if ($foundStart > 0 && $line =~ /Total Input Noise/) { $foundEnd = 1; }
}

close(NETLIST);
close(ERRMSG);

if ( $foundStart != 0 || $foundEnd !=0 ){
  print "Total Noise Integral lines found in $CIRFILE.out, when they should not\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;

