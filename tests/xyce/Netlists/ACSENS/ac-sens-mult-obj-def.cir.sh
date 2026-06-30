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
# primary objective of this test). The .SENS line in the _baseline.cir netlist
# follows the old rule and only uses objvars. Comparing the output from these
# netlists validates that both forms of defining the sensitivity objective
# functions are equivalent. 

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

$CIR_NameBase = $CIRFILE;
$CIR_NameBase =~ s/\.cir//;
$CIR_baseline = "$CIR_NameBase\_baseline.cir";
$CIR_modified = "$CIR_NameBase\_modified.cir";

system("rm -f $CIR_NameBase*FD.SENS.* $CIR_NameBase*\.out $CIR_NameBase*\.err");

$CMD_baseline="$XYCE $CIR_baseline > $CIR_baseline.out 2>$CIR_baseline.err";
$CMD_modified="$XYCE $CIR_modified > $CIR_modified.out 2>$CIR_modified.err";

## Run first circuit

$retval = system("$CMD_baseline");
if ($retval != 0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n";
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIR_baseline;
    exit 13;
  }
  else
  {
    print "Exit code = 10\n";
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIR_baseline;
    exit 10;
  }
}

$xyce_exit = 0;
if ( not -s "$CIR_baseline.FD.SENS.prn")
{
  print "Missing output file $CIR_baseline.FD.SENS.prn\n";
  $xyce_exit = 14;
}

if ($xyce_exit != 0) { print "Exit code = $xyce_exit\n"; exit $xyce_exit;}

## Run second circuit

$retval = system("$CMD_modified");
if ($retval != 0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n";
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIR_modified;
    exit 13;
  }
  else
  {
    print "Exit code = 10\n";
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIR_modified;
    exit 10;
  }
}

$xyce_exit = 0;
if ( not -s "$CIR_modified.FD.SENS.prn")
{
  print "Missing output file $CIR_modified.FD.SENS.prn\n";
  $xyce_exit = 14;
}

if ($xyce_exit != 0) { print "Exit code = $xyce_exit\n"; exit $xyce_exit;}

##

# now compare the results from both netlists
$retcode=0;

$absTol=1e-5;
$relTol=1e-3;
$zeroTol=1e-10;
$freqreltol=1e-6;
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;

$CIR_NameBase = "$CIR_NameBase\_compare";
$CMD="$XYCE_ACVERIFY $CIR_baseline.FD.SENS.prn $CIR_modified.FD.SENS.prn $absTol $relTol $zeroTol $freqreltol";
$retval = system("$CMD");

$retval = $retval >> 8;
if ($retval != 0)
{
  print STDERR "Comparator exited with exit code $retval on file $CIR_modified.FD.SENS.prn\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;