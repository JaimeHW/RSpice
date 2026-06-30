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
# $ARGV[4] = location of gold standard dat file

# If Xyce does not produce a dat file, then we return exit code 10.
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
$GOLDPRN=$ARGV[4];

$XPLAT_DIFF = $XYCE_VERIFY;
$XPLAT_DIFF =~ s/xyce_verify/xplat_diff/;

$CIR = "$CIRFILE";
$RES = "$CIR.res";
$PRN = "$CIR.dat";
$RESGS  = "$RES.gs";
$RESGSPL = "$RESGS.pl";
$PRNGS = "$PRN.gs";
$PRNGSPL = "$PRNGS.pl";


$CMD="$XYCE $CIR > $CIR.out 2> $CIR.err";
$returnval=system("$CMD");

if ($returnval != 0)
{
    $xyceexit=$returnval >> 8;
    $signal=$returnval & 255;

    print STDERR "Xyce crashed.  Exit code of Xyce is $xyceexit\n" if ($xyceexit!=0);
    print STDERR "Xyce crashed with signal $signal\n" if ($signal != 0);
    print "Exit code = 13\n";    exit 13;
}

# fix up line endings and exponent problems on dat file
open (INFILE, "<$CIR.dat");
open (OUTFILE, ">$CIR.bak");
while(<INFILE>)
{
  s/\r//g;                                    # line ending fix
  s/(-*\d\.\d+e[\+-])0([0-9][0-9])/ \1\2/g;   # number fix
  s/[ \t]+/ /g;                               # whitespace fix
  s/^ //;                                     # the gold standard doesn't lead with a space
  print OUTFILE; 
}
close(INFILE);
close(OUTFILE);
rename("$CIR.bak", "$CIR.dat");


# generate the *.res gold standard
if (system("./$RESGSPL") != 0) { $failure=1; print STDERR "resgspl failed\n";}

# diff the *.res with the res gold standard 
$CMD_DIFF="$XPLAT_DIFF $RES $RESGS > $RES.out 2> $RES.err";
if (system("$CMD_DIFF") != 0) { $failure=1; print STDERR "$CMD_DIFF failed\n";}



# generate the *.dat gold standard
if (system("./$PRNGSPL") != 0) { $failure=1; print STDERR "prngspl failed\n";}


# normalize whitespace as it's platform variable
open (INFILE, "<bug_460.cir.dat.gs");
open (OUTFILE, ">temp.bak");
while(<INFILE>)
{
  s/[ \t]+/ /g;                               # whitespace fix
  print OUTFILE;
}
close(INFILE);
close(OUTFILE);
rename("temp.bak", "bug_460.cir.dat.gs");

# diff the *.dat with the dat gold standard 
#$CMD_DIFF2="diff  $PRN $PRNGS > $PRN.out 2> $PRN.err";
#if (system("$CMD_DIFF2") != 0) { $failure=1; }
$CMD_VERIFY="$XYCE_VERIFY $CIR $PRNGS $PRN";
if (system("$CMD_VERIFY") != 0) { $failure=1; print STDERR "cmd_verify failed\n";}


if (defined $failure)
{
  print "Exit code = 2\n"; exit 2;
}
else
{
  print "Exit code = 0\n"; exit 0;
}

print "Exit code = 1\n"; exit 1;
