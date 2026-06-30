#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are: 
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];
$DIAGFILE=$CIRFILE;
$DIAGFILE=~s/\..*$/.dia/;

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }
if (not -s "$DIAGFILE" ) 
{ 
  print "Diagnostic File not generated\n Exit code = 19\n"; 
  exit 19; 
}

# open the diagnostic file and verify that the reported nodes 
# have values over 6.0 which was the limit used in the simulation file
open( DIAGF, '<', $DIAGFILE) or die $!;

my $blockType=' ';
my $i = 0;
while( <DIAGF> )
{
  $aLine = $_;
  chomp($aLine);
  # remove any leading and trailing whitespace
  $aLine =~ s/^\s+|\s+$//g;
  my @word = split(/\s+/, $aLine);
  # try splitting the last word into varName = value 
  my @vals = split(/=/, $word[$#word] );
  
  # since the diagnostic output blocks take up multiple lines figure out 
  # if this is an extreme, voltage or current block.
  if( $word[0] eq "Extreme")
  {
    $blockType = 'E';
    next;
  }
  elsif ( $word[0] eq "Voltage")
  {
    $blockType = 'V';
    next;
  }
  elsif( $word[0] eq "Current")
  {
    $blockType = 'I';
    next;
  }
  elsif( $word[0] eq "Analysis")
  {
    $blockType = 'S';
    next;
  }
  
  if( ($blockType eq 'E') && ($#word < 3))
  {
    $retval = 2;
    print( "Found a extrema output block when there shouldn't be one: $blocktype $word[0] \n");
    print(  "Exit code = $retval\n" );
    exit 2;
  }
  if( $blockType eq 'V')
  {
    if( abs($vals[$#vals]) < 5.0 )
    {
      $retval = 2;
      print( "Found a value under the limit used in the circuit: $word[$#word]\n");
      print(  "Exit code = $retval\n" );
      exit 2;
    }
  }
  if( $blockType eq 'I')
  {
    $retval = 2;
    print( "Found a current output block when there shouldn't be one: $word[0] $word[$#word]\n");
    print(  "Exit code = $retval\n" );
    exit 2;
  }
  $i = $i + 1;
}

# check that we actually looked at more than just the header of the diagnostic file.
if( $i < 2)
{
  # due to some sort of error we didn't really process any lines in the diagnostic file
  # this should be treated as an error 
  $retval = 2;
  print( "Couldn't process any lines in the diagnostic file\n");
  print(  "Exit code = $retval\n" );
  exit 2;
}

if ($retval != 0) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}
print "Exit code = 0\n"; 
exit 0;


