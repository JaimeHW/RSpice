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

$ORIGINAL_DAKOTA_INPUT_FILE="opt_soi.dak";
$MODIFIED_DAKOTA_INPUT_FILE="opt_soi.mod.dak";

# this is an optimization problem for Xyce-Dakota to solve
# with dakota calling Xyce in black box mode 
#
# first we need to verify that dakota.sh or (dakota.bat under windows)
# is on the path.  If it is not then exit with an error.
#
# second, if we find dakota then we need to modify the dakota configuration 
# file opt_soi.dak in this case to have the Xyce binary path in the 
# analysis_driver variable.


my $retval = -1;
my $dakota="not found";
if( (system("dakota -help") >>8) == 0 )
{
  print "dakota command found\n";
  $dakota="dakota";
}
if( (system("dakota.sh -help") >>8) == 0 )
{
  print "dakota.sh command found\n";
  $dakota="dakota.sh";
}
if( (system("dakota.bat -help") >>8) == 0 )
{
  print "dakota.bat command found\n";
  $dakota="dakota.bat";
}

if( $dakota =~ "not found" )
{
  print "No dakota binary found in the environment.\n";
  $retval=18;
  print "Exit code = $retval\n";
  exit $retval;
}

#
# if we get to here we have found dakota on the test system
# Now edit the dakota input file to specify the copy of Xyce 
# being tested.

open( dakotaConfig, "$ORIGINAL_DAKOTA_INPUT_FILE");
open( newDakotaConfig, ">$MODIFIED_DAKOTA_INPUT_FILE");

while( my $line = <dakotaConfig> )
{
  if ($line =~ "analysis_driver")
  {
    # output line with correct Xyce path 
    print newDakotaConfig "analysis_driver = '$XYCE opt_soi.cir -l opt_soi.log -prf '\n";
  }
  else
  {
    # just echo over old line.
    print newDakotaConfig $line;
  }
}
close (dakotaConfig);
close (newDakotaConfig);

$retval = system("$dakota opt_soi.dak > opt_soi_dakota.out") >> 8;

print "Exit code = $retval\n";
exit $retval;



$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err ";
$retval = system("$CMD");
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
open( outputFile, "$CIRFILE.out");
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
  
  if( $line =~ /Direct function:/ )
  {
    $inParameterSection="false";
    $resultHash{ $key } = $runNumber;
  }
  
  if( $line =~ /Duplication detected:/ )
  {
    # don't store this key, value pair
    $inParameterSection="false";
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
    chop $line;
    @vals = split " ", $line;
    if( (scalar @vals) > 1 )
    {
      $key = $key . ":" . @vals[0] . ":" .  @vals[1];
    }
  }
  
  if( $line =~ /Parameters for function evaluation/ )
  {
    # get the evaluation number
    chop $line;
    @vals = split " |:", $line;
    $runNumber = @vals[  scalar @vals - 1 ];
    $inParameterSection="true";
    $key="";
  }
  
  if( $line =~ /<<<<< Best parameters          =/ )
  {
    # start gathering the best parameters as if they
    # were any other key, when get to the text
    # <<<<< Best objective function  =
    # then we've found the end of this key and can act on it
    $inParameterSection="true";
    $key="";
  }
}
close outputFile;

my $optimalResultFile = "$CIRFILE.run$bestRunNumber.prn";
my $CMD="$XYCE_VERIFY $CIRFILE $GOLDPRN $optimalResultFile";
$retval = 0;
$retval = system("$CMD");

print "Exit code = $retval\n";
exit $retval;

