#!/usr/bin/env perl

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

$CIRFILE="bug_668.cir";
$CIR_RAW="bug_668_raw.cir";
$CIR_RAW_PRINTLINE="bug_668_raw_printline.cir";
$CIR_RAW_ASCII="bug_668_raw_asciiprintline.cir";
$CIR_PROBE="bug_668_probe.cir";
$CIR_TECPLOT="bug_668_tecplot.cir";

$TRANSLATESCRIPT=$XYCE_VERIFY;
$TRANSLATESCRIPT =~ s/xyce_verify.pl/convertToPrn.py/;
$TRANSLATE="python $TRANSLATESCRIPT ";

`rm -f *_orig`;

# create the output files
$result = system("$XYCE $CIRFILE" );
if ( ( $result != 0 ) ) 
{
  print "Crashed trying to run standard format \n";
  print "Exit code = 13\n";
  exit 13;
}
if ( ! -e "$CIRFILE.prn")
{
  print "Failed to produce standard format \n";
  print "Exit code = 14\n";
  exit 14;
}

$result = system("$XYCE -r $CIR_RAW.bin $CIR_RAW");
if ( ( $result != 0 ) ) 
{
  print "Crashed trying to run binary rawfile \n";
  print "Exit code = 13\n";
  exit 13;
}
if ( ! -e "$CIR_RAW.bin")
{
  print "Failed to produce binary raw \n";
  print "Exit code = 14\n";
  exit 14;
}

$result = system("$XYCE  $CIR_RAW_PRINTLINE");
if ( ( $result != 0 ) ) 
{
  print "Crashed trying to run binary rawfile from print line \n";
  print "Exit code = 13\n";
  exit 13;
}
if ( ! -e "$CIR_RAW_PRINTLINE.raw")
{
  print "Failed to produce binary rawfile from print line \n";
  print "Exit code = 14\n";
  exit 14;
}

$result = system("$XYCE -r $CIR_RAW.txt -a $CIR_RAW");
if ( ( $result != 0 ) ) 
{
  print "Crashed trying to run ASCII rawfile \n";
  print "Exit code = 13\n";
  exit 13;
}
if ( ! -e "$CIR_RAW.txt")
{
  print "Failed to produce ASCII raw \n";
  print "Exit code = 14\n";
  exit 14;
}

$result = system("$XYCE -a $CIR_RAW_ASCII");
if ( ( $result != 0 ) ) 
{
  print "Crashed trying to run ASCII rawfile from print line \n";
  print "Exit code = 13\n";
  exit 13;
}
if ( ! -e "$CIR_RAW_ASCII.txt")
{
  print "Failed to produce ASCII raw from print line\n";
  print "Exit code = 14\n";
  exit 14;
}

$result = system( "$XYCE $CIR_PROBE" );
if ( ( $result != 0 ) ) 
{
  print "Crashed trying to run probe \n";
  print "Exit code = 13\n";
  exit 13;
}
if ( ! -e "$CIR_PROBE.prn")
{
  print "Failed to produce probe  \n";
  print "Exit code = 14\n";
  exit 14;
}

$result = system( "$XYCE $CIR_TECPLOT");
if ( ( $result != 0 ) ) 
{
  print "Crashed trying to run tecplot \n";
  print "Exit code = 13\n";
  exit 13;
}
if ( ! -e "$CIR_TECPLOT.dat")
{
  print "Failed to produce tecplot  \n";
  print "Exit code = 14\n";
  exit 14;
}



# translate output to std format
$result = system( "$TRANSLATE $CIR_RAW.bin" );
if ( $result != 0 )
{
  print "Failed to translate binary RAW to STD\n";
  print "Exit code = 2\n";
  exit 2;
}

$result = system( "$TRANSLATE $CIR_RAW.txt" );
if ( $result != 0 )
{
  print "Failed to translate all ASCII RAW to STD\n";
  print "Exit code = 2\n";
  exit 2;
}

$result = system( "$TRANSLATE $CIR_RAW_PRINTLINE.raw" );
if ( $result != 0 )
{
  print "Failed to translate binary RAW (print line) to STD\n";
  print "Exit code = 2\n";
  exit 2;
}

$result = system( "$TRANSLATE $CIR_RAW_ASCII.txt" );
if ( $result != 0 )
{
  print "Failed to translate all ASCII RAW (printline)to STD\n";
  print "Exit code = 2\n";
  exit 2;
}

$result = system( "$TRANSLATE $CIR_PROBE.prn" );
if ( $result != 0 )
{
  print "Failed to translate PROBE  to STD\n";
  print "Exit code = 2\n";
  exit 2;
}

$result = system( "$TRANSLATE $CIR_TECPLOT.dat" );
if ( $result != 0 )
{
  print "Failed to translate TECPLOT to STD\n";
  print "Exit code = 2\n";
  exit 2;
}


# OK, the rawfiles need some serious hand-massaging.
# Rawfiles output voltage branches as "vx#branch" instead of "I(vx)".
# So we fix that.
# Furthermore, rawfiles produced with "-r" output data in whatever order
# it actually appears in the solution vector, with no regard to what order
# they have on any print line.
# This impacts parallel testing, where things are in a different order
# than in serial, thanks to distribution of the solution vector.
# We must re-order the data so it's in the same order as the .prn file!

# Step one:  create a hash based on the header line of the .prn file, such
# that $headerhash{var name}=column#

open(FOO,"<$CIRFILE.prn") || die "Could not open $CIRFILE.prn";
$line=<FOO>;  # get the first line, only
close(FOO);
$line =~ s/^\s*//; # kill leading spaces
$line =~ s/\s+/ /g; # squeeze spaces out
@headerlabels=split(/\s/, $line);

