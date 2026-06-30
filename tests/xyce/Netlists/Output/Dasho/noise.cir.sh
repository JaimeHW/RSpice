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
# output from comparison to go into $CIRFILE.raw.out and the STDERR output from
# comparison to go into $CIRFILE.raw.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$DASHOFILE="noiseOutput";
$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

# Remove the previous output files, including some that are only made if the
# previous run failed.
system("rm -f $CIRFILE.NOISE* $CIRFILE.err $CIRFILE.out");
system("rm -f $DASHOFILE* noiseGrepOutput noiseFoo");

# run Xyce
$CMD="$XYCE -o $DASHOFILE.NOISE.prn $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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
if ( (-f "$CIRFILE.NOISE.prn") || (-f "$CIRFILE.NOISE.csv") ) {
  print STDERR "Extra output file $CIRFILE.NOISE.prn or $CIRFILE.NOISE.csv\n";
  $xyceexit=2;
}

if ( -f "noiseFoo") {
  print STDERR "Extra output file noiseFoo\n";
  $xyceexit=2;
}

if ( !(-f "$DASHOFILE.NOISE.prn") ){
  print STDERR "Missing -o output file for .PRINT NOISE, $DASHOFILE.noise.prn\n";
  $xyceexit=14;
}

if ($xyceexit!=0) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

# do a valgrind run, if that's been requested
if ($XYCE_VERIFY =~ m/valgrind_check/)
{
  print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
  if (system("$XYCE_VERIFY $CIRFILE junk $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
  {
    #valgrind_check.sh has reported a memory error, test is a failure
    print "Exit code = 2 \n";
    exit 2;
  }
  else
  {
    #valgrind_check.sh has reported no memory errors, pass
    print "Exit code = 0 \n";
    exit 0;
  }
}

# Now verify the output file.  Use file_compare.pl since I'm also testing print
# line concatenation and that the simulation footer is present.
$retcode=0;

$fc=$XYCE_VERIFY;
$fc =~ s/xyce_verify/file_compare/;
$abstol=1e-4;
$reltol=1e-3;
$zerotol=1e-6;

$CMD="$fc $DASHOFILE.NOISE.prn $GOLDPRN.NOISE.prn $abstol $reltol $zerotol > $DASHOFILE.NOISE.prn.out 2> $DASHOFILE.NOISE.prn.err";
if (system($CMD) != 0) {
    print STDERR "Verification failed on file $DASHOFILE.NOISE.prn, see $DASHOFILE.NOISE.prn.err\n";
    $retcode = 2;
}

# output file should not have any commas in it
if ( system("grep ',' $DASHOFILE.NOISE.prn > noiseGrepOutput") == 0)
{
  print STDERR "Verification failed on file $DASHOFILE.NOISE.prn.  It should not have any commas in it\n";
  $retcode = 2;
}

print "Exit code = $retcode\n"; exit $retcode;
