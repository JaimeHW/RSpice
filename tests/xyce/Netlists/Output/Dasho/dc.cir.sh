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

$DASHOFILE="dcOutput";
$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end

# Remove the previous output files, including some that are only made if the
# previous run failed.
system("rm -f $CIRFILE.prn $CIRFILE.csv $CIRFILE.err $CIRFILE.out");
system("rm -f $DASHOFILE* dcGrepOutput dcFoo*");

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

if ( (-f "$CIRFILE.prn") || (-f "$CIRFILE.csv") ) {
  print STDERR "Extra output file $CIRFILE.prn or $CIRFILE.csv\n";
  $xyceexit=2;
}

if ( -f "$CIRFILE.HOMOTOPY.prn") {
  print STDERR "Extra output file $CIRFILE.HOMOTOPY.prn\n";
  $xyceexit=2;
}

if ( (-f "dcFoo") || (-f "dcFoo1") ) {
  print STDERR "Extra output file dcFoo or dcFoo1\n";
  $xyceexit=2;
}

if ( !(-f "$DASHOFILE.prn") ){
  print STDERR "Missing -o output file for .PRINT DC, $DASHOFILE.prn\n";
  $xyceexit=14;
}

if ( !(-f "$DASHOFILE.HOMOTOPY.prn") ){
  print STDERR "Missing -o output file for .PRINT HOMOTOPY, $DASHOFILE.HOMOTOPY.prn\n";
  $xyceexit=14;
}

if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# Now verify the output .PRINT files.  Use file_compare.pl
# since I'm also testing print line concatenation
$retcode=0;

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;
$abstol=1e-4;
$reltol=1e-3;
$zerotol=1e-6;

$CMD="$fc $DASHOFILE.prn $GOLDPRN.prn $abstol $reltol $zerotol > $DASHOFILE.prn.out 2> $DASHOFILE.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.prn, see $DASHOFILE.prn.err\n";
    $retcode = 2;
}

$CMD="$fc $DASHOFILE.HOMOTOPY.prn $GOLDPRN.HOMOTOPY.prn $abstol $reltol $zerotol > $DASHOFILE.HOMOTOPY.prn.out 2> $DASHOFILE.HOMOTOPY.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.HOMOTOPY.prn, see $DASHOFILE.HOMOTOPY.prn.err\n";
    $retcode = 2;
}

# output files should not have any commas in them
if ( system("grep ',' $DASHOFILE.prn > dcGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

if ( system("grep ',' $DASHOFILE.HOMOTOPY.prn > tranGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.HOMOTOPY.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
