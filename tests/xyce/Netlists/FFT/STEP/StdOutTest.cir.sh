#!/usr/bin/env perl

use XyceRegression::Tools;
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
# output from comparison to go into $CIRFILE.csv.out and the STDERR output from
# comparison to go into $CIRFILE.csv.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$CB = $CIRFILE;
$CB =~ s/.cir$//;

system("rm -f $CIRFILE.fft* $CIRFILE.errmsg $CIRFILE.nonstep.errmsg");
system("rm -f $CB.s*.cir.prn $CB.s*.cir.out $CB.s*.cir.fft0 $CB.s*.cir.err");

# create the output files for the Step netlist
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) {print "Exit code = $retval\n"; exit $retval;}

# Did we make both fft files for the step netlist
if ( !(-f "$CIRFILE.fft0")) {
    print STDERR "Missing output file $CIRFILE.fft0\n";
    print "Exit code = 14\n";
    exit 14;
}

if ( !(-f "$CIRFILE.fft1")) {
    print STDERR "Missing output file $CIRFILE.fft1\n";
    print "Exit code = 14\n";
    exit 14;
}

$retcode = 0;

# Make the non-step output files and check each non-step .fft0 output file against the
# corresponding .fftX output from the step netlist
foreach my $idx (0 .. 1)
{
  $NSF="$CB.s$idx.cir"; # name of the non-step file
  $retval=$Tools->wrapXyce($XYCE,"$NSF");
  if ($retval != 0)
  {
    print "Xyce crashed running non-step netlist $NSF for Step $idx\n";
    print "See $NSF.out and $NSF.err\n";
    print "Exit code = $retval\n";
    exit $retval;
  }

  # Did we make a .fft0 file for the non-step netlist
  if (not -s "$NSF.fft0" )
  {
    print "Failed to make file $NSF.fft0";
    $retcode = 17;
  }

  # also check for the .out file
  if (not -s "$NSF.out") {$retcode=17;}
}

if ($retcode !=0)
{
  print "Exit code = $retcode\n";
  exit $retcode;
}

# check that .out file for the .STEP netlist exists, and open it if it does
if (not -s "$CIRFILE.out")
{
  print "Exit code = 17\n";
  exit 17;
}
else
{
  open(NETLIST, "$CIRFILE.out");
  open(ERRMSG,">$CIRFILE.errmsg") or die $!;
}

# parse the .out file to find the text related to .FFT and .MEASURE
my $foundStart=0;
my $foundEnd=0;
my @outLine;
my $lineCount=0;
while( $line=<NETLIST> )
{
  if ($line =~ /FFT Analyses/) 
  { 
    $foundStart = 1;
    $foundEnd = 0; 
  }

  if ($foundStart > 0 && $line =~ /LASTMEASURE/ )
  {
    print ERRMSG $line;
    #print "Found last measure line\n";
    $foundEnd = 1; 
    $foundStart = 0;
  }
  elsif ( $foundStart > 0 && ( $line =~ /Solution Summary/ || $line =~ /Beginning DC/ ) )
  { 
    #print "Stopping on Solution Summary or Beginning DC line\n";
    $foundEnd = 1; 
    $foundStart = 0;
  }  
   
  if ($foundStart > 0 && $foundEnd < 1)
  {
    print ERRMSG $line;
  }
}
close(NETLIST);
close(ERRMSG);

# parse the non-step .out files to find the text related to .FFT and .MEASURE,
# combine the relevant sections into one file
open(ERRMSG,">$CIRFILE.nonstep.errmsg") or die $!;
foreach my $idx (0 .. 1)
{
  $NSF="$CB.s$idx.cir"; # name of the non-step file
  open(NETLIST, "$NSF.out");
  $foundStart=0;
  $foundEnd=0;
  $lineCount=0;
  while( $line=<NETLIST> )
  {
    if ($line =~ /FFT Analyses/) 
    { 
      $foundStart = 1;
      $foundEnd = 0; 
    }

    if ($foundStart > 0 && $line =~ /LASTMEASURE/ )
    {
      print ERRMSG $line;
      #print "Found last measure line\n";
      $foundEnd = 1; 
      $foundStart = 0;
    }
    elsif ( $foundStart > 0 && ( $line =~ /Solution Summary/ || $line =~ /Beginning DC/ ) )
   { 
      #print "Stopping on Solution Summary or Beginning DC line\n";
      $foundEnd = 1; 
      $foundStart = 0;
    }  
   
    if ($foundStart > 0 && $foundEnd < 1)
    {
      print ERRMSG $line;
    }
  }
  close(NETLIST);
}
close(ERRMSG);

$CMD="diff $CIRFILE.errmsg $CIRFILE.nonstep.errmsg";
$retval = system($CMD);
$retval = $retval >> 8;

# check the diff's return value
if ( $retval != 0 ) { $retcode = 2;}

print "Exit code = $retcode\n";
exit $retcode;