# Now we know what order things need to be in.  Rearrange columns as appropriate
# and fix the vx#branch crud at the same time, too.
`mv test_$CIR_RAW.bin test_$CIR_RAW.bin_orig`;
open(FOO,"<test_$CIR_RAW.bin_orig") || die "Could not open test_$CIR_RAW.bin_orig";
open(BAR,">test_$CIR_RAW.bin") || die "Could not open test_$CIR_RAW.bin";
$firstline=1;
while($line=<FOO>)
{
   if ($firstline==1)
   {
       $line =~ s/([vV][^#\)])*#branch/I($1)/g;
       $line =~ s/^\s*//; # kill leading spaces
       $line =~ s/\s+/ /g; # squeeze spaces out
       @testlabels=split(/\s/, $line);
       for ($i=0;$i<=$#testlabels;$i++)
       {
           $headerhash{lc($testlabels[$i])} = $i;
       }
       $firstline=0;
       # write the header using the PRN ordering
       for ($i = 0 ; $i <= $#headerlabels ; $i++)
       {
           print BAR "$headerlabels[$i] ";
       }
       print BAR "\n";
   }
   else
   {
       if ($line =~ /End of Xyce/)
       {
           print BAR $line;
       }
       else
       {
           chomp($line); # kill newline
           $line =~ s/^\s*//; # kill leading spaces
           $line =~ s/\s+/ /g; # squeeze spaces out
           @datacolumns=split(/\s/, $line);
           for ($i=0;$i<=$#testlabels;$i++)
           {
               print BAR "$datacolumns[$headerhash{lc($headerlabels[$i])}] ";
           }
           print BAR "\n";
       }
   }
}
close FOO;
close BAR;

#do the same for the ascii rawfile
`mv test_$CIR_RAW.txt test_$CIR_RAW.txt_orig`;
open(FOO,"<test_$CIR_RAW.txt_orig") || die "Could not open test_$CIR_RAW.txt_orig";
open(BAR,">test_$CIR_RAW.txt") || die "Could not open test_$CIR_RAW.txt";
$firstline=1;
while($line=<FOO>)
{
   if ($firstline==1)
   {
       $line =~ s/([vV][^#\)])*#branch/I($1)/g;
       $line =~ s/^\s*//; # kill leading spaces
       $line =~ s/\s+/ /g; # squeeze spaces out
       @testlabels=split(/\s/, $line);
       for ($i=0;$i<=$#testlabels;$i++)
       {
           $headerhash{lc($testlabels[$i])} = $i;
       }
       $firstline=0;
       # write the header using the PRN ordering
       for ($i = 0 ; $i <= $#headerlabels ; $i++)
       {
           print BAR "$headerlabels[$i] ";
       }
       print BAR "\n";
   }
   else
   {
       if ($line =~ /End of Xyce/)
       {
           print BAR $line;
       }
       else
       {
           chomp($line); # kill newline
           $line =~ s/^\s*//; # kill leading spaces
           $line =~ s/\s+/ /g; # squeeze spaces out
           @datacolumns=split(/\s/, $line);
           for ($i=0;$i<=$#testlabels;$i++)
           {
               print BAR "$datacolumns[$headerhash{lc($headerlabels[$i])}] ";
           }
           print BAR "\n";
       }
   }
}
close FOO;
close BAR;

# It is NOT necessary to do this same thing for rawfiles produced with
# format=raw, because they call these branches I(Vx)!


# diff the remaining results; ignore differences in header case
$d1of5 = system( "diff -ib test_$CIR_RAW.bin test_$CIR_RAW.txt > $CIR_RAW.stdout 2>> $CIRFILE.stderr");
$d2of5 = system( "diff -ib test_$CIR_RAW.txt test_$CIR_PROBE.prn > $CIR_PROBE.stdout 2>> $CIRFILE.stderr");
$d3of5 = system( "diff -ib test_$CIR_PROBE.prn test_$CIR_TECPLOT.dat > $CIR_TECPLOT.stdout 2>> $CIRFILE.stderr");
$d4of5 = system( "diff -ib test_$CIR_RAW.bin test_$CIR_RAW_PRINTLINE.raw > $CIR_RAW_PRINTLINE.stdout 2>> $CIRFILE.stderr");
$d5of5 = system( "diff -ib test_$CIR_RAW.bin test_$CIR_RAW_ASCII.txt > $CIR_RAW_ASCII.stdout 2>> $CIRFILE.stderr");

if ( $d1of5 or $d2of5 or $d3of5 or $d4of5 or $d5of5 )
{
  print "Failed to confirm that all translated results are the same\n";
  print "Failed compare test_$CIR_RAW.bin to test_$CIR_RAW.txt\n" if $d1of5;
  print "Failed on test_$CIR_RAW.txt test_$CIR_PROBE.prn\n" if $d2of5;
  print "Failed on test_$CIR_PROBE.prn to test_$CIR_TECPLOT.dat\n" if $d3of5;
  print "Failed on test_$CIR_RAW.bin to test_$CIR_RAW_PRINTLINE.raw\n" if $d4of5;
  print "Failed on test_$CIR_RAW.bin to test_$CIR_RAW_ASCII.txt\n" if $d5of5;
  print "Exit code = 2\n";
  exit 2;
}

# verify a prn match
$result = system( "$XYCE_VERIFY $CIRFILE $CIRFILE.prn test_$CIR_RAW.bin > $CIRFILE.prn.out 2> $CIRFILE.prn.err" );

# output final result
print "Exit code = $result\n";
exit $result;
