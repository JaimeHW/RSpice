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

use Scalar::Util qw(looks_like_number);

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# Comparison tolerances. 
# phase is output in degrees and for Fourier components with very small magnitude, 
# it can be have several degrees of scatter.  Thus it gets its own abstol
my $absTol = 2.0e-6;
my $phaseAbsTol = 1.0;
my $relTol = 0.02;
my $zeroTol = 1.0e-8;

# change ending on gold standard file from ".prn" as passed in to ".gs" 
# which is the file storing the Fourier results.

$GOLDPRN =~ s/prn$/gs/;
$FOURFILE = $CIRFILE;

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$FOURFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$FOURFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl

if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $FOURFILE junk $FOURFILE.prn > $FOURFILE.prn.out 2>&1 $FOURFILE.prn.err"))
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

#
# Did we make a Fourier file
#
if (not -s "$FOURFILE.four0" ) { print "Exit code = 17\n"; exit 17; }

# Now look for the Fourier output file and compare it to a 
# gold standard line by line.
#
open(RESULTS2, "$FOURFILE.four0");
open(GOLD_STD, $GOLDPRN);

# Advance each of these files to where the results are, bypass the header information.
$retval = 2;
while( $line2=<RESULTS2> )
{
  # process a line into text and values.
  chop $line2;
  # Remove leading spaces on line, otherwise the spaces become 
  # element 0 of "@lineOfDataFromXyce" instead of the first column of data.
  $line2 =~ s/^\s*//;
  @lineOfDataFromXyce = (split(/[\s,]+/, $line2));
  if ( $lineOfDataFromXyce[0] eq 'Harmonic' )
  {
    $retval = 0;
    last;
  }
}
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

$retval = 2;
while( $line_gs=<GOLD_STD> )
{
  # process a line into text and values.
  chop $line_gs;
  # Remove leading spaces on line, otherwise the spaces become 
  # element 0 of "@lineOfDataFromXyce" instead of the first column of data.
  $line_gs =~ s/^\s*//;
  @lineOfDataFromXyce = (split(/[\s,]+/, $line_gs));
  if ( $lineOfDataFromXyce[0] eq 'Harmonic' )
  {
    $retval = 0;
    last;
  }
}
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }

# Now compare data from this point on.
while( ($line2=<RESULTS2>) && ($line_gs=<GOLD_STD>) )
{
  # process a line into text and values.
  chop $line2;
  # Remove leading spaces on line, otherwise the spaces become 
  # element 0 of "@lineOfDataFromXyce" instead of the first column of data.
  $line2 =~ s/^\s*//;
  @line2OfDataFromXyce = (split(/[\s,]+/, $line2));
  
  # process a line_gs into text and values.
  chop $line_gs;
  $line_gs =~ s/^\s*//;
  @gsLineOfDataFromXyce = (split(/[\s,]+/, $line_gs));
  
  if( ($#line2OfDataFromXyce != $#gsLineOfDataFromXyce) )
  {
    print "Xyce's Fourier file doesn't match the gold standard\n";
    print "Xyce's Fourier output: $line2\n";
    print "Gold Standard: $line_gs\n";
    $retval=2;
  }
  else
  {
    # the two files have the same number of items on a line.  
    # compare individual values as scalars  This will have the 
    # effect of 
    for( $i=0; $i<=$#line2OfDataFromXyce; $i++ )
    {
      if (looks_like_number($line2OfDataFromXyce[$i]) && looks_like_number($gsLineOfDataFromXyce[$i] ) )
      {
        if( (abs( $line2OfDataFromXyce[$i] ) < $zeroTol) &&  (abs( $gsLineOfDataFromXyce[$i] ) < $zeroTol ))
        {
           #print "number compare below zeroTol $lineOfDataFromXyce[$i] , $gsLineOfDataFromXyce[$i] ok\n";
        }
        elsif( (($i==3) || ($i==5)) && (abs( $line2OfDataFromXyce[$i] - $gsLineOfDataFromXyce[$i] ) < $phaseAbsTol )) 
        {
           # phase needs different handling
           #print "Phase comparison passed $lineOfDataFromXyce[$i] , $gsLineOfDataFromXyce[$i]\n";
        }
        #elsif( (abs( $line2OfDataFromXyce[$i] - $gsLineOfDataFromXyce[$i] ) < $absTol )) 
        #{
           # regular compare
        #}
        elsif (   (abs(($line2OfDataFromXyce[$i] - $gsLineOfDataFromXyce[$i])/$gsLineOfDataFromXyce[$i]) ) < $relTol ) 
        {
           # regular compare, using reltol.  
        }
        else
        {
          print "$FOURFILE.four0 and $GOLDPRN found a numeric difference\n";
          print "Fourier data \"$line2OfDataFromXyce[$i]\" to GS data \"$gsLineOfDataFromXyce[$i]\" as ";
          print "Failed numeric compare\n";
          $retval=2;
          last;
        }
      }
      elsif( ($line2OfDataFromXyce[$i] eq $gsLineOfDataFromXyce[$i]) )
      {
        # same in string context so ok.
        print "string compare of $line2OfDataFromXyce[$i] and $gsLineOfDataFromXyce[$i]ok\n";
      }
      else
      {
        print "Elements failed compare:\n";
        print "Xyce Fourier produced: \"$line2OfDataFromXyce[$i]\"\n";
        print "Gold standard: \"$gsLineOfDataFromXyce[$i]\"\n";
        $retval=2;
        last;
      }
    }
  }
}
close(RESULTS2);
close(GOLD_STD);

if ( $retval != 0 )
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

print "Exit code = $retval\n"; 
exit $retval;

