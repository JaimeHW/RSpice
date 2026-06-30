#!/usr/bin/env perl

use Cwd;

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();
#$Tools->setFork(1);
#$Tools->setDebug(1);

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$CIRFILE="ADC_DACRunTest.cir";

$TESTROOT = cwd;

# DEBUG: paths are hardcoded!
$PREFIX="";
$TARGETDIR="";
$XYCEROOT="missing ";

print "XYCE = $XYCE\n";

# Try to decode Xyce root directory by stripping off bin/Xyce or src/Xyce
$XYCE =~ m/([^\/]*)(.*)\/bin\/Xyce.*/;
if (-d "$2") { $PREFIX=$1; $XYCEROOT=$2; }

$XYCE =~ m/([^\/]*)(.*)\/src\/(Release\/|Debug\/|RelWithDebInfo\/|)Xyce.*/;
if (-d "$2") { $PREFIX=$1; $XYCEROOT=$2; $TARGETDIR=$3}

print "PREFIX = $PREFIX\n";
print "XYCEROOT = $XYCEROOT\n";
print "TARGETDIR = $TARGETDIR\n";

#Check if we need a ".exe" extension (simply check for cygwin in uname)
$EXT="";
$result=system("uname | grep -i -q cygwin");
if ($result == 0)
{
  $EXT=".exe";
}

# set build dir and bin name
$MAKEROOT = "$XYCEROOT/src/test/DeviceInterface";
#UGH
if (-d "$MAKEROOT/CMakeFiles")
{
  $EXT="";
}
$XYCE_LIBTEST = "$XYCEROOT/src/test/DeviceInterface/${TARGETDIR}ADC_DACRunTest$EXT";
print("XYCE_LIBTEST = $XYCE_LIBTEST\n");
if (-d "$MAKEROOT") {
  if ((-e "$MAKEROOT/Makefile") and not (-d "$MAKEROOT/CMakeFiles") ) {
    chdir($MAKEROOT);
    print "Building ADC_DACRunTest in $MAKEROOT\n";
    print "NOTICE:   make clean -------------------\n";
    $result = system("make clean");
    print "NOTICE:   make ADC_DACRunTest -------------\n";
    $result += system("make ADC_DACRunTest$EXT");
    if($result) {
      print "WARNING:  make failures! ---------------\n";
      $retval = $result;
    }
  } elsif (-d "$MAKEROOT/CMakeFiles") {
    print "Using CMake, so assuming pre-built ADC_DACRunTest binary\n";
  } else {
    print "ERROR:    No build files!\n";
    $retval = 1;
  }
  chdir($TESTROOT);
} else {
    print "ERROR:    cannot chdir to $MAKEROOT\n";
  $retval = 1;
}

if (-x $XYCE_LIBTEST) {
  $CMD="$PREFIX$XYCE_LIBTEST $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
  print "NOTICE:   running ----------------------\n$CMD\n";
  $retval = system("$CMD");
  if($retval != 0)
  {
    print "ERROR:    ADC_DACRunTest failures! --------\n";
    $returncode = $retval >> 8;
    $signalcode = $retval & 127;

    if ($retval == -1)
    {
        print "    failure to execute $PREFIX$XYCE_LIBTEST\n";
        $retval = 1;  # SHELL SCRIPT EXITED WITH ERROR
    }
    else
    {
        if ($signalcode != 0)
        {
            print "    code died with signal $signalcode\n";
            $retval=13;   # crashed
        }
        else
        {
            print "  returned non-zero exit code\n"  ;
            $retval=$returncode;
        }
    }
  }

} else {
  print "ERROR:    cannot find $XYCE_LIBTEST\n";
  $retval = 1;
}

print "Exit code = $retval\n"; exit $retval;


