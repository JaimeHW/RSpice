#!/usr/bin/env perl

use Cwd;

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# Let's test Xyce's HB analysis and output with General External Coupling

$XYCE=$ARGV[0];

$XYCE_VERIFY=$ARGV[1];
$AC_COMPARE=$ARGV[1];
$AC_COMPARE =~ s/xyce_verify/ACComparator/;

$CIRFILE=$ARGV[3];

$CIRFILE="rc_hb.cir";
$CIRFILE2="rc_hb_nogenext.cir";
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
$MAKEROOT = "$XYCEROOT/src/test/GenExtTestHarnesses/";
#UGH
if (-d "$MAKEROOT/CMakeFiles")
{
  $EXT="";
}
$TestProgram="testGenCoup$EXT";
$XYCE_LIBTEST = "$MAKEROOT/$TARGETDIR$TestProgram";

if (-d "$MAKEROOT") {
  if ( (-e "$MAKEROOT/Makefile") and not (-d "$MAKEROOT/CMakeFiles") ) {
    chdir($MAKEROOT);
    print "NOTICE:   make -------------------------\n";
    $result += system("make $TestProgram");
    if($result) {
      print "WARNING:  make failures! ---------------\n";
      $retval = $result;
    }
  } elsif (-d "$MAKEROOT/CMakeFiles") {
    print "Using CMake, so assuming pre-built $TestProgram binary\n";
  } else {
    print "ERROR:    No build files!\n";
    $retval = 1;
  }
  chdir($TESTROOT);
} else {
    print "ERROR:    cannot chdir to $MAKEROOT\n";
  $retval = 1;
}

# Run the GenExt version:
if (-x $XYCE_LIBTEST) {
  print "NOTICE:   running ----------------------\n";
  $CMD="$XYCE_LIBTEST -iotest5 $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
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
    $CMD="$XYCE $CIRFILE2 > $CIRFILE2.out 2> $CIRFILE2.err";
    $result = system("$CMD");
    if ($result == 0){ $retval=0;} else {$retval=10;}
}


# Now compare the frequency domain data:
if ($retval==0 && $result == 0)
{
    print "NOTICE:  Comparing ------------------------------------\n";
    $CMD="$AC_COMPARE $CIRFILE2.HB.FD.prn $CIRFILE.HB.FD.prn 1e-6 2e-2 1e-15 1e-6 > $CIRFILE.prn.out 2> $CIRFILE.prn.err";
    $result = system("$CMD");
    if ($result == 0)
    { 
        $retval=0;
    } 
    else 
    {
        print STDERR "Comparison failed between Xyce-only and GenExt frequency domain output.\n";
        $retval=2;
    }
    # compare time domain data
    if ($retval == 0)
    {
        $CMD="$XYCE_VERIFY $CIRFILE $CIRFILE2.HB.TD.prn $CIRFILE.HB.TD.prn >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
        $result = system("$CMD");
        if ($result == 0)
        { 
            $retval=0;
        } 
        else 
        {
            print STDERR "Comparison failed between Xyce-only and GenExt time domain output.\n";
            $retval=2;
        }
    }
    if ($retval == 0)
    {
        print "NOTICE:  Comparing .print and external out----------------\n";
        $CMD="$AC_COMPARE $CIRFILE.HB.FD.prn ioTest5.FD.out  1e-6 2e-2 1e-16 1e-6 >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
        $result = system("$CMD");
        if ($result == 0)
        { 
                $retval=0;
        } 
        else 
        {
            print STDERR "Comparison failed between GenExt .print output and ExternalOutput mechanism in frequency domain.\n";
            $retval=2;
        }
    }
    if ($retval == 0)
    {
        print "NOTICE:  Comparing .print and external out----------------\n";
        $CMD="$XYCE_VERIFY $CIRFILE $CIRFILE.HB.TD.prn ioTest5.TD.out  >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
        $result = system("$CMD");
        if ($result == 0)
        { 
                $retval=0;
        } 
        else 
        {
            print STDERR "Comparison failed between GenExt .print output and ExternalOutput mechanism in time domain.\n";
            $retval=2;
        }
    }
}

print "Exit code = $retval\n"; exit $retval;

