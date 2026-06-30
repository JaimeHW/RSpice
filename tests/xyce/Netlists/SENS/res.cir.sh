#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script  (This is NEVER used and can be ignored)
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

# Comparison tolerance
my $absTol = 1.0e-8;

# change ending on gold standard file from ".prn" as passed in to ".gs" 
# which is the file storing the measure results.
$GOLDSENS = $CIRFILE . "_dodp.txt.gs";
$ADJOINTFILE = $CIRFILE . "0_dodpAdjoint.txt";
$DIRECTFILE = $CIRFILE . "0_dodpDirect.txt";

$retval = -1;
$retval=$Tools->wrapXyce($XYCE,$CIRFILE);
if ($retval != 0) { print "Exit code = $retval\n"; exit $retval; }
if (not -s "$CIRFILE.prn" ) { print "Exit code = 14\n"; exit 14; }

#If this is a VALGRIND run, we don't do our normal verification, we
# merely run "valgrind_check.sh" as if it were xyce_verify.pl

if ($XYCE_VERIFY =~ m/valgrind_check/)
{
    print STDERR "DOING VALGRIND RUN INSTEAD OF REAL RUN!";
    if (system("$XYCE_VERIFY $CIRFILE junk $CIRFILE.prn > $CIRFILE.prn.out 2>&1 $CIRFILE.prn.err"))
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
# Did we make a adjoint file
#
if (not -s "$ADJOINTFILE" ) { print "Exit code = 17\n"; exit 17; }

#
# Did we make a direct file
#
if (not -s "$DIRECTFILE" ) { print "Exit code = 17\n"; exit 17; }


# check that the files have the same number of lines
open(ADJRESULTS, "$ADJOINTFILE");
open(DIRRESULTS, "$DIRECTFILE");
open(GOLD_STD, $GOLDSENS);
for ($adjCount=0; <ADJRESULTS>; $adjCount++) { }
for ($dirCount=0; <DIRRESULTS>; $dirCount++) { }
for ($gs_Count=0; <GOLD_STD>; $gs_Count++) { }
close(ADJRESULTS);
close(DIRRESULTS);
close(GOLD_STD);

if( ($adjCount != $gs_Count) || ($dirCount != $gs_Count) )
{
  print "Xyce's Direct or Adjoint file doesn't match the gold standard\n";
  print "Xyce's Direct line count: $dirCount\n";
  print "Xyce's Adjoint line count: $adjCount\n";
  print "Gold Standard line count: $gs_Count\n";
  $retval=2;
}
else
{
  # If the line counts match, then compare in detail.
  # Re-open the adjoint and direct output files and compare them to the
  # gold standard line by line.
  open(ADJRESULTS, "$ADJOINTFILE");
  open(DIRRESULTS, "$DIRECTFILE");
  open(GOLD_STD, $GOLDSENS);

  # Compare data 
  while( ($lineAdj=<ADJRESULTS>) && ($lineDir=<DIRRESULTS>) && ($line_gs=<GOLD_STD>) )
  {
    # process a line into text and values.
    chop $lineAdj;
    # Remove leading spaces on lineAdj, otherwise the spaces become 
    # element 0 of "@adjointXyceData" instead of the first column of data.
    $lineAdj =~ s/^\s*//;
    @adjointXyceData = (split(/[\s,]+/, $lineAdj));

    # process a line into text and values.
    chop $lineDir;
    # Remove leading spaces on line, otherwise the spaces become 
    # element 0 of "@adjointXyceData" instead of the first column of data.
    $lineDir =~ s/^\s*//;
    @directXyceData = (split(/[\s,]+/, $lineDir));

    # process a line_gs into text and values.
    chop $line_gs;
    $line_gs =~ s/^\s*//;
    @gsLineOfDataFromXyce = (split(/[\s,]+/, $line_gs));

    if( ($#adjointXyceData != $#gsLineOfDataFromXyce) || ($#directXyceData != $#gsLineOfDataFromXyce) )
    {
      print "Xyce's Direct or Adjoint file doesn't match the gold standard\n";
      print "Xyce's Direct output: $lineDir\n";
      print "Xyce's Adjoint output: $lineAdj\n";
      print "Gold Standard: $line_gs\n";
      $retval=2;
    }
    else
    {
      # the two files have the same number of items on a line.  
      # compare individual values as scalars  This will have the 
      # effect of 
      for( $i=0; $i<=$#adjointXyceData; $i++ )
      {
        if (looks_like_number($adjointXyceData[$i]) && 
            looks_like_number($directXyceData[$i]) && 
            looks_like_number($gsLineOfDataFromXyce[$i] ) )
        {
          if( (abs( $adjointXyceData[$i] - $gsLineOfDataFromXyce[$i] ) < $absTol ) && 
              (abs( $directXyceData[$i] - $gsLineOfDataFromXyce[$i] ) < $absTol )) 
          {
             # regular compare
          }
          else
          {
            print "$ADJOINTFILE, $DIRECTFILE and $GOLDSENS found a numeric difference\n";
            print "Comparing Xyce's adjoint data \"$adjointXyceData[$i]\" and direct data \"$directXyceData[$i]\" to GS data \"$gsLineOfDataFromXyce[$i]\" as ";
            print "Failed numeric compare\n";
            $retval=2;
            last;
          }
        }
        elsif( ($adjointXyceData[$i] eq $gsLineOfDataFromXyce[$i]) && 
               ($directXyceData[$i] eq $gsLineOfDataFromXyce[$i]) )
        {
          # same in string context so ok.
          # print "string compare ok\n";
        }
        else
        {
          print "Elements failed compare:\n";
          print "Xyce Adjoint produced: \"$adjointXyceData[$i]\"\n";
          print "Xyce Direct produced: \"$directXyceData[$i]\"\n";
          print "Gold standard: \"$gsLineOfDataFromXyce[$i]\"\n";
          $retval=2;
          last;
        }
      
      }
    
    }
  }
  close(ADJRESULTS);
  close(DIRRESULTS);
  close(GOLD_STD);
}

if ( $retval != 0 )
{
  print "Exit code = $retval\n"; 
  exit $retval;
}

print "Exit code = $retval\n"; 
exit $retval;

