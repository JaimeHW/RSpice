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
# need to use the AC comparator, because the normal one doesn't understand what
# variables to expect in a *.SENS.prn file.
$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;

$TRANSLATESCRIPT=$XYCE_VERIFY;
$TRANSLATESCRIPT =~ s/xyce_verify.pl/convertToPrn.py/;
$TRANSLATE="python $TRANSLATESCRIPT ";

$CIR1="diode_adjoint_prn.cir";
$CIR2="diode_adjoint_tecplot.cir";

#comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-2;  #1%
$zerotol=1e-8;
$freqreltol=1e-6;

system("rm -f $CIR1.SENS.* $CIR2.SENS.* test_$CIR2.SENS.dat");
$retval=0;
$retval=$Tools->wrapXyce($XYCE,$CIR1);
if ($retval != 0)
{
  `echo "Xyce EXITED WITH ERROR! on $CIR1" >> $CIR1.err`;
  print "Exit code = $retval\n";
  exit $retval;
}

`rm -f $CIR2.SENS.dat`;

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
# We only do the first run. 
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIR1 $CIR1.prn $CIR1.prn > $CIR1.prn.out 2>&1 $CIR1.prn.err"))
    {
        print "Exit code = 2 \n";
        exit 2;
    }
    else
    {
        print "Exit code = 0 \n";
        exit 0;
    }
}

# cannot use wrap xyce for CIR2 because CIR2 produces a *.dat file, and does
# not produce a *prn file.  wrapXyce will return a 14 (missing prn file)
# error on CIR2, even if it runs fine.
$CMD="$XYCE $CIR2 > $CIR2.out 2>$CIR2.err";
$retval=system($CMD);

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

$exit_code = 0;
if ( not -s "$CIR1.SENS.prn" )
{
  print STDERR "Missing output file $CIR1.SENS.prn\n";
  $exit_code = 14;
}

if ( not -s "$CIR2.SENS.dat" )
{
  print STDERR "Missing output file $CIR2.SENS.dat\n";
  $exit_code = 14;
}

if ($exit_code != 0)
{
  print "Exit code = $exit_code\n";
  exit $exit_code;
}

$retcode = 0;

# check the number of TITLE outputs
$titlecount = `grep -ic title $CIR2.SENS.dat 2>/dev/null`;
if ($titlecount =~ 1)
{
  printf "Tecplot file contained correct number(%d) of titles.\n", $titlecount;
}
else
{
  printf "Tecplot file contained wrong number(%d) of titles.\n", $titlecount;
  $retcode = 2;
}

# check the number of ZONE outputs
$zonecount = `grep -ic zone $CIR2.SENS.dat 2>/dev/null`;
if ($zonecount =~ 1)
{
  printf "Tecplot file contained correct number(%d) of zones.\n",$zonecount;
}
else
{
  printf "Tecplot file contained wrong number(%d) of zones.\n",$zonecount;
  $retcode = 2;
}

# check the number of DATASETAUXDATA outputs
$datasetauxdatacount = `grep -ic datasetauxdata $CIR2.SENS.dat 2>/dev/null`;
if ($datasetauxdatacount =~ 2)
{
  printf "Tecplot file contained correct number(%d) of datasetauxdata's.\n",$datasetauxdatacount;
}
else
{
  printf "Tecplot file contained wrong number(%d) of datasetauxdata's.\n",$datasetauxdatacount;
  $retcode = 2;
}


#VARIABLES = " V(2)"
#" I(V1)"
#" {I(V1)}"
#" d{I(V1)}/d(DA:RS)_Adj"
#" d{I(V1)}/d(DA:IS)_Adj"

# check the number of VARIABLES outputs
$variablescount = `grep -ic variables $CIR2.SENS.dat 2>/dev/null`;
if ($variablescount =~ 1)
{
  printf "Tecplot file contained correct number(%d) of variables strings.\n",$variablescount;
}
else
{
  printf "Tecplot file contained wrong number(%d) of variables strings.\n",$variablescount;
  $retcode = 2;
}

$footercount = `grep -ic "End of Xyce(TM) Sensitivity Simulation" $CIR2.SENS.dat 2>/dev/null`;
if ($footercount =~ 1)
{
  print "Tecplot $CIRFILE.SENS.dat file contained correct simulation footer\n";
}
else
{
  print "Tecplot $CIRFILE.SENS.dat file contained wrong simulation footer.\n";
  $retcode = 2;
}

# convert and compare the *SENS.dat file.
$result = system( "$TRANSLATE $CIR2.SENS.dat");
if ( $result != 0 )
{
  print "Failed to translate TECPLOT to STD\n";
  $retcode = 2;
}
else
{
 $CMD="$XYCE_ACVERIFY --skipfooter $CIR1.SENS.prn test_$CIR2.SENS.dat $abstol $reltol $zerotol $freqreltol > $CIR2.SENS.dat.out 2> $CIR2.SENS.dat.err";
 if (system($CMD) != 0) {
     print STDERR "Verification failed on file $CIR2.SENS.dat, see $CIR2.SENS.dat.err\n";
     $retcode = 2;
 }
}

print "Exit code = $retcode\n"; exit $retcode;

