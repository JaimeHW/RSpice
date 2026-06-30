#!/usr/bin/env perl

# The input arguments to this script are:
# $ARGV[0] = location of Xyce binary
# $ARGV[1] = location of xyce_verify.pl script
# $ARGV[2] = location of compare script (NEVER USED!)
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
$XYCE_VERIFY=~ s/xyce_verify/ACComparator/;
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];

$XPLAT_DIFF = $ARGV[1];
$XPLAT_DIFF =~ s/xyce_verify/xplat_diff/;

# comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-2;  #1%
$zerotol=1e-7;
$freqreltol=1e-6;

# remove previous output file
system("rm -f $CIRFILE.FD.prn $CIRFILE.out $CIRFILE.err");

#run Xyce
print "Testing $CIRFILE file ...\n";
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
$retval = system($CMD);

if ($retval != 0)
{
  if ($retval & 127)
  {
    print "Exit code = 13\n"; 
    printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIRFILE; 
    exit 13;
  }
  else
  {
    print "Exit code = 10\n"; 
    printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE; 
    exit 10;
  }
}

$GOLDPRN =~ s/\.prn$//; # remove the .prn at the end.

$CMD="$XPLAT_DIFF -u -b $CIRFILE.res.gs $CIRFILE.res > $CIRFILE.res.out 2> $CIRFILE.res.err";
$retval = system("$CMD");

if ($retval != 0) 
{ 
    print STDERR "RES file mismatch!\n";
    print "Exit code = 2\n";
    exit 2;
}

#If that matches, do an AC compare of the regular FD output.
if (-f "$CIRFILE.FD.prn")
{
  $CMD="$XYCE_VERIFY $GOLDPRN.FD.prn $CIRFILE.FD.prn $abstol $reltol $zerotol $freqreltol";
  $retval = system($CMD);
  $retval = $retval >> 8;
  if ($retval == 0) 
  { 
    $retcode = 0;
  }
  else 
  {
    print STDERR "Comparator exited with exit code $retval\n";
    $retcode = 2;
  }
}
else 
{ 
  print STDERR "Missing output file: $CIRFILE.FD.prn\n";
  $retcode = 14; 
}

# if the main file matches the gold standard, try the "noFlip_P" circuit
# and check it against the main file's version.
$CIRFILE2=$CIRFILE;
$CIRFILE2 =~ s/.cir/_noFlip_P.cir/;
# remove previous output file
system("rm -f $CIRFILE2.FD.prn");
    
if ($retcode == 0)
{
    print "Testing $CIRFILE2 file ...\n";
    $CMD="$XYCE $CIRFILE2 > $CIRFILE2.out 2> $CIRFILE2.err ";
    $retval = system($CMD);
    
    if ($retval != 0) 
    {
      if ($retval & 127)
      {
        print "Exit code = 13\n"; 
        printf STDERR "Xyce crashed with signal %d on file %s\n",($retval&127),$CIRFILE; 
        exit 13;
      }
      else
      {
        print "Exit code = 10\n"; 
        printf STDERR "Xyce exited with exit code %d on %s\n",$retval>>8,$CIRFILE; 
        exit 10;
      }
    }

    $CMD="$XPLAT_DIFF -u -b $CIRFILE.res.gs $CIRFILE2.res > $CIRFILE2.res.out 2> $CIRFILE2.res.err";
    $retval = system("$CMD");
    
    if ($retval != 0) 
    { 
        print STDERR "RES file mismatch!\n";
        print "Exit code = 2\n";
        exit 2;
    }
    
    #If that matches, do an AC compare with the regular FD output.
    if (-f "$CIRFILE2.FD.prn")
    {
        $CMD="$XYCE_VERIFY $CIRFILE.FD.prn $CIRFILE2.FD.prn $abstol $reltol $zerotol $freqreltol";
        $retval = system($CMD);
        $retval = $retval >> 8;
        if ($retval == 0) 
        { 
          $retcode = 0;
        }
        else 
        {
          print STDERR "Comparator exited with exit code $retval\n"; 
          $retcode = 2;
        }
    }
    else 
    { 
      print STDERR "Missing output file: $CIRFILE2.FD.prn\n";
      $retcode = 14; 
    }
}

if ($retcode == 0)
{
    `rm *.res.out *.res.err`;
}

print "Exit code = $retcode\n"; exit $retcode;

