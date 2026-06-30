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
$XYCE_VERIFY=$ARGV[1];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# Now run that netlist
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
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

$CIRPRN=$CIRFILE;
$CIRPRN =~ s/\.cir/\.cir\.prn/g;

# Now read in the Xyce .OP std output and locate the model used by the 2 MOSFET instances
$correct_models_found=0;

$M1params =`grep 'BSIM4 instances:' -A 2 $CIRFILE.out `;
$M1params=~ s/\s+/ /g;

@list = split(' ',$M1params);

$size = scalar @list;

foreach my $i (0 .. $#list) {
  if ($list[$i] eq 'name')
  {
    if ($i < $size-1)
    {
      $firstInstanceFromList = $list[$i+1];
    }

    if ($i < $size-2)
    {
      $secondInstanceFromList = $list[$i+2];
    }

  }

  if ($list[$i] eq 'model')
  {
    if ($i < $size-1)
    {
      $firstModelFromList = $list[$i+1];
    }

    if ($i < $size-2)
    {
      $secondModelFromList = $list[$i+2];
    }
  }
}

if ($firstInstanceFromList eq 'X1:MN'  &&  $firstModelFromList eq 'NCH.2'
  && $secondInstanceFromList eq 'X1:MP'  &&  $secondModelFromList eq 'PCH.2' )
{ 
  $correct_models_found = 1;
} 

if ($correct_models_found == 1)
{
  # Now compare output with the gold standard 
  $CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2>$CIRFILE.prn.err";
  if (system("$CMD") != 0)
  {
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
  print "Exit code = 1\n";
  exit 2;
}


