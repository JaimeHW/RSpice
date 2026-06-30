#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script (NEVER USED)
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10
# Otherwise we return the exit code of compare or xyce_verify.pl

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

`rm -f $CIRFILE.prn > /dev/null 2>&1`;
`rm -f $CIRFILE.err > /dev/null 2>&1`;
$CMD = "$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
if ( system("$CMD") != 0 ) 
{ 
  `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
  $xyceexit = 1;
}
else
{
  if ( -z "$CIRFILE.err" ) { `rm -f $CIRFILE.err`; }
}

if (defined($xyceexit)) 
{ 
    print "Exit code = 10\n";   # Xyce exited with error
    exit 10; 
}

# Check the output against the gold standard
$CMD="$XYCE_VERIFY --goodres=$CIRFILE.res.gs --testres=$CIRFILE.res $CIRFILE $GOLDPRN $CIRFILE".".prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
$result= system("$CMD");

if ($result != 0)
{
    print "Exit code = 2\n";
    print STDERR "Base netlist $CIRFILE failed to match gold standard.\n";
    exit 2;
}

`echo "Test passed!" >> $CIRFILE.prn.out`;
print "Exit code = 0\n";
exit 0;
