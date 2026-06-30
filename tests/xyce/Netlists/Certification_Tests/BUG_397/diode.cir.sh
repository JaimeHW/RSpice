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
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

$verbose = 1;
sub verbosePrint
{
  print @_ if $verbose;
}

`rm -f $CIRFILE.out.gs $CIRFILE.err.gs`;
$CMD="$XYCE $CIRFILE > $CIRFILE.out.gs 2> $CIRFILE.err.gs";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }

`rm -f $CIRFILE.out $CIRFILE.err`;
$CMD="$XYCE $CIRFILE -l sample.log > $CIRFILE.out 2> $CIRFILE.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }

$CMD="$XYCE $CIRFILE -l sample2.log -verbose sample2.verbose > $CIRFILE-sample2.out 2> $CIRFILE-sample2.err";
if (system("$CMD") != 0) { print "Exit code = 10\n"; exit 10; }

# This diff needs to be more flexible.
# First I'm going to remove two timing numbers from the output that screw up the diff
# Then I'm going to remove all empty lines and compare the remainder.

if ( not -s "$CIRFILE.out.gs" ) { print "Exit code = 14\n"; exit 14; }
if ( not -s "sample.log" ) { print "Exit code = 14\n"; exit 14; }
if ( not -s "sample2.log" ) { print "Exit code = 14\n"; exit 14; }
if ( not -s "sample2.verbose" ) { print "Exit code = 14\n"; exit 14; }

open(outgs,"<$CIRFILE.out.gs");
@s = <outgs>; 
close(outgs);
@goldoutlines = fixUpLogLines(@s);

open(logfile,"<sample.log");
@s = <logfile>;
close(logfile);
@sampleloglines = fixUpLogLines(@s);

open(log2file,"<sample2.log");
@s = <log2file>;
close(log2file);
@sample2loglines = fixUpLogLines(@s);

open(verbosefile,"<sample2.verbose");
@s = <verbosefile>;
close(verbosefile);
@sample2verboselines = fixUpLogLines(@s);

verbosePrint "======================================================\n";
verbosePrint "Xyce output:\n";
verbosePrint "======================================================\n";
verbosePrint @soldoutlines;
verbosePrint "======================================================\n";
verbosePrint "Log output:\n";
verbosePrint "======================================================\n";
verbosePrint @sampleloglines;
verbosePrint "======================================================\n";
verbosePrint "Log (with -verbose) output:\n";
verbosePrint "======================================================\n";
verbosePrint @sample2loglines;
verbosePrint "======================================================\n";
verbosePrint "Verbose (with -verbose) output:\n";
verbosePrint "======================================================\n";
verbosePrint @sample2verboselines;
verbosePrint "======================================================\n";

if (!open(sampleerrfile,">sample.log.err"))
{
  print "Exit code = 1\n"; 
  exit 1;
}

if (!open(sample2errfile,">sample2.log.err"))
{
  print "Exit code = 1\n"; 
  exit 1;
}

