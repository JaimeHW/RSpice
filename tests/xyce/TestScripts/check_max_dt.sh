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

# Since the shell script runs Xyce and the comparison program, it is
# responsible for capturing any error output from Xyce and the STDOUT & STDERR
# from the comparison program.  The outside script, run_xyce_regression,
# expects the STDERR output from Xyce to go into $CIRFILE.err, the STDOUT
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.  

$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

# this takes a value with modifiers (k, u, meg, mil, m, etc.) and
# returns the actual numeric value by multiplying by the appropriate scale
# factor
# Anything after the modifier is ignored, too, so "1 us" is returned as
# 1e-6, we ignore units.
sub modVal2Float
{
  my ($i)=$_[0];
  my ($mod)="";

  ($mod=$i) =~ s/([\d-+Ee]+)(.*)/$2/;
  if ( $mod ne "")
  {
    $mod=lc($mod);
  SWITCH: for($mod)
  {
    /t/ && do { $i = $i*1e12; last ;} ;
    /g/ && do { $i = $i*1e9; last ;} ;
    /meg/ && do { $i = $i*1e6; last ;} ;
    /k/ && do { $i = $i*1e3; last ;} ;
    /m/ && do { $i = $i*1e-3; last ;} ;
    /mil/ && do { $i = $i*25.4e-6; last ;} ;
    /u/ && do { $i = $i*1e-6; last ;} ;
    /n/ && do { $i = $i*1e-9; last ;} ;
    /p/ && do { $i = $i*1e-12; last ;} ;
    /f/ && do { $i = $i*1e-15; last ;} ;
# ignore unrecognized modifiers
  }
  }
  return $i;
}

#$CMD="$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err";
#if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10 }

open(CIRIN,"$CIRFILE");
while ($line = <CIRIN>)
{
  if ($line =~ m/^\.tran/i)
  {
    @linelist = split(" ",$line);
    $maxdt = modVal2Float($linelist[4]);
    $mag = 10**(log($maxdt)/log(10));
  }
}
close(CIRIN);

open(CIRPRN,"$CIRFILE.prn");
while ($line = <CIRPRN>)
{
  if ($line =~ m/^Index/) { next; }
  if ($line =~ m/^End/) { next; }
  @linelist = split(" ",$line);
  $time = $linelist[1];
  #printf ("time = %.20g\n",$time);
  if ($linelist[0] > 0)
  {
    $dt = $time - $time_old;
    #printf ("dt = %.20g\n",$dt);
    $diff = $dt-$maxdt;
    if ($diff > $zerotol*$mag)
    {
      print "Exit code = 2\n";
      exit 2
    }
  }
  else
  {
    $digits = $linelist[1];
    $digits = length($digits)-6;
    $zerotol = 10**(-$digits+1);
  }
  $time_old = $time;
}
close(CIRPRN);

# At this point the test has passed, so lets remove the Xyce error output.
`rm -f $CIRFILE.err`; 

print "Exit code = 0\n";
exit 0


