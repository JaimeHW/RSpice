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
$GOLDRAW=$ARGV[4];

#$GOLDRAW =~ s/\.prn$//; # remove the .prn at the end.

$CIR1 = $CIRFILE;
$CIR2 = $CIRFILE;
$CIR2 =~ s/.cir/_ref.cir/;


system("rm -f $CIR1.raw $CIR1.err");
system("rm -f $CIR2.raw $CIR2.err");

####
$CMD1="$XYCE -r $CIR1.raw -a $CIR1  > $CIR1.out 2>$CIR1.err";

if (system($CMD1) != 0) {
    `echo "Xyce EXITED WITH ERROR! on $CIR1" >> $CIR1.err`;
    $xyceexit1=1;
}

if (defined ($xyceexit1)) {print "Exit code = 10\n"; exit 10;}

if ( !(-f "$CIR1.raw")) {
    print STDERR "Missing output file $CIR1.raw\n";
    $xyceexit1=14;
}
if (defined ($xyceexit1)) {print "Exit code = $xyceexit1\n"; exit $xyceexit1;}

####
$CMD2="$XYCE -r $CIR2.raw -a $CIR2  > $CIR2.out 2>$CIR2.err";

if (system($CMD2) != 0) {
    `echo "Xyce EXITED WITH ERROR! on $CIR2" >> $CIR2.err`;
    $xyceexit2=1;
}

if (defined ($xyceexit2)) {print "Exit code = 10\n"; exit 10;}

if ( !(-f "$CIR2.raw")) {
    print STDERR "Missing output file $CIR2.raw\n";
    $xyceexit2=14;
}
if (defined ($xyceexit2)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}

$retcode = 0;

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl

if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIR1 $GOLDRAW $CIR1.prn > $CIR1.prn.out 2>&1 $CIR1.prn.err"))
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

# using spice_read helps check the format.  
$XYCE_RAW_READ = $XYCE_VERIFY;
$XYCE_RAW_READ =~ s?xyce_verify\.pl?spice_read.py?;

if (system("$XYCE_RAW_READ $CIR1.raw | grep -v 'The file:' | grep -v 'Date:' > $CIR1.raw.info 2>$CIR1.raw.out") != 0) {
    print STDERR "spice_read failed on file $CIR1.raw, see $CIR1.raw.out\n";
    $retcode = 2;
}
if (system("$XYCE_RAW_READ $CIR2.raw | grep -v 'The file:' | grep -v 'Date:' > $CIR2.raw.info 2>$CIR2.raw.out") != 0) {
    print STDERR "spice_read failed on file $CIR2.raw, see $CIR2.raw.out\n";
    $retcode = 2;
}

$CMD3="diff -bi $CIR1.raw.info $CIR2.raw.info > $CIR1.raw.out";
if (system("$CMD3") != 0) {
    print STDERR "Verification failed on file $CIR1.raw, see $CIR1.raw.out\n";
    $retcode = 2;
}

# this is the part that really tests issue 888.  
# There should not be any "adjoint" fields in this raw file, since that format is not supported.
$CMD4="grep -i adj $CIR1.raw > $CIR1.raw.adj ";
if (system("$CMD4") != 0) {
    #  do nothing
}
else
{
  print STDERR "Verification failed on file $CIR1.raw, see $CIR1.raw.adj\n";
  $retcode = 2;
}


print "Exit code = $retcode\n"; exit $retcode;
