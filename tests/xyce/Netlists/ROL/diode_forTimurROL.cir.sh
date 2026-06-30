#!/usr/bin/env perl

# This script generates a Xyce netlist that creates a large number of 
# normally distributed random numbers, then runs Xyce on it.  It scans
# the Xyce output for the reported random seed.

# It then re-runs Xyce with the "-randseed" option, and checks that the
# random output so generated is identical to the one produced previously.

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
$CIRFILE="diode_forTimurROL.cir";
$ROLOUTPUT=rol_output;

# Now run that netlist
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
if (system($CMD) != 0)
{
  `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
  $xyceexit=1;
}
elsif (not -s "$ROLOUTPUT.txt" )
{
  print "Exit code = 17\n";
  exit 17;
}
else
{
  if (-z "$CIRFILE.err" ) {`rm -f $CIRFILE.err`;}
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

# Gold standard values; could process these from a file but this will work for now.
$DA_IS_INIT=1e-10;
$DA_RS_INIT=0.5;
$DA_IS_SOLN=1e-12;
$DA_RS_SOLN=0.25;

$passed=1;

# Read in the ROL output file and find the solution values for DA:IS 
$DA_IS=`grep -m 2 'DA:IS' $ROLOUTPUT.txt`;
my @DA_IS_VALUES = split(' ', $DA_IS);
if ($DA_IS_VALUES[2] != $DA_IS_INIT)
{
  $passed=0;
}
if ($DA_IS_VALUES[5] != $DA_IS_SOLN)
{
  $passed=0;
}

# Read in the ROL output file and find the solution values for DA:RS
$DA_RS=`grep -m 2 'DA:RS' $ROLOUTPUT.txt`;
my @DA_RS_VALUES = split(' ', $DA_RS);
if ($DA_RS_VALUES[2] != $DA_RS_INIT)
{
  $passed=0;
}
if ($DA_RS_VALUES[5] != $DA_RS_SOLN)
{
  $passed=0;
}

if ($passed==1)
{
    print "Exit code = 0\n";
    exit 0;
}
else
{
    print "Exit code = 2\n";
    exit 2;
}

