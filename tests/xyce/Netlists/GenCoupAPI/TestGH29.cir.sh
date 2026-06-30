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

$CIRFILE="VRC.sp";
$CIRFILE2="VRC_withprint.sp";

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
$TestProgram="TestIssue29$EXT";
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

# Run the netlist without a print line, and only an V(*) I(*) external output:
if (-x $XYCE_LIBTEST) {
  print "NOTICE:   running netlist with no print line ------------\n";
  $CMD="$XYCE_LIBTEST $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
  print "$CMD\n";
  $result = system("$CMD");
  if ($result == 0){ $retval=0;} else {$retval=10;}
} else {
  print "ERROR:    cannot find $XYCE_LIBTEST\n";
  $retval = 1;
}

if ($retval==0)
{
    system ("mv ioTest.out ioTest.out_noprint");
    # Now run the netlist with BOTH a print line and external output
    print "NOTICE:   running netlist with print line ---------------\n";
    $CMD="$XYCE_LIBTEST $CIRFILE2 > $CIRFILE2.out 2> $CIRFILE2.err";
    print "$CMD\n";
    $result = system("$CMD");
    if ($result == 0){ $retval=0;} else {$retval=10;}
}

# Now compare them:
if ($retval==0 && $result == 0)
{
    print "NOTICE:  Comparing the two external outputs ----------------\n";
    $CMD = "diff ioTest.out ioTest.out_noprint";
    $result = system($CMD);
    if ($result == 0)
    {
        print "NOTICE: the two external outputs match ----------------\n";
        $retval=0;
    }
    else
    {
        print "The external outputs of the two runs do not match\n";
        print STDERR "The external outputs of the two runs do not match\n";
        $retval=2;
    }

    if ($retval==0)
    {
        print "NOTICE:  Comparing the .print output with first external output --\n";
        # xyce_verify doesn't grok wildcards.  Create a fake netlist
        # using the first line of the actual output resulting from
        # wildcards, and replace the print line with what the wildcard
        # version actually produced
        open(STARVARS,"<${CIRFILE2}.prn") || die "Cannot open ${CIRFILE2}.prn";
        $svhead=<STARVARS>;
        close STARVARS;
        chomp($svhead);
        @svvars=split(" ",$svhead);
        shift @svvars; shift @svvars;

        open (CIRFILE,"<$CIRFILE2");
        `rm -f fakecir`;
        open (FAKECIR,">fakecir");
        while (<CIRFILE>)
        {
            if (! (/^.print tran/i))
            {
                print FAKECIR;
            }
            else
            {
                print FAKECIR ".print tran ";
                foreach $var (@svvars)
                {
                    print FAKECIR " $var";
                }
                print FAKECIR "\n.end\n";
                last;
            }
        }
        close FAKECIR;
        close CIRFILE;

        $CMD="$XYCE_VERIFY fakecir ${CIRFILE2}.prn ioTest.out >> $CIRFILE.prn.out 2>> $CIRFILE.prn.err";
        $result = system("$CMD");
        if ($result == 0)
        {
            # Comparison passed... but did we get any warnings about duplicate
            # lines?
            $CMD="grep 'WARNING.*Throwing away line with duplicate' $CIRFILE.prn.err";
            $result= system("$CMD");
            if ( $result==0)
            {
                #Found such a line, report as failure.
            print STDERR "Comparison passed between GenExt .print starvar output and ExternalOutput mechanism, but duplicate lines present so marking as failure.\n";
                $retval=2;
            }
            else
            {
                print "NOTICE:  .print output and first external output match --\n";
                $retval=0;
            }
        }
        else
        {
            print STDERR "Comparison failed between GenExt .print starvar output and ExternalOutput mechanism.\n";
            $retval=2;
        }
    }
}

print "Exit code = $retval\n"; exit $retval;

