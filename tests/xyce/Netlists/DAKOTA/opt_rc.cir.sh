#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setDebug(1);
#$Tools->setVerbose(1);

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

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

# this is an optimization problem for Xyce-Dakota to solve

#$CIR0="opt_rc.cir";
$XyceWithOptions = "$XYCE -dakota opt_rc.dak";

# Now this is annoying.  The ERROR measure looks at the filename suffix
# to determine whether it's a supported file type, but ".prn" files are 
# automatically removed by the run_xyce_regression script prior to starting
# the test (in the runTest function).  So we have to move the reference
# data to a .prn suffix before running Xyce.
`mv opt_rc.reference_data opt_rc.prn`;
$retval = -1;
$retval=$Tools->wrapXyce($XyceWithOptions,$CIRFILE );
#
# it's ok to pass a retval of 14 as this just means that our 
# circuit.cir didn't produce a circuit.cir.prn.  Not a problem here
# becasue we make circuit.cir.runXXX.prn files
#
if ($retval != 0 && $retval != 14) 
{ 
  print "Exit code = $retval\n"; 
  exit $retval; 
}

# now scan through output to find the run that produced the
# best fit.  We can use that to compare to the gold standard
open( outputFile, "outputDakota.txt");
my $runNumber="";
my $bestRunNumber="";
my $keyString="";
my $inParameterSection="false";
my %resultHash ={};
while( my $line = <outputFile> )
{
  # search the output for lines like
  # Parameters for function evaluation 10:
  #                    9.9829841628e-05 dakota_C1C
  # and remember these data in a dictionary 
  
  # then when we get to the end of the output we'll
  # see 
  #
  # <<<<< Best parameters          =
  #                    9.9829841628e-05 dakota_C1C
  # <<<<< Best objective function  =
  #
  # and we can use that as a key to look up an evaluation number
  #
  
  if( $line =~ /Direct interface/ )
  {
    $inParameterSection="false";
    $resultHash{ $key } = $runNumber;
      next;
  }
  
  if( $line =~ /Duplication detected:/ )
  {
    # don't store this key, value pair
    $inParameterSection="false";
      next;
  }

  if( $line =~ /<<<<< Best objective function  =/ )
  {
    # now we have all the parameters of the best 
    # function eval. So lookup the run number in our hash
    $bestRunNumber = $resultHash{ $key };
    $inParameterSection="false";
    break;
  }
  
  if( $inParameterSection =~ /true/ )
  {
      chomp $line;
      if ($line =~ /^\s*([^\s]*) ([^\s]*)\s*$/)
      {
	  #    @vals = split " ", $line;
	  @vals=($1,$2);
	  $key = $key . ":" . $vals[0] . ":" .  $vals[1];
      }
      next;
  }
  
  if( $line =~ /Parameters for evaluation/ )
  {
    chomp $line;
    # get the evaluation number
    $line =~ /Parameters for evaluation ([0-9]*)/;
    $runNumber = $1;
    $inParameterSection="true";
    $key="";
      next;
  }
  
  if( $line =~ /<*\s*Best parameters\s*=/ )
  {
    # start gathering the best parameters as if they
    # were any other key, when get to the text
    # <<<<< Best objective function  =
    # then we've found the end of this key and can act on it
    $inParameterSection="true";
    $key="";
      next;
  }
}
close outputFile;

my $optimalResultFile = "$CIRFILE.run$bestRunNumber.prn";
my $CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $optimalResultFile";
$retval = 0;
$retval = system("$CMD");
# The Perl system does not return the raw exit code of the process it
# ran, it returns that in the high byte and the signal caught in the low
# byte.  Extract the return code 
$retval = $retval >> 8;
#xyce_verify.pl returns "20" for a failed compare, but that's not what
# run_xyce_regression wants to see --- we need a "2" instead
if ($retval == 20)
{
    $retval=2;
}
print "Exit code = $retval\n";
exit $retval;

