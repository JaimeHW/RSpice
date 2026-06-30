#!/usr/bin/env perl

use XyceRegression::Tools;
$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

if (defined($verbose)) { $Tools->setVerbose(1); }

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
# output from comparison to go into $CIRFILE.raw.out and the STDERR output from
# comparison to go into $CIRFILE.raw.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$DASHOFILE="hbOutput";
$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

$TMPCIRFILE_TD="printLine_for_hb_td.cir";
$TMPCIRFILE_IC="printLine_for_hb_ic.cir";
$TMPCIRFILE_SU="printLine_for_hb_startup.cir";

# Remove the previous output files, including some that are only made if the
# previous run failed.
system("rm -f $CIRFILE.HB* $CIRFILE.hb* $CIRFILE.startup* $CIRFILE.err $CIRFILE.out");
system("rm -f $DASHOFILE* hbGrepOutput hbFoo");

# run Xyce
$CMD="$XYCE -o $DASHOFILE -delim COMMA $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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

# check for output files
$xyceexit=0;
if ( (-f "$CIRFILE.HB.FD.prn" ) || (-f "$CIRFILE.HB.FD.csv") ) {
  print STDERR "Extra output file $CIRFILE.HB.FD.prn or $CIRFILE.HB.FD.csv\n";
  $xyceexit=2;
}

if ( (-f "$CIRFILE.HB.TD.prn" ) || (-f "$CIRFILE.HB.TD.dat") ) {
  print STDERR "Extra output file $CIRFILE.HB.TD.prn or $CIRFILE.HB.TD.dat\n";
  $xyceexit=2;
}

if ( (-f "$CIRFILE.hb_ic.prn") || (-f "$CIRFILE.hb_ic.csv") ) {
  print STDERR "Extra output file $CIRFILE.hb_ic.prn or $CIRFILE.hb_ic.csv\n";
  $xyceexit=2;
}

if ( (-f "$CIRFILE.startup.prn") || (-f "$CIRFILE.startup.csv") ) {
  print STDERR "Extra output file $CIRFILE.startup.prn or $CIRFILE.startup.csv\n";
  $xyceexit=2;
}

if ( -f "hbFoo") {
  print STDERR "Extra output file hbFoo\n";
  $xyceexit=2;
}

if ( !(-f "$DASHOFILE.HB.FD.prn") ){
  print STDERR "Missing -o output file $DASHOFILE.HB.FD.prn\n";
  $xyceexit=14;
}

if ( !(-f "$DASHOFILE.HB.TD.prn") ){
  print STDERR "Missing -o output file $DASHOFILE.HB.TD.prn\n";
  $xyceexit=14;
}

if ( !(-f "$DASHOFILE.startup.prn") ){
  print STDERR "Missing -o output file $DASHOFILE.startup.prn\n";
  $xyceexit=14;
}

if ( !(-f "$DASHOFILE.hb_ic.prn") ){
  print STDERR "Missing -o output file $DASHOFILE.hb_ic.prn\n";
  $xyceexit=14;
}

if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# Now verify the output files.  Use file_compare.pl since I'm also testing
# print line concatenation and that the simulation footer is present.
$retcode=0;

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;
$abstol=1e-4;
$reltol=1e-3;
$zerotol=1e-6;

$CMD="$fc $DASHOFILE.HB.FD.prn $GOLDPRN.HB.FD.prn $abstol $reltol $zerotol > $DASHOFILE.HB.FD.prn.out 2> $DASHOFILE.HB.FD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.HB.FD.prn, see $DASHOFILE.HB.FD.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY $TMPCIRFILE_TD $DASHOFILE.HB.TD.prn $GOLDPRN.HB.TD.prn > $DASHOFILE.HB.TD.prn.out 2> $DASHOFILE.HB.TD.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.HB.TD.prn, see $DASHOFILE.HB.TD.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY $TMPCIRFILE_IC $DASHOFILE.hb_ic.prn $GOLDPRN.hb_ic.prn > $DASHOFILE.hb_ic.prn.out 2> $DASHOFILE.hb_ic.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.hb_ic.prn, see $DASHOFILE.hb_ic.prn.err\n";
    $retcode = 2;
}

$CMD="$XYCE_VERIFY $TMPCIRFILE_SU $DASHOFILE.startup.prn $GOLDPRN.startup.prn > $DASHOFILE.startup.prn.out 2> $DASHOFILE.startup.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.startup.prn, see $DASHOFILE.startup.prn.err\n";
    $retcode = 2;
}

# output files should not have any commas in them
if ( system("grep ',' $DASHOFILE.HB.FD.prn > hbGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.HB.FD.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

if ( system("grep ',' $DASHOFILE.HB.TD.prn > hbGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.HB.TD.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

if ( system("grep ',' hbOutput.hb_ic.prn > hbGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.hb_ic.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

if ( system("grep ',' hbOutput.startup.prn > hbGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.startup.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
