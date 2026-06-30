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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# make name of gold .mt0 file
$GOLDMT0 = $GOLDPRN;
$GOLDMT0 =~ s/prn/mt0/;

# make BASE_NAME
$BASE_NAME = $CIRFILE;
$BASE_NAME =~ s/\.cir$//; # remove the .cir at the end.

# find root of Xyce build directory
$XYCE_BASE = $XYCE;
$XYCE_BASE =~ s/\/src\/Xyce$//;
$XYCE_BASE =~ s/\/bin\/Xyce$//;
print "XYCE_BASE = $XYCE_BASE\n";

# find path to where Xyce source code was.  This is a
# GIANT HACK, until I move from this shell-based approach
# testing to a make-based approach.
$XYCE_SRC_BASE = `grep --include Makefile "VPATH =" $XYCE_BASE/src/test/GenExtTestHarnesses/*`;
chomp($XYCE_SRC_BASE);
$XYCE_SRC_BASE =~ s/VPATH = //;
$XYCE_SRC_BASE =~s/\/src\/test\/GenExtTestHarnesses//;
print "XYCE_SRC_BASE = $XYCE_SRC_BASE\n";

# find the root of the verilog installation
$VERILOG_BASE = `which iverilog`;
chomp($VERILOG_BASE);
$VERILOG_BASE =~ s/\/bin\/iverilog//;
print "VERILOG_BASE = $VERILOG_BASE\n";
# remove files from previous runs
system("rm -f $CIRFILE.out $CIRFILE.prn $CIRFILE.mt0");
system("rm -f $BASE_NAME.o $BASE_NAME.vpi $BASE_NAME.vvp $BASENAME.gcc.* $BASE_NAME.icc.*");

# create vvp file, assuming gcc
print "Trying to make and run vvp file with gcc\n";
print "./create_vvp_file_w_gcc.sh $BASE_NAME $XYCE_BASE $VERILOG_BASE $XYCE_SRC_BASE >$BASE_NAME.gcc.out 2>$BASE_NAME.gcc.err\n";
$retval= system("./create_vvp_file_w_gcc.sh $BASE_NAME $XYCE_BASE $VERILOG_BASE $XYCE_SRC_BASE >$BASE_NAME.gcc.out 2>$BASE_NAME.gcc.err");
if ($retval != 0)
{
 print "iverilog failed to make $BASE_NAME.vvp file using gcc\n";
 print "Exit code = 1\n"; exit 1;
}

## Run the vvp program, first compiled with gcc. Re-try with icc, if that fails.
$retval = system("vvp -M. -m$BASE_NAME $BASE_NAME.vvp >$CIRFILE.out 2>$CIRFILE.err");
if ($retval != 0)
{
 print "$BASE_NAME.vvp file failed to execute, when made with gcc\n";
 print "retrying with icc\n";

 # re-try compilation of the vvp program with icc
 $retval= system("./create_vvp_file_w_icc.sh $BASE_NAME $XYCE_BASE $VERILOG_BASE $XYCE_SRC_BASE >$BASE_NAME.icc.out 2>$BASE_NAME.icc.err");
 if ($retval != 0)
 {
   print "iverilog failed to make $BASE_NAME.vvp file using icc\n";
   print "Exit code = 1\n"; exit 1;
 }

 # re-run the vvp program, now compiled with icc
 $retval = system("vvp -M. -m$BASE_NAME $BASE_NAME.vvp >$CIRFILE.out 2>$CIRFILE.err");
 if ($retval != 0)
 {
   print "$BASE_NAME.vvp file also failed to execute, when complied with icc\n";
   print "Exit code = 1\n"; exit 1;
 }
 else
 {
   printf "$BASE_NAME.vvp ran successfully, when compiled with icc\n";
 }
}
else
{
 printf "$BASE_NAME.vvp ran successfully, when compiled with gcc\n";
}

# We had a successful vvp run, either with gcc or icc.
# So, check for the existence of the output files.
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

# check for info in stdout
@searchstrings = ("Num ADCs and Max ADC Name Length: 2 9",
    "ADC YADC!ADC1 has width=1, R=1.00e\\+12, UVL=2.00e\\+00, LVL=0.00e\\+00, ST=5.00e-08",
    "ADC YADC!ADC2 has width=4, R=1.00e\\+12, UVL=2.00e\\+00, LVL=0.00e\\+00, ST=5.00e-08",
    "Widths are: 2 3");

if ( $Tools->checkError("$CIRFILE.out",@searchstrings) != 0)
{
 print "Failed to find all of the correct XyceCInterface information in stdout\n";
 $retval = 2;
}

# now check the correctness of the timeArray and voltageArray values
$fc = $XYCE_VERIFY;
$fc=~ s/xyce_verify/file_compare/;
$absTol=5e-3;
$relTol=1e-2;
$zeroTol=1e-8;

print ("Testing timeArray and voltageArray values\n");
$CMD="$fc $CIRFILE.TVarrayData $CIRFILE.TVarrayGoldData $absTol $relTol $zeroTol";
if (system($CMD) != 0)
{
  print STDERR "Verification failed on file $CIRFILE.TVarrayData";
  $retval = 2;
}

# print final exit code
print "Exit code = $retval\n"; exit $retval;
