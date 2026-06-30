#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script  (unused here)
# $ARGV[2] = location of compare script         (never used)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file  (unused here)

# This test runs three netlists that run exactly the same
# HB circuit, but output in three different formats, prn, csv and tecplot.
#
# All of the outputs are converted to prn using the convertToPrn2.py
# script.  The converted files are compared to each other, and should
# be diff-equivalent.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];

$CONVERT_TO_PRN=$XYCE_VERIFY;
$CONVERT_TO_PRN =~ s/xyce_verify.pl/convertToPrn2.py/;

$CIR2=$CIRFILE;
$CIR3=$CIRFILE;
$CIR2 =~ s/.cir/_prn.cir/;
$CIR3 =~ s/.cir/_tecp.cir/;

`rm -f $CIRFILE*.csv $CIR2*.prn $CIR3*.dat *_converted.prn`;
# This test simply makes sure Xyce doesn't fail on the main netlist, as
# it did in all released versions of Xyce prioer to 6.6.
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$sysret=system("$CMD");
if ($sysret!= 0) 
{
    if ($sysret & 127)
    {
        print "Exit code = 13\n"; printf STDERR "Xyce crashed with signal %d on file %s\n",($sysret&127),$CIRFILE; exit 13;
    }
    else
    {
        print "Exit code = 10\n"; printf STDERR "Xyce exited with exit code %d on %s\n",$sysret>>8,$CIRFILE; exit 10;
    }
}
if ( ! -f $CIRFILE.".HB.FD.csv")
{
    print "Exit code = 14\n";
    print STDERR "Xyce did not produce output files for $CIRFILE\n";
    exit 14;
}

$CMD="$XYCE $CIR2 > $CIR2.out 2>$CIR2.err";
$sysret=system("$CMD");
if ($sysret!= 0) 
{
    if ($sysret & 127)
    {
        print "Exit code = 13\n"; printf STDERR "Xyce crashed with signal %d on file %s\n",($sysret&127),$CIR2; exit 13;
    }
    else
    {
        print "Exit code = 10\n"; printf STDERR "Xyce exited with exit code %d on %s\n",$sysret>>8,$CIR2; exit 10;
    }
}

if (! -f $CIR2.".HB.FD.prn")
{
    print "Exit code = 14\n";
    print STDERR "Xyce did not produce output files for $CIR2\n";
    exit 14;
}

$CMD="$XYCE $CIR3 > $CIR3.out 2>$CIR3.err";
$sysret=system("$CMD");
if ($sysret!= 0) 
{
    if ($sysret & 127)
    {
        print "Exit code = 13\n"; printf STDERR "Xyce crashed with signal %d on file %s\n",($sysret&127),$CIR3; exit 13;
    }
    else
    {
        print "Exit code = 10\n"; printf STDERR "Xyce exited with exit code %d on %s\n",$sysret>>8,$CIR3; exit 10;
    }
}
if (! -f $CIR3.".HB.FD.dat")
{
    print "Exit code = 14\n";
    print STDERR "Xyce did not produce output files for $CIR3\n";
    exit 14;
}


# All three have run successfully.  Now start comparing data
# Convert the CSV outputs to PRN
foreach $outfile ($CIRFILE.".HB.FD.csv", $CIRFILE.".HB.TD.csv", $CIRFILE.".hb_ic.csv")
{
    $CMD="$CONVERT_TO_PRN $outfile";
    if (system("$CMD") != 0) { print "Exit code = 10\n"; print STDERR "Failed to convert $outfile to prn\n"; exit 10;}
}

# Convert the PRN outputs to a standardized PRN (to assure diff-equivalent formatting)
foreach $outfile ($CIR2.".HB.FD.prn", $CIR2.".HB.TD.prn", $CIR2.".hb_ic.prn")
{
    $CMD="$CONVERT_TO_PRN $outfile";
    if (system("$CMD") != 0) { print "Exit code = 10\n"; print STDERR "Failed to convert $outfile to prn\n"; exit 10;}
}

# Convert the Tecplot outputs to PRN 
foreach $outfile ($CIR3.".HB.FD.dat", $CIR3.".HB.TD.dat", $CIR3.".hb_ic.dat")
{
    $CMD="$CONVERT_TO_PRN $outfile";
    if (system("$CMD") != 0) { print "Exit code = 10\n"; print STDERR "Failed to convert $outfile to prn\n"; exit 10;}
}

# Now diff the HB.FD data
$CMD="diff $CIRFILE.HB.FD.csv_converted.prn $CIR2.HB.FD.prn_converted.prn";
if (system("$CMD") != 0) { print "Exit code = 2\n"; print STDERR "converted $CIRFILE.HB.FD.csv does not match convered $CIR2.HB.FD.prn\n"; exit 2;}

$CMD="diff $CIRFILE.HB.FD.csv_converted.prn $CIR3.HB.FD.dat_converted.prn";
if (system("$CMD") != 0) { print "Exit code = 2\n"; print STDERR "converted $CIRFILE.HB.FD.csv does not match convered $CIR3.HB.FD.dat\n"; exit 2;}

# Now diff the HB.TD data
$CMD="diff $CIRFILE.HB.TD.csv_converted.prn $CIR2.HB.TD.prn_converted.prn";
if (system("$CMD") != 0) { print "Exit code = 2\n"; print STDERR "converted $CIRFILE.HB.TD.csv does not match convered $CIR2.HB.TD.prn\n"; exit 2;}

$CMD="diff $CIRFILE.HB.TD.csv_converted.prn $CIR3.HB.TD.dat_converted.prn";
if (system("$CMD") != 0) { print "Exit code = 2\n"; print STDERR "converted $CIRFILE.HB.TD.csv does not match convered $CIR3.HB.TD.dat\n"; exit 2;}


# Now diff the hb_ic data
$CMD="diff $CIRFILE.hb_ic.csv_converted.prn $CIR2.hb_ic.prn_converted.prn";
if (system("$CMD") != 0) { print "Exit code = 2\n"; print STDERR "converted $CIRFILE.hb_ic.csv does not match convered $CIR2.hb_ic.prn\n"; exit 2;}

$CMD="diff $CIRFILE.hb_ic.csv_converted.prn $CIR3.hb_ic.dat_converted.prn";
if (system("$CMD") != 0) { print "Exit code = 2\n"; print STDERR "converted $CIRFILE.hb_ic.csv does not match convered $CIR3.hb_ic.dat\n"; exit 2;}
    
print "Exit code = 0\n"; 
exit 0; 

