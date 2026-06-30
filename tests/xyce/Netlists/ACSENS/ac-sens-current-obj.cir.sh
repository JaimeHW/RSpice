#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce the appropriate output files, then we return exit code 14.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

# Test Definition:
# The main purpose of this test is to verify that acobjfunc and objvars can both
# be used on the same .SENS line to define AC sensitivity objective functions.
# Previously, Xyce would return an error if both were used on the same .SENS line.
# (see gitlab issue 270). 

# This script runs Xyce on two netlist files and compares the results (which
# should be identical). The only different between the netlists is the objective
# function definitions on the .SENS line. The netlist with the _modifed.cir suffix
# uses both acobjfunc and objvars (running this netlist without error is the
# primary objective of this test). The .SENS line in the 1.cir netlist
# follows the old rule and only uses objvars. Comparing the output from these
# netlists validates that both forms of defining the sensitivity objective
# functions are equivalent. 

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

$CIR1 = $CIRFILE;
$CIR2 = $CIRFILE;
$CIR2 =~ s/.cir/_FD.cir/;

system("rm -f $CIR1*FD.SENS.* $CIR1*\.out $CIR1*\.err");
system("rm -f $CIR2*FD.SENS.* $CIR2*\.out $CIR2*\.err");

$CMD1="$XYCE $CIR1 > $CIR1.out 2>$CIR1.err";
$CMD2="$XYCE $CIR2 > $CIR2.out 2>$CIR2.err";

## Run first circuit

$retval = system("$CMD1");
if ($retval != 0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n";
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIR1;
    exit 13;
  }
  else
  {
    print "Exit code = 10\n";
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIR1;
    exit 10;
  }
}

$xyce_exit = 0;
if ( not -s "$CIR1.FD.SENS.prn")
{
  print "Missing output file $CIR1.FD.SENS.prn\n";
  $xyce_exit = 14;
}

if ($xyce_exit != 0) { print "Exit code = $xyce_exit\n"; exit $xyce_exit;}

## Run second circuit

$retval = system("$CMD2");
if ($retval != 0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n";
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIR2;
    exit 13;
  }
  else
  {
    print "Exit code = 10\n";
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIR2;
    exit 10;
  }
}

$xyce_exit = 0;
if ( not -s "$CIR2.FD.prn")
{
  print "Missing output file $CIR2.FD.prn\n";
  $xyce_exit = 14;
}

if ($xyce_exit != 0) { print "Exit code = $xyce_exit\n"; exit $xyce_exit;}

##

# now compare the results from both netlists
# The header for CIR2.prn was generated with finite differences, rather than .SENS analysis.
# As such, the header is different and must be modified.
$HEADER1=`head -1 $CIR1.FD.SENS.prn`;
chomp $HEADER1;
$SEDCMD="sed '1 s-^.*\$-".$HEADER1."-g'";
`$SEDCMD $CIR2.FD.prn > $CIR2.FD.SENS.prn`;


$retcode=0;

$absTol=1e-5;
$relTol=1e-2;
$zeroTol=1e-10;
$freqreltol=1e-6;
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;

$CMD="$XYCE_ACVERIFY --skipfooter $CIR1.FD.SENS.prn $CIR2.FD.SENS.prn $absTol $relTol $zeroTol $freqreltol";
$retval = system("$CMD");

$retval = $retval >> 8;
if ($retval != 0)
{
  print STDERR "Comparator exited with exit code $retval on file $CIR1.FD.SENS.prn\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
