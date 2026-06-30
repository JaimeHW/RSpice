#!/usr/bin/env perl

use File::Basename;
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

# If Xyce does not produce the appropriate output files, then we return exit code 14.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
#$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$TRANSLATESCRIPT=$XYCE_VERIFY;
$TRANSLATESCRIPT =~ s/xyce_verify.pl/convertToPrn.py/;
$TRANSLATE="python $TRANSLATESCRIPT ";

$CIR1="mpde-tecplot.cir";
$CIR2="mpde-tecplot-fallback.cir";

$TMPCIRFILE1="printLine_for_mpde-tecplot.cir";
$TMPCIRFILE2="printLine_for_mpde-tecplot-fallback.cir";

$GOLDDIR = dirname($GOLDPRN);
$GOLD1 = "$GOLDDIR/$CIR1";
$GOLD2 = "$GOLDDIR/$CIR2";

# remove previous output files
system("rm -f $CIR1.prn $CIR1.MPDE.* $CIR1.mpde_ic.* $CIR1.startup.* $CIR1.out $CIR1.err");
system("rm -f $CIR2.prn $CIR2.MPDE.* $CIR2.mpde_ic.* $CIR2.startup.* $CIR2.out $CIR2.err");
system("rm -f $CIR1.prn.* $CIR2.prn.*");

# run Xyce on both netlists
$CMD="$XYCE $CIR1 > $CIR1.out 2>$CIR1.err";
$retval=system($CMD);

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

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIR1 $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
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

$CMD="$XYCE $CIR2 > $CIR2.out 2>$CIR2.err";
$retval=system($CMD);

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

# check for output files from both netlists
if ( !(-f "$CIR1.prn")) {
    print STDERR "Missing output file $CIR1.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR1.MPDE.dat")) {
    print STDERR "Missing output file $CIR1.MPDE.dat\n";
    $xyceexit=14;
}
if ( !(-f "$CIR1.mpde_ic.dat")) {
    print STDERR "Missing output file $CIR1.mpde_ic.dat\n";
    $xyceexit=14;
}
if ( !(-f "$CIR1.startup.dat")) {
    print STDERR "Missing output file $CIR1.startup.dat\n";
    $xyceexit=14;
}

# check for output files from both netlists
if ( !(-f "$CIR2.prn")) {
    print STDERR "Missing output file $CIR2.prn\n";
    $xyceexit=14;
}
if ( !(-f "$CIR2.MPDE.dat")) {
    print STDERR "Missing output file $CIR2.MPDE.dat\n";
    $xyceexit=14;
}
if ( !(-f "$CIR2.mpde_ic.dat")) {
    print STDERR "Missing output file $CIR2.mpde_ic.dat\n";
    $xyceexit=14;
}
if ( !(-f "$CIR2.startup.dat")) {
    print STDERR "Missing output file $CIR2.startup.dat\n";
    $xyceexit=14;
}

if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

$retcode = 0;
# check for simulation footer text
# Output from Netlist 1
$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR1.MPDE.dat 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR1.MPDE.dat contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR1.MPDE.dat contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR1.mpde_ic.dat 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR1.mpde_ic.dat contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR1.mpde_ic.dat contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR1.startup.dat 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR1.startup.dat contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR1.startup.dat contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

# output for Netlist 2
$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR2.MPDE.dat 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR2.MPDE.dat contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR2.MPDE.dat contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR2.mpde_ic.dat 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR2.mpde_ic.dat contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR2.mpde_ic.dat contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

$xyceEndCount = `grep -ic "End of Xyce(TM) Simulation" $CIR2.startup.dat 2>/dev/null`;
if ($xyceEndCount =~ 1)
{
  printf "Output file $CIR2.startup.dat contained correct number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
}
else
{
  printf "Output file $CIR2.startup.dat contained wrong number(%d) of \"End of Xyce(TM) Simulation\" statements.\n",$xyceEndCount;
  $retcode = 2;
}

# check contents of MPDE-specific output files for both netlists.
print "Checking contents of output files\n";

# Only check header line in <netlistName>.MPDE.dat files
$headerTest = testTecplotHeaders("$CIR1.MPDE.dat");
if ($headerTest != 0) { $retcode = $headerTest;}
print "Done with testing Tecplot headers for $CIR1.MPDE.dat\n";
print "At this point retcode = $retcode\n";

