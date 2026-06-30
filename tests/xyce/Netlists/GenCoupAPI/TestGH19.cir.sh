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

$TESTROOT = cwd;

`rm -f $CIRFILE.out $CIRFILE.err`;

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
$TestProgram="TestGH19$EXT";
$XYCE_LIBTEST = "$MAKEROOT/$TARGETDIR$TestProgram";

if (-d "$MAKEROOT") {
  if ( (-e "$MAKEROOT/Makefile") and not (-d "$MAKEROOT/CMakeFiles") ) {
    chdir($MAKEROOT);
    print "NOTICE:   make clean -------------------\n";
    $result = system("make clean");
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

if (-x $XYCE_LIBTEST) {
  print "NOTICE:   running ----------------------\n";
  $CMD="$XYCE_LIBTEST > $CIRFILE.out 2> $CIRFILE.err";
  print "$CMD\n";
  $result = system("$CMD");
  if ($result == 0){ $retval=0;} else {$retval=10;}

} else {
  print "ERROR:    cannot find $XYCE_LIBTEST\n";
  $retval = 1;
}

# Now check for appropriate error messages and output
if ($retval==0 && $result == 0)
{
    print "NOTICE:  Checking results ------------------------------------\n";
    `echo "NOTICE:  Checking results ------------------------------------" > $CIRFILE.prn.out`;
    `echo "NOTICE:  Checking results ------------------------------------" > $CIRFILE.prn.err`;


    # The program will output everything to its standard out (nothing to
    # std err) and should have reported "first instance crashed" and
    # not "second instance crashed".  The second instance should have
    # produced a line saying "Total Devices 6" (modulo white space)
    # So let's open the output and read it line by line and check for those
    # things
    $FIRST_CRASHED=0;    # should crash
    $SECOND_CRASHED=0;   # should not crash
    $TOTAL_DEV_6=0;      # should be 6
    open(OUTFILE,"<$CIRFILE.out") || die "Could not open $CIRFILE.out for reading";
    while (<OUTFILE>)
    {
        if (m/first instance crashed/)
        {
            $FIRST_CRASHED=1;
        }
        if (m/second instance crashed/)
        {
            $SECOND_CRASHED=1;
        }
        if (m/Total Devices.*6/)
        {
            $TOTAL_DEV_6=1;
        }
    }
    close(OUTFILE);

    if ($FIRST_CRASHED==1 && $SECOND_CRASHED == 0 && $TOTAL_DEV_6 == 1)
    {
        $retval=0;
        print STDERR "The first instance crashed as it should have, the second didn't as it should not have, and the device count was correct\n";
    }
    else
    {
        if ($FIRST_CRASHED==0)
        {
            print STDERR "The first instance did not crash and should have\n";
        }
        if ($SECOND_CRASHED==1)
        {
            print STDERR "The second instance crashed and should not have\n";
        }
        if ($TOTAL_DEV_6==0)
        {
            print STDERR "The second instance did not produce output saying that the total number of devices was 6\n";
        }
        $retval=2;
    }

}

print "Exit code = $retval\n"; exit $retval;

