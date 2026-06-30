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
# output from comparison to go into $CIRFILE.txt.out and the STDERR output from
# comparison to go into $CIRFILE.txt.err.

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDTXT=$ARGV[4];

$GOLDTXT =~ s/\.prn$//; # remove the .prn at the end.
$GOLDTXT = "$GOLDTXT.txt"; # add .txt

# clean up files from previous runs
system("rm -f $CIRFILE.out $CIRFILE.err $CIRFILE.names.txt $CIRFILE.noisenames.txt");
system("rm -f $CIRFILE.diff.out $CIRFILE.diff.err");

#run Xyce to make the namesfile and noisenamesfile, and exit on Xyce failure.
$CMD="$XYCE -namesfile $CIRFILE.names.txt -noise_names_file $CIRFILE.noisenames.txt $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$xyceexit = system($CMD);
if ($xyceexit != 0) 
{
  if ($xyceexit & 127)
  {
    print "Exit code = 13\n"; 
    printf STDERR "Xyce crashed with signal %d on file %s\n",($xyceexit&127),$CIRFILE; 
    exit 13;
  }
  else
  {
    print "Exit code = 10\n"; 
    printf STDERR "Xyce exited with exit code %d on %s\n",$xyceexit>>8,$CIRFILE; 
    exit 10;
  }
} 

# check that namesfile was made, but don't check its contents
if ( !(-f "$CIRFILE.names.txt")) {
    print STDERR "Missing output namesfile $CIRFILE.names.txt\nExit code = 14\n";
    exit 14;
}

# check that noisenamesfile was made, and that the gold noisenamesfile exists
if ( !(-f "$CIRFILE.noisenames.txt")) {
    print STDERR "Missing output noisenamesfile $CIRFILE.noisenames.txt\nExit code = 14\n";
    exit 14;
}

if ( !(-f "$GOLDTXT")) {
    print STDERR "Missing gold noisenamesfile $GOLDTXT\nExit code = 2\n";
    exit 2;
}

# assume success, and set $retval to 2 if either of the diffs fail.
$retval=0;

# make comparison files, for the "header" and "body" of the noise names file.  
# Check that header lines match exactly with the gold file.
$numHeaderLines=7;  # number of lines until line with the text "deviceName 	 noiseSource"
`head -7 $GOLDTXT > $CIRFILE.goldhdr.txt`;
`head -7 $CIRFILE.noisenames.txt > $CIRFILE.testhdr.txt`;
$CMD="diff -b $CIRFILE.testhdr.txt $CIRFILE.goldhdr.txt> $CIRFILE.diff.out 2>$CIRFILE.diff.err";
$retval=system($CMD);
$retval = $retval >> 8;
if ($retval!=0)
{
  print STDERR "Diff failed on header portion of $CIRFILE.noisenames.txt. See $CIRFILE.diff.out\nExit code = 2\n";
  exit 2;
}  

# need to sort both files because the ordering, of the device-specific info, may differ 
# between serial and parallel.
`tail -n +8 $GOLDTXT | sort -d > $CIRFILE.goldbody.txt`;
`tail -n +8 $CIRFILE.noisenames.txt | sort -d > $CIRFILE.testbody.txt`;
$CMD="diff -b $CIRFILE.testbody.txt $CIRFILE.goldbody.txt> $CIRFILE.diff.out 2>$CIRFILE.diff.err";
$retval=system($CMD);
$retval = $retval >> 8;
if ($retval!=0)
{
  print STDERR "Diff failed on device-info portion of $CIRFILE.noisenames.txt. See $CIRFILE.diff.out\nExit code = 2\n";
  exit 2;
}  

print "Exit code = $retval\n"; exit $retval;
