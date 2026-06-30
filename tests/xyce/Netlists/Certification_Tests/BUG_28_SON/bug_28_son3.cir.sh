#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard dat file


# This script checks bug_28_son3.cir to make sure it runs without error.
# It then runs bug_28_son3_noparams.cir to make sure that the first circuit
# produced identical results.

# Prior to the final fix to bug 28(SON) the first circuit gave a parameter
# resolution error.
# During a  partial fix to that error, the circuit ran, but produced
# incorrect results.
# By comparing to a version that doesn't use parameters at all, we assure
# that both components of the fix are still working

# The third case tested here uses global params instead of subcircuit-local
# params.  It didn't work either, and should give results identical to the
# "noparams" version.
$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

$XPLAT_DIFF = $XYCE_VERIFY;
$XPLAT_DIFF =~ s/xyce_verify/xplat_diff/;

$CIRFILE2=$CIRFILE;
$CIRFILE2 =~ s/.cir/_noparams.cir/;

$CIRFILE3=$CIRFILE;
$CIRFILE3 =~ s/.cir/_globalp.cir/; 

$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
$returnval=system("$CMD");

if ($returnval != 0)
{
    $xyceexit=$returnval >> 8;
    $signal=$returnval & 255;

    if ($signal !=0)
    {
        print STDERR "Xyce crashed on $CIRFILE.  Exit code of Xyce is $xyceexit\n";
        print STDERR "Xyce crashed with signal $signal\n";
        print "Exit code = 13\n" ;    exit 13;
    }
    else
    {
        print STDERR "Xyce exited with error on $CIRFILE.  Exit code of Xyce is $xyceexit\n";
        print "Exit code = 10\n" ;    exit 10;
    }
}

$CMD="$XYCE $CIRFILE2 > $CIRFILE2.out 2> $CIRFILE2.err";
$returnval=system("$CMD");

if ($returnval != 0)
{
    $xyceexit=$returnval >> 8;
    $signal=$returnval & 255;

    if ($signal !=0)
    {
        print STDERR "Xyce crashed on $CIRFILE2.  Exit code of Xyce is $xyceexit\n";
        print STDERR "Xyce crashed with signal $signal\n";
        print "Exit code = 13\n" ;    exit 13;
    }
    else
    {
        print STDERR "Xyce exited with error on $CIRFILE2.  Exit code of Xyce is $xyceexit\n";
        print "Exit code = 10\n" ;    exit 10;
    }
}

$CMD="$XYCE $CIRFILE3 > $CIRFILE3.out 2> $CIRFILE3.err";
$returnval=system("$CMD");

if ($returnval != 0)
{
    $xyceexit=$returnval >> 8;
    $signal=$returnval & 255;

    if ($signal !=0)
    {
        print STDERR "Xyce crashed on $CIRFILE3.  Exit code of Xyce is $xyceexit\n";
        print STDERR "Xyce crashed with signal $signal\n";
        print "Exit code = 13\n" ;    exit 13;
    }
    else
    {
        print STDERR "Xyce exited with error on $CIRFILE3.  Exit code of Xyce is $xyceexit\n";
        print "Exit code = 10\n" ;    exit 10;
    }
}

$CMD_DIFF="$XPLAT_DIFF $CIRFILE.prn $CIRFILE2.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
if (system("$CMD_DIFF") != 0) 
{ 
    print STDERR "$CMD_DIFF failed\n";
    $CMD = "$XYCE_VERIFY $CIRFILE $CIRFILE.prn $CIRFILE2.prn >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
    `echo "diff failed, now trying xyce_verify" >> $CIRFILE.prn.err`;
    `echo "$CMD" >> $CIRFILE.prn.err`;
    if ( system("$CMD") != 0 )
    { 
      `echo "Test Failed!" >> $CIRFILE.prn.out`;
      print "Exit code = 2\n";
      exit 2;
    }
}


$CMD_DIFF="$XPLAT_DIFF $CIRFILE3.prn $CIRFILE2.prn > $CIRFILE3.prn.out 2> $CIRFILE3.prn.err";
if (system("$CMD_DIFF") != 0) 
{ 
    print STDERR "$CMD_DIFF failed\n";
    $CMD = "$XYCE_VERIFY $CIRFILE3 $CIRFILE3.prn $CIRFILE2.prn >> $CIRFILE3.prn.out 2>> $CIRFILE3.prn.err";
    `echo "diff failed, now trying xyce_verify" >> $CIRFILE3.prn.err`;
    `echo "$CMD" >> $CIRFILE3.prn.err`;
    if ( system("$CMD") != 0 )
    { 
      `echo "Test Failed!" >> $CIRFILE3.prn.out`;
      print "Exit code = 2\n";
      exit 2;
    }
    else
    {
        print "Exit code = 0\n";
        exit 0;
    }
}
else
{
    print "Exit code = 0\n";
    exit 0;
}


    
    