$result = system("$TRANSLATE $CIR1.mpde_ic.dat");
if ( $result != 0 )
{
  print "Failed to translate TECPLOT to STD for $CIR1.mpde_ic.dat\n";
  $retcode = 2;
}
else
{
  $CMD="$XYCE_VERIFY --printline=mpde_ic $TMPCIRFILE1 test_$CIR1.mpde_ic.dat $GOLD1.mpde_ic.prn > $CIR1.mpde_ic.dat.out 2> $CIR1.mpde_ic.dat.err";
  if (system("$CMD") != 0) {
      print STDERR "Verification failed on file test_$CIR1.mpde_ic.dat with $GOLD1.mpde_ic.prn, see $CIR1.mpde_ic.dat.err\n";
      $retcode = 2;
  }
}

$result = system("$TRANSLATE $CIR1.startup.dat");
if ( $result != 0 )
{
  print "Failed to translate TECPLOT to STD for $CIR1.startup.dat\n";
  $retcode = 2;
}
else
{
  $CMD="$XYCE_VERIFY --printline=mpde_startup $TMPCIRFILE1 test_$CIR1.startup.dat $GOLD1.startup.prn > $CIR1.startup.dat.out 2> $CIR1.startup.dat.err";
  if (system("$CMD") != 0) {
      print STDERR "Verification failed on file test_$CIR1.startup.dat with $GOLD1.startup.prn, see $CIR1.startup.dat.err\n";
      $retcode = 2;
  }
}

$CMD="$XYCE_VERIFY $CIR1 $GOLD1.prn $CIR1.prn > $CIR1.prn.out 2> $CIR1.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIR1.prn, see $CIR1.prn.err\n";
    $retcode = 2;
}

# netlist 2

$headerTest = testTecplotHeaders("$CIR2.MPDE.dat");
print "Done with testing Tecplot headers for $CIR2.MPDE.dat\n";
if ($headerTest != 0) { $retcode = $headerTest;}
print "At this point retcode = $retcode\n";

$result = system("$TRANSLATE $CIR2.mpde_ic.dat");
if ( $result != 0 )
{
  print "Failed to translate TECPLOT to STD for $CIR2.mpde_ic.dat\n";
  $retcode = 2;
}
else
{
  $CMD="$XYCE_VERIFY --printline=mpde $TMPCIRFILE2 test_$CIR2.mpde_ic.dat $GOLD2.mpde_ic.prn > $CIR2.mpde_ic.dat.out 2> $CIR2.mpde_ic.dat.err";
  if (system("$CMD") != 0) {
      print STDERR "Verification failed on file test_$CIR2.mpde_ic.dat with $GOLD2.mpde_ic.prn, see $CIR2.mpde_ic.dat.err\n";
      $retcode = 2;
  }
}

$result = system("$TRANSLATE $CIR2.startup.dat");
if ( $result != 0 )
{
  print "Failed to translate TECPLOT to STD for $CIR2.startup.dat\n";
  $retcode = 2;
}
else
{
  $CMD="$XYCE_VERIFY --printline=mpde $TMPCIRFILE2 test_$CIR2.startup.dat $GOLD2.startup.prn > $CIR2.startup.dat.out 2> $CIR2.startup.dat.err";
  if (system("$CMD") != 0) {
      print STDERR "Verification failed on file test_$CIR2.startup.dat with $GOLD2.startup.prn, see $CIR2.startup.dat.err\n";
      $retcode = 2;
  }
}

$CMD="$XYCE_VERIFY $CIR2 $GOLD2.prn $CIR2.prn > $CIR2.prn.out 2> $CIR2.prn.err";
if (system("$CMD") != 0) {
    print STDERR "Verification failed on file $CIR2.prn, see $CIR2.prn.err\n";
    $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;

sub testTecplotHeaders {

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
  if ($zonecount =~ 1)
  {
    printf "Tecplot file %s contained correct number(%d) of zones.\n", $filename,$zonecount;
  }
  else
  {
    printf "Tecplot file %s contained wrong number(%d) of zones.\n", $filename,$zonecount;
    $retval = 2;
  }

   # check the number of DATASETAUXDATA outputs
  $datasetauxdatacount = `grep -ic datasetauxdata $filename 2>/dev/null`;
  if ($datasetauxdatacount =~ 1)
  {
    printf "Tecplot file %s contained correct number(%d) of datasetauxdata's.\n", $filename,$datasetauxdatacount;
  }
  else
  {
    printf "Tecplot file %s contained wrong number(%d) of datasetauxdata's.\n", $filename,$datasetauxdatacount;
    $retval = 2;
  }

  return $retval;
}
