#!/usr/bin/env perl

use Cwd;

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
#$GOLDPRN=$ARGV[4];

$GOLDPRN=$CIRFILE.".prn.gs";

# At this point we need to make sure we've actually been given a Xyce binary
# that is properly installed (via "make install"), and that the buildxyceplugin
# script exists along side it.

# First let us isolate the directory name of the Xyce binary given
# Before we can do that, we have to tease out the binary from the command line,
# which could include mpirun commands:
if ($XYCE =~ m/^(.*) (.*Xyce)$/)
{
  $MPIRUN=$1;
  $XYCE=$2;
}

print "XYCE: $XYCE\n";
print "MPIRUN: $MPIRUN\n";

$TESTROOT = cwd;

# Now, if $XYCE is identically "Xyce" we're running an installed binary
# that is in fact already in our PATH, and not being given a complete
# path name.  We do different things in that case.
if ($XYCE eq "Xyce")
{
    print "Running Xyce right out of path!\n";
    # is buildxyceplugin actually in our path, too?

    `which buildxyceplugin > /dev/null`;
    if ( $? != 0 )
    {
	print "buildxyceplugin is not in your path, so cannot build plugins!\n";
	print "Exit code = 1\n";
	exit 1;
    }
    else
    {
	print "Running buildxyceplugin right out of path!\n";
	$BUILDXYCEPLUGIN="buildxyceplugin";
    }

    # If buildxyceplugin is in the path, assume all else is fine (i.e. that
    # that libraries and headers are installed, too)
}
else
{
    # Otherwise it's an actual path, decode it and sanity check
    $XyceDir=`dirname $XYCE`;
    chomp($XyceDir);
    $XyceRoot=`dirname $XyceDir`;
    chomp($XyceRoot);
    $BinOrSrc=`basename $XyceDir`;
    chomp($BinOrSrc);

    if ($BinOrSrc != "bin")
    {
	print "The binary is in $XyceDir, and ";
	print "it is not in a bin directory --- this test cannot work.\n";
	print "Exit code = 1\n";
	exit 1;
    }

# Now, does buildxyceplugin exist?
    if (! (-x "$XyceDir/buildxyceplugin"))
    {
	print "the buildxyceplugin script was not found in $XyceDir\n";
	print "Exit code = 1\n";
	exit 1;
    }
    $BUILDXYCEPLUGIN="$XyceDir/buildxyceplugin";

# Are the libraries there?
    if (! (-d "$XyceRoot/lib" && (-e "$XyceRoot/lib/libxyce.so" || -e "$XyceRoot/lib/libxyce.dylib")))
    {
	print "the libraries were not found in $XyceRoot/lib\n";
	print "Exit code = 1\n";
	exit 1;
    }
}

# is admsXml actually in our path?
`admsXml -v > /dev/null`;
if ( $? != 0 )
{
    print "admsXml is not in your path, so cannot build plugins!\n";
    print "Exit code = 1\n";
    exit 1;
}

# Now reassemble the Xyce command we were given
if ($MPIRUN ne "")
{
    $XYCE="$MPIRUN $XYCE";
}
#Hooray, we can run this test.
$VERILOG_SOURCES="toys/capacitor.va toys/diode2.va toys/resistor.va toys/vsrc.va";

print "Building the plugin...\n";

$result=system("$BUILDXYCEPLUGIN -o toys $VERILOG_SOURCES .");
if ($result != 0)
{
    print "Plugin creation failed, see ${CIRFILE}_buildxyceplugin.log\n";
    system("mv buildxyceplugin.log ${CIRFILE}_buildxyceplugin.log");
    print "Exit code = 1";
    exit 1;
}
$PLUGINPATH="$TESTROOT/toys.so";
print "The plugin path is $PLUGINPATH\n";


# Now we can run the CIRFILE
$result=system("$XYCE -plugin $PLUGINPATH $CIRFILE > $CIRFILE.out 2> $CIRFILE.err");
if ($result != 0)
{
    if ($result & 127)
    {
        print "Exit code = 13\n";
        printf STDERR "Xyce crashed with signal %d on file %s\n",
            ($result&127),$CIRFILE;
        exit 13;
    }
    else
    {
        print "Exit code = 10\n";
        printf STDERR "Xyce exited with exit code %d on %s\n",
            $result>>8,$CIRFILE;
        exit 10;
    }
}

# Now compare to our gold standard
$result=system("$XYCE_VERIFY $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2>$CIRFILE.prn.err");
if ($result == 0)
{
    $retval=0;
}
else
{
    print "Failed comparison to gold standard\n";
    $retval=2;
}

print "Exit code = $retval\n"; exit $retval;

