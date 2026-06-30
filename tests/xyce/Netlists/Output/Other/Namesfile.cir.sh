#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script 
# $ARGV[3] = location of circuit file to test
# $ARGV[4] = location of gold standard prn file

# If Xyce does not produce the appropriate output files, then we return exit code 14.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.txt.out and the STDERR output from
# comparison to go into $CIRFILE.txt.err.

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDTXT=$ARGV[4];

$GOLDTXT =~ s/\.prn$//; # remove the .prn at the end.
$GOLDTXT = "$GOLDTXT.txt"; # add .txt

# declare the comparison arrays and line count variables for later use
my @firstColumnTest;
my @secondColumnTest;
my @firstColumnGold;
my @secondColumnGold;
my $goldCount;
my $testCount;

# clean up files from previous runs
system("rm -f $CIRFILE.out $CIRFILE.err $CIRFILE.txt");
system("rm -f $CIRFILE.txt.out $CIRFILE.txt.err");

#run Xyce to make the namesfile, and exit on Xyce failure.
$CMD="$XYCE -namesfile $CIRFILE.txt $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
$xyceexit = system($CMD);
if ($xyceexit != 0) 
{
  if ($xyceexit & 127)
  {
    print "Exit code = 13\n"; 
    printf STDERR "Xyce crashed with signal %d on file %s\n",($xyceexit&127),$CIRFILE; 
    exit 13;
  }
  else
  {
    print "Exit code = 10\n"; 
    printf STDERR "Xyce exited with exit code %d on %s\n",$xyceexit>>8,$CIRFILE; 
    exit 10;
  }
} 

# check that namesfile and gold namesfile both exist
$retcode = 0;
if ( !(-f "$CIRFILE.txt")) {
    print STDERR "Missing output namesfile $CIRFILE.txt\n";
    $retcode=14;
}
if ($retcode != 0) {print "Exit code = $retcode\n"; exit $retcode;}

if ( !(-f "$GOLDTXT")) {
    print STDERR "Missing GOLD namesfile $GOLDTXT\n";
    $retcode=2;
}
if ($retcode != 0) {print "Exit code = $retcode\n"; exit $retcode;}

# Check that both files have the same number of lines, and build the
# comparison arrays for the first and second columns in the output
# and gold namesfiles.
open(TESTFILE,"$CIRFILE.txt");
open(GSFILE,"$GOLDTXT");

$testCount=0;
while ($lineTest=<TESTFILE>) 
{ 
  $testCount++;
  chop $lineTest;
  # Remove leading spaces on line, otherwise the spaces become 
  # element 0 of "testData" instead of the first column of data.
  $lineTest =~ s/^\s*//;
  @testData = (split(/[\s,]+/, $lineTest));
  #print "for testCount=$testCount, testData=@testData\n";
  #print "number of elements in testData=$#testData\n";

  # First row should only have one element. Other rows should have two elements.
  # The variable $#testData has the index of the last element in the testData array,
  # rather than the size of testData.  So, $#testData+1 is the size of the array.
  if ( ($testCount == 1) && ($#testData+1 != 1))
  {
    print STDERR "First line of file $CIRFILE.txt should only have one element\n";
    print "Exit code = 2\n"; exit 2;
  }
  elsif ( ($testCount > 1) && ($#testData+1 != 2))
  {
    print STDERR "Line in file $CIRFILE.txt should have two elements\n";
    print "Exit code = 2\n"; exit 2;
  }

  # Populate the comparison arrays.  The firstColumnTest array will have the same
  # size as the number of lines in the file.  The secondColumnTest array
  # will be one entry smaller.
  if ($testCount == 1) 
  {
    #print "testData0 = $testData[0]\n";
    $firstColumnTest[0]=$testData[0];
  }
  else
  {
    #print "testData0 = $testData[0]\n";
    #print "testData1 = $testData[1]\n";
    $firstColumnTest[$testCount-1]=$testData[0];
    $secondColumnTest[$testCount-2]=$testData[1];
  }
  #print "firstColumnTest=@firstColumnTest\n";
  #print "secondColumnTest=@secondColumnTest\n\n";
}

$goldCount=0;
while ($lineGold=<GSFILE>)
{ 
  $goldCount++;
  chop $lineGold;
  # Remove leading spaces on line, otherwise the spaces become 
  # element 0 of the gold data instead of the first column of data.
  $lineGold =~ s/^\s*//;
  @goldData = (split(/[\s,]+/, $lineGold));
  #print "for goldCount=$goldCount, goldData=@goldData\n";
  #print "number of elements in goldData=$#goldData\n";

  # First row should only have one element. other rows should have two elements.
  # The variable $#goldData has the index of the last element in the goldData array,
  # rather than the size of goldData.  So, $#goldData+1 is the size of the array.
  if ( ($goldCount == 1) && ($#goldData+1 != 1))
  {
    print STDERR "First line of file $GOLDTXT should only have one element\n";
    print "Exit code = 2\n"; exit 2;
  }
  elsif ( ($goldCount > 1) && ($#goldData+1 != 2))
  {
    print STDERR "Line in file $GOLDTXT should have two elements\n";
    $retcode=2;
    print "Exit code = 2\n"; exit 2;
  }

  # Populate the comparison arrays.  The firstColumnGold array will have the same
  # size as the number of lines in the file.  The secondColumnGold array
  # will be one entry smaller.
  if ($goldCount == 1) 
  {
    $firstColumnGold[0]=$goldData[0];
  }
  else
  {
    $firstColumnGold[$goldCount-1]=$goldData[0];
    $secondColumnGold[$goldCount-2]=$goldData[1];
  }
}

close(GSFILE);
close(TESTFILE);

# sort the second column of the two namesfiles since they may be
# ordered differently on different platforms.
my @secondColumnTestSorted = sort @secondColumnTest;
my @secondColumnGoldSorted = sort @secondColumnGold;

#print "test,gold line counts = $testCount, $goldCount\n";
#print "firstColumnTest=@firstColumnTest\n";
#print "firstColumnGold=@firstColumnGold\n";
#print "secondColumnTestSorted=@secondColumnTestSorted\n";
#print "secondColumnGoldSorted=@secondColumnGoldSorted\n";

if ($testCount != $goldCount) 
{
  print STDERR "Output namesfile doesn't match the Gold namesfile\n";
  print STDERR "Output namesfile line count= $testCount\n";
  print STDERR "Gold namesfile line count= $goldCount\n";
  print "Exit code = 2\n"; exit 2;
}

$retcode=0;
# compare first columns
foreach $idx (0 .. $testCount-1)
{
  if ($firstColumnTest[$idx] ne $firstColumnGold[$idx])
  {
    print STDERR "Output namesfile doesn't match the Gold namesfile at idx=$idx\n";
    print STDERR "First column of output namesfile= $firstColumnTest[$idx]\n";
    print STDERR "First column of Gold namesfile= $firstColumnGold[$idx]\n";
    $retcode=2;
  }
}

# compare second columns.  Use sorted versions.
foreach $idx (0 .. $testCount-2)
{
  if ($secondColumnTestSorted[$idx] ne $secondColumnGoldSorted[$idx])
  {
    print STDERR "Output namesfile doesn't match the Gold namesfile at idx=$idx\n";
    print STDERR "Second column of sorted output namesfile= $secondColumnTestSorted[$idx]\n";
    print STDERR "Second column of sorted Gold namesfile= $secondColumnGoldSorted[$idx]\n";
    $retcode=2;
  }
}

print "Exit code = $retcode\n"; exit $retcode;
