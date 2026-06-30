#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();

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
#$GOLDPRN=$ARGV[4];

$CIR0="first_last.cir";
$CIR1="first.cir";
$CIR2="last.cir";

$retcode = 0;

# -------------------------------------------------------------------------------
# run the two gold standards ----------------------------------------------------
# -------------------------------------------------------------------------------
system("rm -f $CIR1.*");
system("rm -f $CIR2.*");
$CMD="$XYCE $CIR1 > $CIR1.out 2> $CIR1.err";
$ret1 = system("$CMD");

$CMD="$XYCE $CIR2 > $CIR2.out 2> $CIR2.err";
$ret2 = system("$CMD");

# -------------------------------------------------------------------------------
# NO OPTION ---------------------------------------------------------------------
# -------------------------------------------------------------------------------
system("rm -f $CIR0.nooption*");
$CMD="$XYCE -o $CIR0.nooption $CIR0 > $CIR0.nooption.out 2> $CIR0.nooption.err";
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

if (not -s "$CIR0.nooption.prn" )
{
  print "$CIR0.nooption.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

$CMD="$XYCE_VERIFY $CIRFILE $CIR1.prn $CIR0.nooption.prn > $CIR0.nooption.prn.out 2> $CIR0.nooption.prn.err";
if (system("$CMD") != 0) {
  print STDERR "Verification failed on file $CIR0.nooption.prn\n";
  $retcode = 2;
  print "Exit code = $retcode\n"; exit $retcode;
}

# -------------------------------------------------------------------------------
# IGNORE -----------------------------------------------------------------------
# -------------------------------------------------------------------------------
system("rm -f $CIR0.ignore*");
$CMD="$XYCE -o $CIR0.ignore -redefined_params ignore $CIR0 > $CIR0.ignore.out 2> $CIR0.ignore.err";
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

if (not -s "$CIR0.ignore.prn" )
{
  print "$CIR0.ignore.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

$CMD="$XYCE_VERIFY $CIRFILE $CIR1.prn $CIR0.ignore.prn > $CIR0.ignore.prn.out 2> $CIR0.ignore.prn.err";
if (system("$CMD") != 0) {
  print STDERR "Verification failed on file $CIR0.ignore.prn\n";
  $retcode = 2;
  print "Exit code = $retcode\n"; exit $retcode;
}

# -------------------------------------------------------------------------------
# WARNING ----------------------------------------------------------------------
# -------------------------------------------------------------------------------
system("rm -f $CIR0.warning*");
@searchstrings1 = ("Netlist warning: Parameter FOOBAR defined more than once. Using first one.");
$XYCE_WITH_ARGS1="$XYCE -redefined_params warning ";
$retval = $Tools->runAndCheckWarning($CIR0,$XYCE_WITH_ARGS1,@searchstrings1);

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

if (not -s "$CIR0.prn" )
{
  print "$CIR0.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}
else
{
  system("mv $CIR0.prn $CIR0.warning.prn ");
}

if (not -s "$CIR0.warning.prn" )
{
  print "$CIR0.warning.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

$CMD="$XYCE_VERIFY $CIRFILE $CIR1.prn $CIR0.warning.prn > $CIR0.warning.prn.out 2> $CIR0.warning.prn.err";
if (system("$CMD") != 0) {
  print STDERR "Verification failed on file $CIR0.warning.prn\n";
  $retcode = 2;
  print "Exit code = $retcode\n"; exit $retcode;
}

# -------------------------------------------------------------------------------
# WARN -------------------------------------------------------------------------
# -------------------------------------------------------------------------------
system("rm -f $CIR0.warn*");
@searchstrings2 = ("Netlist warning: Parameter FOOBAR defined more than once. Using first one.");
$XYCE_WITH_ARGS2="$XYCE -redefined_params warn ";
$retval = $Tools->runAndCheckWarning($CIR0,$XYCE_WITH_ARGS2,@searchstrings2);
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

if (not -s "$CIR0.prn" )
{
  print "$CIR0.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}
else
{
  system("mv $CIR0.prn $CIR0.warn.prn ");
}

if (not -s "$CIR0.warn.prn" )
{
  print "$CIR0.warn.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

$CMD="$XYCE_VERIFY $CIRFILE $CIR1.prn $CIR0.warn.prn > $CIR0.warn.prn.out 2> $CIR0.warn.prn.err";
if (system("$CMD") != 0) {
  print STDERR "Verification failed on file $CIR0.warn.prn\n";
  $retcode = 2;
  print "Exit code = $retcode\n"; exit $retcode;
}

# -------------------------------------------------------------------------------
# USEFIRST ---------------------------------------------------------------------
# -------------------------------------------------------------------------------
system("rm -f $CIR0.usefirst*");
$CMD="$XYCE -o $CIR0.usefirst -redefined_params usefirst $CIR0 > $CIR0.usefirst.out 2> $CIR0.usefirst.err";
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

if (not -s "$CIR0.usefirst.prn" )
{
  print "$CIR0.usefirst.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}
$CMD="$XYCE_VERIFY $CIRFILE $CIR1.prn $CIR0.usefirst.prn > $CIR0.usefirst.prn.out 2> $CIR0.usefirst.prn.err";
if (system("$CMD") != 0) {
  print STDERR "Verification failed on file $CIR0.usefirst.prn\n";
  $retcode = 2;
  print "Exit code = $retcode\n"; exit $retcode;
}

# -------------------------------------------------------------------------------
# USEFIRSTWARN ------------------------------------------------------------------
# -------------------------------------------------------------------------------
system("rm -f $CIR0.usefirstwarn*");
@searchstrings3 = ("Netlist warning: Parameter FOOBAR defined more than once. Using first one.");
$XYCE_WITH_ARGS3="$XYCE -redefined_params usefirstwarn ";
$retval = $Tools->runAndCheckWarning($CIR0,$XYCE_WITH_ARGS3,@searchstrings3);
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

if (not -s "$CIR0.prn" )
{
  print "$CIR0.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}
else
{
  system("mv $CIR0.prn $CIR0.usefirstwarn.prn ");
}

if (not -s "$CIR0.usefirstwarn.prn" )
{
  print "$CIR0.usefirstwarn.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

$CMD="$XYCE_VERIFY $CIRFILE $CIR1.prn $CIR0.usefirstwarn.prn > $CIR0.usefirstwarn.prn.out 2> $CIR0.usefirstwarn.prn.err";
if (system("$CMD") != 0) {
  print STDERR "Verification failed on file $CIR0.usefirstwarn.prn\n";
  $retcode = 2;
  print "Exit code = $retcode\n"; exit $retcode;
}

# -------------------------------------------------------------------------------
# USELAST -----------------------------------------------------------------------
# -------------------------------------------------------------------------------
system("rm -f $CIR0.uselast*");
$CMD="$XYCE -o $CIR0.uselast -redefined_params uselast $CIR0 > $CIR0.uselast.out 2> $CIR0.uselast.err";
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

if (not -s "$CIR0.uselast.prn" )
{
  print "$CIR0.uselast.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

$CMD="$XYCE_VERIFY $CIRFILE $CIR2.prn $CIR0.uselast.prn > $CIR0.uselast.prn.out 2> $CIR0.uselast.prn.err";
if (system("$CMD") != 0) {
  print STDERR "Verification failed on file $CIR0.uselast.prn\n";
  $retcode = 2;
  print "Exit code = $retcode\n"; exit $retcode;
}

# -------------------------------------------------------------------------------
# USELASTWARN -------------------------------------------------------------------
# -------------------------------------------------------------------------------
system("rm -f $CIR0.uselastwarn*");
@searchstrings4 = ("Netlist warning: Parameter FOOBAR defined more than once. Using last one.");
$XYCE_WITH_ARGS4="$XYCE -redefined_params uselastwarn ";
$retval = $Tools->runAndCheckWarning($CIR0,$XYCE_WITH_ARGS4,@searchstrings4);
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

if (not -s "$CIR0.prn" )
{
  print "$CIR0.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}
else
{
  system("mv $CIR0.prn $CIR0.uselastwarn.prn ");
}

if (not -s "$CIR0.uselastwarn.prn" )
{
  print "$CIR0.uselastwarn.prn file is missing\n";
  print "Exit code = 14\n";
  exit 14;
}

$CMD="$XYCE_VERIFY $CIRFILE $CIR2.prn $CIR0.uselastwarn.prn > $CIR0.uselastwarn.prn.out 2> $CIR0.uselastwarn.prn.err";
if (system("$CMD") != 0) {
  print STDERR "Verification failed on file $CIR0.uselastwarn.prn\n";
  $retcode = 2;
  print "Exit code = $retcode\n"; exit $retcode;
}

# -------------------------------------------------------------------------------
# ERROR -------------------------------------------------------------------------
# -------------------------------------------------------------------------------
@searchstrings = ("Parameter FOOBAR defined more than once");

$XYCEWITHARG="$XYCE -redefined_params error ";
$retval = $Tools->runAndCheckError($CIR0,$XYCEWITHARG,@searchstrings);
if ($retval !=0)
{
  print "Exit code = $retval\n";
  exit $retval
}


print "Exit code = $retcode\n"; exit $retcode;
