#!/usr/bin/env perl

use Cwd;

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$CIRFILEPY="python_gmls_diode_with_limiting.cir";
$CIRFILEGOLD="diode_with_limiting_nogenext.cir";

$TESTROOT = cwd;
print "TESTROOT = $TESTROOT\n";

# DEBUG: paths are hardcoded!
$PREFIX="";
$XYCEROOT="missing ";

print "XYCE = $XYCE\n";

# Decode Xyce root directory by stripping off Xyce
$XYCE =~ m/([^\/]*)(.*)\/Xyce.*/;
if (-d "$2") { $PREFIX=$1; $XYCEROOT=$2; }

print "XYCEROOT = $XYCEROOT\n";

#Check if we need a ".exe" extension (simply check for cygwin in uname)
$EXT="";
$result=system("uname | grep -i -q cygwin");
if ($result == 0)
{
  $EXT=".exe";
}

# set build dir and bin name
#UGH
if (-d "$XYCEROOT/CMakeFiles")
{
  $EXT="";
}
$TestProgram="Xyce-PyMi$EXT";
$XYCE_LIBTEST = "$XYCEROOT/$TestProgram";

# Run the Xyce-PyMi version:
if (-x $XYCE_LIBTEST) {
  print "NOTICE:   running ----------------------\n";
  $CMD="KOKKOS_NUM_THREADS=2 KOKKOS_PROFILE_LIBRARY=\"\" $XYCE_LIBTEST $CIRFILEPY > $CIRFILEPY.out 2> $CIRFILEPY.err";
  print "$CMD\n";
  $result = system("$CMD");
  if ($result == 0){ $retval=0;} else {$retval=10;}

} else {
  print "ERROR:    cannot find $XYCE_LIBTEST\n";
  $retval = 1;
}

if ($retval==0)
{

# Now run the Xyce-only version
    print "NOTICE:   running Xyce equivalent----------------------\n";
    $CMD="$XYCE $CIRFILEGOLD > $CIRFILEGOLD.out 2> $CIRFILEGOLD.err";
    $result = system("$CMD");
    if ($result == 0){ $retval=0;} else {$retval=10;}
}

# Now compare them:
if ($retval==0 && $result == 0)
{
    print "NOTICE:  Comparing ------------------------------------\n";
    $CMD="$XYCE_VERIFY $CIRFILEPY $CIRFILEGOLD.prn $CIRFILEPY.prn > $CIRFILEPY.prn.out 2> $CIRFILEPY.prn.err";
    $result = system("$CMD");
    if ($result == 0)
    { 
        # Comparison passed... but did we get any warnings about duplicate 
        # lines?
        $CMD="grep 'WARNING.*Throwing away line with duplicate' $CIRFILEPY.prn.err";
        $result= system("$CMD");
        if ( $result==0)
        {
            #Found such a line, report as failure.
            print STDERR "Comparison passed between Xyce-only and GenExt test output, but duplicate lines present so marking as failure.\n";
            $retval=2;
        }
        else
        {
            $retval=0;
        }
    } 
    else 
    {
        print STDERR "Comparison failed between Xyce-only and GenExt test output.\n";
        $retval=2;
    }
}    

print "Exit code = $retval\n"; exit $retval;

