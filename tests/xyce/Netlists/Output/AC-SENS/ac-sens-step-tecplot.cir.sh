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
# output from comparison to go into $CIRFILE.dat.out and the STDERR output from
# comparison to go into $CIRFILE.dat.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDDAT=$ARGV[4];

$GOLDDAT =~ s/\.prn$//; # remove the .prn at the end.

$GOLDDAT =~ s/tecplot/prn/; # use the prn gold standard.

$XYCE_ACVERIFY = $XYCE_VERIFY;
$XYCE_ACVERIFY =~ s/xyce_verify/ACComparator/;

$TRANSLATESCRIPT=$XYCE_VERIFY;
$TRANSLATESCRIPT =~ s/xyce_verify.pl/convertToPrn.py/;
$TRANSLATE="python $TRANSLATESCRIPT ";

#comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-3;  #0.1%
$zerotol=1e-8;
$freqreltol=1e-6;

# remove previous output files
system("rm -f $CIRFILE.FD.SENS.* $CIRFILE.out $CIRFILE.err test_$CIRFILE.*");

#run Xyce
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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

# check if the tecplot output files exist.
if ( !(-f "$CIRFILE.FD.SENS.dat")) {
    print STDERR "Missing output file $CIRFILE.FD.dat\n";
    print "Exit code = 14\n"; exit 14;
}

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE unusedarg > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
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

$retcode = testFileBasic("$CIRFILE.FD.SENS.dat");
print "Done with call testFileBasic!  At this point retcode = $retcode\n";

# convert and compare the *FD.SENS.dat file.
$result = system( "$TRANSLATE $CIRFILE.FD.SENS.dat");
if ( $result != 0 )
{
  print "Failed to translate TECPLOT to STD\n";
  $retcode = 2;
}
else
{
  $CMD="$XYCE_ACVERIFY --skipfooter $GOLDDAT.FD.SENS.prn test_$CIRFILE.FD.SENS.dat $abstol $reltol $zerotol $freqreltol > $CIRFILE.FD.SENS.dat.out 2> $CIRFILE.FD.SENS.dat.err";
  if (system("$CMD") != 0) {
       print STDERR "Verification failed on file $CIRFILE.FD.SENS.dat vs $GOLDDAT.FD.SENS.prn, see $CIRFILE.FD.SENS.dat.err/out\n";
       $retcode = 2;
  }
}


print "Exit code = $retcode\n"; exit $retcode;

########## sub declarations

sub testFileBasic {

  my ($filename) = @_;

  $retval = 0;

  # check the number of TITLE outputs
  $titlecount = `grep -ic title $filename 2>/dev/null`;
  if ($titlecount =~ 1)
  {
    printf "Tecplot file %s contained correct number(%d) of titles.\n", $filename, $titlecount;
  }
  else
  {
    printf "Tecplot file %s contained wrong number(%d) of titles.\n", $filename, $titlecount;
    $retval = 2;
  }

  # check the number of ZONE outputs
  $zonecount = `grep -ic zone $filename 2>/dev/null`;
  if ($zonecount =~ 2)
  {
    printf "Tecplot file %s contained correct number(%d) of zones.\n", $filename,$zonecount;
  }
  else
  {
    printf "Tecplot file %s contained wrong number(%d) of zones.\n", $filename,$zonecount;
    $retval = 2;
  }

  # check the number of AUXDATA outputs
  $auxdatacount = `grep -iv datasetauxdata $filename | grep -ic auxdata 2>/dev/null`;
  if ($auxdatacount =~ 2)
  {
    printf "Tecplot file %s contained correct number(%d) of auxdata's.\n", $filename,$auxdatacount;
  }
  else
  {
    printf "Tecplot file %s contained wrong number(%d) of auxdata's.\n", $filename,$auxdatacount;
    $retval = 2;
  }

  # check the number of DATASETAUXDATA outputs
  $datasetauxdatacount = `grep -ic datasetauxdata $filename 2>/dev/null`;
  if ($datasetauxdatacount =~ 2)
  {
    printf "Tecplot file %s contained correct number(%d) of datasetauxdata's.\n", $filename,$datasetauxdatacount;
  }
  else
  {
    printf "Tecplot file %s contained wrong number(%d) of datasetauxdata's.\n", $filename,$datasetauxdatacount;
    $retval = 2;
  }

  # check the number of "End of Xyce(TM) Parameter Sweep" outputs
  $xyceEndCount = `grep -ic "End of Xyce(TM) Sensitivity Simulation" $filename 2>/dev/null`;
  if ($xyceEndCount =~ 1)
  {
    printf "Tecplot file %s contained correct number(%d) of \"End of Xyce(TM) Sensitivity Simulation\" statements.\n", $filename,$xyceEndCount;
  }
  else
  {
    printf "Tecplot file %s contained wrong number(%d) of \"End of Xyce(TM) Sensitivity Simulation\" statements.\n", $filename,$xyceEndCount;
    $retval = 2;
  }

  return $retval;
}

