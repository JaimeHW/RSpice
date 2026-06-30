#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3]; 

$CIR1 = $CIRFILE;
$CIR2 = $CIRFILE;
$CIR2 =~ s/.cir/Gold.cir/;

`rm -f $CIR1.SENS.prn $CIR1*.out $CIR2.SENS.prn $CIR2*.out *.err`;
$CMD="$XYCE $CIR1 > $CIR1.out 2> $CIR1.err";
if (system($CMD) != 0)
{
    print "Xyce EXITED WITH ERROR! on $CIR1\n";
    $xyceexit=1;
}
else
{
    if (-z "$CIR1.err" ) {`rm -f $CIR1.err`;}
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

if ( ! -f "$CIR1.SENS.prn")
{
    print STDERR "$CIR1.SENS.prn not created\n";
    print "Exit code = 14\n";
    exit 14;
}

#Run the gold standard circuit
$CMD="$XYCE $CIR2 > $CIR2.out 2> $CIR2.err";
if (system($CMD) != 0)
{
    print "Xyce EXITED WITH ERROR! on $CIR2\n";
    $xyceexit=1;
}
else
{
    if (-z "$CIR2.err" ) {`rm -f $CIR2.err`;}
}

if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

if ( ! -f "$CIR2.SENS.prn")
{
    print STDERR "$CIR2.SENS.prn not created\n";
    print "Exit code = 14\n";
    exit 14;
}

$HEADER1=`head -1 $CIR1.SENS.prn`;
chomp $HEADER1;
print "$HEADER1\n";

$SEDCMD="sed '1 s-^.*\$-".$HEADER1."-g'";

$CMD1="$SEDCMD $CIR2.SENS.prn > $CIR2.tmpFile; mv $CIR2.tmpFile $CIR2.SENS.prn";  
system($CMD1);

$CMD="diff --strip-trailing-cr $CIR1.SENS.prn $CIR2.SENS.prn > $CIR1.SENS.prn.out 2> $CIR1.SENS.prn.err";

if (system($CMD) != 0)
{
  print "diff failed.  \n";

  print "Exit code = 2\n";
  exit 2;
}
else
{
  print "Exit code = 0\n";
  exit 0;
}

