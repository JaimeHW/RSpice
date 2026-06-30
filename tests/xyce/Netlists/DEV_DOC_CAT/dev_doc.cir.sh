#!/usr/bin/env perl

use XyceRegression::Tools;

sub dirname($) {my $file = shift; $file =~ s!/?[^/]*/*$!!; return $file; }

$Tools = XyceRegression::Tools->new();

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
$GOLDTEX=$ARGV[4];

$GOLDDIR = dirname($GOLDTEX);
$GOLDTEX =~ s/\.prn$//; # remove the .prn at the end.

$dirname=`dirname $XYCE_VERIFY`;
chomp($dirname);
$XPLAT_DIFF="$dirname/xplat_diff.pl";

system("rm -f $CIRFILE.tex $CIRFILE.err");

$CMD="$XYCE -doc_cat > $CIRFILE.out 2>$CIRFILE.err";
if (system($CMD) != 0) {
    `echo "Xyce EXITED WITH ERROR! on -doc_cat" >> $CIRFILE.err`;
    $xyceexit=1;
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

@tables_to_compare=("C_1_Device_Instance_Params.tex", "M_10_Device_Instance_Params.tex", "M_10_Device_Model_Params.tex");
$retcode = 0;

foreach $table (@tables_to_compare)
{
    if ( !(-f $table)) 
    {
        print STDERR "Missing output file $table\n";
        $xyceexit=14;
    }

    if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

    $CMD="$XPLAT_DIFF $table $GOLDDIR/$table > $table".".out";
    if (system("$CMD") != 0) {
        print STDERR "Verification failed on file $table, see $table".".out\n";
        $retcode = 2;
    }
}

print "Exit code = $retcode\n"; exit $retcode;