$numsame = 0;
$numsame2 = 0;
$numsameverbose = 0;
$numtotal = $#goldoutlines+1;
for (my $i; $i<=$#sampleloglines ; $i++)
{
  if ($goldoutlines[$i] eq $sampleloglines[$i]) 
  { 
    $numsame++; 
  }
  else
  {
    print sampleerrfile "The following two lines do not match:\n";
    print sampleerrfile "out:$i\n$goldoutlines[$i]";
    print sampleerrfile "log:$i\n$sampleloglines[$i]";
  }

  if ($goldoutlines[$i] eq $sample2loglines[$i]) 
  { 
    $numsame2++; 
  }
  else
  {
    print sampleerrfile "The following two lines do not match:\n";
    print sampleerrfile "out:$i\n$goldoutlines[$i]";
    print sampleerrfile "log:$i\n$sample2loglines[$i]";
  }

  if ($goldoutlines[$i] eq $sample2verboselines[$i]) 
  { 
    $numsameverbose++; 
  }
  else
  {
    print sample2errfile "The following two lines do not match:\n";
    print sample2errfile "out:$i\n$goldoutlines[$i]";
    print sample2errfile "log:$i\n$sample2verboselines[$i]";
  }
}
close(sampleerrfile);
close(sample2errfile);

if ($numsame == $numtotal) { print "Exit code = 0\n"; exit 0; } else { print "Exit code = 2\n"; exit 2; }
if ($numsame2 == $numtotal) { print "Exit code = 0\n"; exit 0; } else { print "Exit code = 2\n"; exit 2; }
if ($numsameverbose == $numtotal) { print "Exit code = 0\n"; exit 0; } else { print "Exit code = 2\n"; exit 2; }

print "Exit code = 1\n"; exit 1;

sub fixUpLogLines
{
  my (@lines) = @_;
  my ($line,@linesout);
  @linesout = undef;
  foreach $line (@lines)
  {

# STOP processing when hit the "Timing summary for ..." line
    if ($line =~ m/Timing summary of.*processor/) {last;}
    if (matchDate($line)) { next; } # remove date/time
    if ($line =~ m/^[\s]*$/) { next; }
    if ($line =~ m/Estimated time to completion/i) { next; }
    if ($line =~ m/Amesos.*Time: /i) { next; }
    # TVR: changed 9 Jun 08 to remove summary run times from comparison
    if ($line =~ m/^[\s]*total.*time/i) { next; }
    if ($line =~ m/total elapsed/i) { next; }
    if ($line =~ m/Current system time/i) { next; }
    $line =~ s/([eE][+-])0([0-9]{2})/\1\2/g; # change E+003 -> E+03
    $line =~ s/\r//g; # Fix up newline issues in windows.

    # Remove decimal and floating point numbers
    $line =~ s/\d+\.\d+//g;
    $line =~ s/[eE][+-][0-9]+//g;

    $line =~ s/ 0 /  /g;     # remove bare zero
    push @linesout,$line;
  }
  return @linesout;
}

# This routine returns true if there is a date somewhere inside the 
# input string
# It recognizes the following formats:
# *nix `date`:  "Tue May  2 16:29:05 MDT 2006"
# *nix `date` without zone:  "Tue May  2 16:29:05 2006"
# Windows date:  "Tue 05/02/2006"
# Windows time:  "02:15 PM"
sub matchDate
{
  my ($line) = @_;
  my $yes;
  my @dayofweek = ( "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat" );
  my @months = ( "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec" );
  my $rweek = join("|",@dayofweek); $rweek = "($rweek)";
  my $rmonth = join("|",@months); $rmonth = "($rmonth)";
  my $rday = "([012]{0,1}[0-9]|[3][01])";
  my $rhr = "([01][0-9]|[2][0-4])";
  my $rmin = "[0-5][0-9]";
  my $rsec = "[0-5][0-9]";
  my $rtzone = "[PMCE][SD]T";
  my $rmonthnum = "([0][1-9]|[1][0-2])";
  my $rdaynum = "([0][1-9]|[1-2][0-9]|[3][01])";
  my $rampm = "[AP]M";
  my $ryr = "[0-9]{4}";
  if ($line =~ m/$rweek\s+$rmonth\s+$rday\s+$rhr:$rmin:$rsec\s+$rtzone\s+$ryr/i)
  {  $yes = 1; }
  elsif ($line =~ m/$rweek\s+$rmonth\s+$rday\s+$rhr:$rmin:$rsec\s+$ryr/i)
  {  $yes = 1; }
  elsif ($line =~ m/$rweek\s+$rmonth\s+$rdaynum\s+$rhr:$rmin:$rsec\s+$ryr/i)
  {  $yes = 1; }
  elsif ($line =~ m/$rweek\s+$rmonthnum[\/]$rdaynum[\/]$ryr/)
  {  $yes = 1; }
  elsif ($line =~ m/$rhr:$rmin\s+$rampm/)
  {  $yes = 1; }
  if (defined($yes))
  {
    verbosePrint "Found date/time in line: $line";
  }
  else
  {
    verbosePrint "No date/time in line: $line";
  }
  return $yes;
}

