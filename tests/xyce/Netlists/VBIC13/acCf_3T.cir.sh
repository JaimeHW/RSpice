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
$abstol=1e-8;
$reltol=1e-5;  
$zerotol=1e-14;
$freqreltol=1e-6;

# remove previous output file
system("rm -f $CIRFILE.FD.prn $CIRFILE.out $CIRFILE.err");

# run Xyce
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

$GOLDPRN =~ s/\.cir\.prn$//; # remove the .cir.prn at the end.

#If that matches, massage the data into the format used by the original gold standard, then compare
if (-f "$CIRFILE.FD.prn")
{
    &massageFile("$CIRFILE.FD.prn","$CIRFILE.FD");
    $CMD="$XYCE_VERIFY --gsformat=otherprn $GOLDPRN.FD $CIRFILE.FD $abstol $reltol $zerotol $freqreltol";
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

# if the main file matches the gold standard, try the "noFlip_P" and "m" variants
# and check them against the main file's version.
foreach $variant ("noFlip_P","m")
{
    $CIRFILE_base=$CIRFILE;
    $CIRFILE_base =~ s/.cir//;
    $CIRFILE2 = $CIRFILE_base."_$variant".".cir";
    
    # remove previous output file
    system("rm -f $CIRFILE2.FD.prn");
    
    if ($retcode == 0)
    {
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
        
        #do an AC compare with the regular FD output.
        if (-f "$CIRFILE2.FD.prn")
        {
            &massageFile("$CIRFILE2.FD.prn","$CIRFILE2.FD");
            $CMD="$XYCE_VERIFY --gsformat=otherprn $CIRFILE.FD $CIRFILE2.FD $abstol $reltol $zerotol $freqreltol";
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
}

print "Exit code = $retcode\n"; exit $retcode;

# Very test-specific function.  The gold standard shipped with VBIC has freq, g(c,b) and c(c,b) data.
# We crunch our Xyce PRN output into this format, which involves throwing away a lot of data
sub massageFile
{
    my ($infile,$outfile)=@_;
    my @Field;
    my $omega, $twoPi,$index;
    my $g,$c;
    $twoPi = 8.0*atan2(1.0,1.0);
    
    open(INFILE,"$infile") || die("Cannot open input file $infile");
    open(OUTFILE,">$outfile") || die("cannot open output file $outfile");

    while (<INFILE>)
    {
        chomp;
        s/^\s+//;s/\s+$//;s/,/ /g;
        if (/^Index/i) 
        {
            print OUTFILE "Index   Freq   g(c,b)  c(c,b)\n";
            $index = 0;
            next;
        }

        @Field=split;
        next if ($Field[0] eq "End") ;

        $omega = $twoPi*$Field[1];

        # G(c,b) is just the Re(I(V_C_B)) column, column 2
        # C(c,b) is just -Im(I(V_C_B))/omega, column 3, divided by omega.
        $g = $Field[2];
        $c = -$Field[3]/$omega;

        print OUTFILE "$index    $Field[1]    $g    $c\n";
        $index++;
    }

}
