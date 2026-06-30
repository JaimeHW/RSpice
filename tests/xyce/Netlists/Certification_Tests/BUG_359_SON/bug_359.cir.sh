#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# This test case is designed to check the behavior of "VSRCSCALE" continuation
# We run a simple circuit with continuation options.  The test passes if
# the ".HOMOTOPY.prn" file shows correct scaling of all DC sources in the
# circuit as the continuation proceeds from VSRCSCALE=0 to VSRCSCALE=1

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];  # The input circuit file is an empty, placeholder file
$GOLDPRN=$ARGV[4];

# we'll use a modified name for creating our real netlist
$CIRFILE =~ s/.cir/_real.cir/;    

#clean up from any previous run

`rm -f $CIRFILE *.prn *.res`;
open(TheNetlist, ">$CIRFILE") || die "Cannot open $CIRFILE for output.";

$V1DCVoltage = 1;
$V2DCVoltage = 8;
$V1LowVoltage = 1;
$V1HighVoltage = 8;
$V1VoltageStep = 1;

print TheNetlist "*Test of VSRCSCALE continuation\n";
print TheNetlist "*This source will be swept with .DC\n";
print TheNetlist "V1 1 0 DC $V1DCVoltage\n";
print TheNetlist "R1 1 0 100\n";
print TheNetlist "*This source will be not swept with .DC\n";
print TheNetlist "V2 2 0 DC $V2DCVoltage\n";
print TheNetlist "R2 2 0 100\n";
print TheNetlist ".DC V1 $V1LowVoltage $V1HighVoltage $V1VoltageStep\n";
print TheNetlist ".print DC V(1) V(2)\n";
print TheNetlist ".print HOMOTOPY V(1) V(2)\n";

print TheNetlist "************************************\n";
print TheNetlist "* **** Start Homotopy Setup ****\n";
print TheNetlist "************************************\n";
print TheNetlist ".options nonlin continuation=1\n";

print TheNetlist ".options loca stepper=0 predictor=0 stepcontrol=0\n";
print TheNetlist "+ conparam=vsrcscale\n";
print TheNetlist "+ initialvalue=0.0 minvalue=-1.0 maxvalue=1.0\n";
print TheNetlist "+ initialstepsize=0.2 minstepsize=1.0e-4 maxstepsize=0.2\n";
print TheNetlist "+ aggressiveness=1.0\n";
print TheNetlist "+ maxsteps=5000 maxnliters=200\n";
print TheNetlist "**********************************\n";
print TheNetlist "* **** End Homotopy Setup ****\n";
print TheNetlist "**********************************\n";
print TheNetlist ".END\n";
close(TheNetlist);

# Now run Xyce on the netlist
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
if (system($CMD) != 0)
    {
        `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
        $xyceexit=1;
    }
else
    {
        if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
    }

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}


# Xyce should have produced a .HOMOTOPY.prn file showing how the continuation
# was carried out.
if ((! -e "$CIRFILE.HOMOTOPY.prn") || (-z "$CIRFILE.HOMOTOPY.prn")) 
{
    print STDERR "Xyce did not produce .HOMOTOPY file\n";
    $exitcode=14;
}
else
{
    # Open the HOMOTOPY.prn file and scan its contents for correctness
    open(TheHomotopyFile, "<$CIRFILE.HOMOTOPY.prn") || die "Cannot open $CIRFILE.HOMOTOPY.prn for input";
    $CurrentV1 = $V1LowVoltage-$V1VoltageStep;
    $exitcode=0;
    while (<TheHomotopyFile>)
    {
        @Fields=split(" ");
        next if ($Fields[0] eq "Index" || $Fields[0] eq "End");
        if ($Fields[1] == 0)
        {
            $CurrentV1 += $V1VoltageStep;
            print STDERR "Next .DC step, V1 voltage is $CurrentV1\n";
        }
        $ExpectedV1=$CurrentV1*$Fields[1];
        $ExpectedV2=$V2DCVoltage*$Fields[1];
        # Fields[2] should be V1*VSRCSCALE ($Fields[1])
        if (abs($Fields[2]-$ExpectedV1) > 1e-12)
        {
            $exitcode=2;
            print STDERR "Index $Fields[0]: $Fields[2] != $ExpectedV1\n";
            last;
        }
        if (abs($Fields[3]-$ExpectedV2) > 1e-12)
        {
            $exitcode=2;
            print STDERR "Index $Fields[0]: $Fields[3] != $ExpectedV2\n";
            last;
        }
    }
    close(TheHomotopyFile);
}

print "Exit code = $exitcode\n";
exit $exitcode;

