#!/usr/bin/env perl

use XyceRegression::Tools;

$Tools = XyceRegression::Tools->new();

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
# output from comparison to go into $CIRFILE.TD.prn.out and the STDERR output from
# comparison to go into $CIRFILE.TD.prn.err.  

$XYCE=$ARGV[0];
$XYCE_VERIFY=$ARGV[1];
$XYCE_VERIFY=~ s/xyce_verify/ACComparator/; 
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
$GOLDPRN=$ARGV[4];
$CIRFILE_PROBE="bug272_probe.cir";

# comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-3;  #0.1%
$zerotol=1e-8;
$freqreltol=1e-6;

$CMD="$XYCE $CIRFILE > $CIRFILE.out ";
$retval = system($CMD);
# check that Xyce exited without error
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

# check that Xyce made a TD.prn file
if (not -s "$CIRFILE.TD.prn" ) { print "No $CIRFILE.TD.prn\nExit code = 14\n"; exit 14; }
# check that Xyce made a FD.prn file
if (not -s "$CIRFILE.FD.prn" ) { print "No $CIRFILE.FD.prn\nExit code = 14\n"; exit 14; }

# check that Xyce can produce PROBE format 
# cant use wrapXyce because it expects a "prn" file result.
# $retval=$Tools->wrapXyce($XYCE,$CIRFILE_PROBE);
$CMD="$XYCE $CIRFILE_PROBE > $CIRFILE_PROBE.out ";
$retval = system($CMD);
# check that Xyce exited without error
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
# check that Xyce made a prn file
if (not -s "$CIRFILE_PROBE.csd" ) { print "No probe file $CIRFILE_PROBE.csd.\nExit code = 14\n"; exit 14; }


#
# now based on the prn file, make a comparison file 
#

open(INPUT,"$CIRFILE.FD.prn");
open(OUTPUT,">$CIRFILE.FD.prn.gs");
while ($line = <INPUT>)
{
  # Handle first and last lines:
  if ($line =~ m/^[IE]/) 
  { 
    print OUTPUT "$line"; 
    next; 
  }
  @s = split(" ",$line);
  $a = 0;
  # .print line is: 
  #.print AC V(Ve) VR(Ve) VI(Ve) VM(Ve) VP(Ve) VDB(Ve) 
  #+ V(out) VR(out) VI(out) VM(out) VP(out) VDB(out)
  #
  # the V(Ve) expands to Re(V(Ve)) and Im(V(Ve)) from which we 
  # can calculate VR(Ve) VI(Ve) VM(Ve) VP(Ve) and VDB(Ve) 
  # and likewise for V(out).
  #
  # Note that INDEX and Frequency are in s[0] and s[1] 
  #
  $R_Ve = $s[2];
  $I_Ve = $s[3];
  $M_Ve = sqrt( $R_Ve * $R_Ve + $I_Ve * $I_Ve );
  $P_Ve = atan2( $I_Ve, $R_Ve ); 
  $DB_Ve = 20*log( sqrt($R_Ve * $R_Ve + $I_Ve * $I_Ve) ) / log(10);
  $R_Vout = $s[9];
  $I_Vout = $s[10];
  $M_Vout = sqrt( $R_Vout * $R_Vout + $I_Vout * $I_Vout );
  $P_Vout = atan2( $I_Vout, $R_Vout ); 
  $DB_Vout = 20*log( sqrt($R_Vout * $R_Vout + $I_Vout * $I_Vout) ) / log(10);
  printf OUTPUT "%3d   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e   %14.8e\n",
    $s[0],$s[1],$s[2],$s[3],$R_Ve,$I_Ve,$M_Ve,$P_Ve,$DB_Ve,
    $s[9],$s[10],$R_Vout,$I_Vout,$M_Vout,$P_Vout,$DB_Vout;
}
close(INPUT);
close(OUTPUT);

#
# call xyce-verify to compare result to generated gold standard.
# lower byte is not relevent here.  xyce_verify's return code is in 
# upper byte so shift it by 8 bits.
#
$CMD="$XYCE_VERIFY $CIRFILE.FD.prn.gs $CIRFILE.FD.prn $abstol $reltol $zerotol $freqreltol";
$retval = system($CMD);
if ($retval != 0) 
{ 
  print STDERR "FD output failed compare to gold standard.\n";
  print "Exit code = 2\n"; 
  exit 2; 
}

# convert the probe format file to a prn format that we can 
# then compare using xyce_verify
open(INPUT,"$CIRFILE_PROBE.csd");
open(OUTPUT,">$CIRFILE_PROBE.FD.prn");
my @columnNames;
push @columnNames, "Index";
my @expandedColumns;
my $headerIndex = 0;
my $index = 0;
$line = <INPUT>;
my $EOF_Found=0;
my $needNewLineRead=1;
while ($EOF_Found == 0)
{
  # Handle first and last lines:
  # Output header in prn format
  if ($line =~ m/SWEEPVAR/)
  {
    # FREQ should be the sweepvar
    # capture that here
    chomp $line;
    @s = split("'",$line);
    push @columnNames, @s[1];
    $needNewLineRead=1;
  }
  elsif ($line =~ m/^#N/) 
  { 
    # column names 
    my $headerDone = 0;
    
    while ( $headerDone == 0 )
    {
      $line = <INPUT>;
      if( $line =~ /^#/ )
      {
        $headerDone = 1;
      }
      else
      {
        chomp $line;
        @s = split("'",$line);
        foreach $name (@s)
        {
          # if name starts with "V(" then it must be 
          # expanded into Re(V(...)) and Im(V(...)) components
          # in translating to FD.prn format as that format doesn't 
          # have a complex type.
          if ($name =~ /\w/) 
          {
            $headerIndex++;
            if( ($name =~ /^V\(/i) )
            {
              $nameRe = "Re(" . $name . ")";
              $nameIm = "Im(" . $name . ")";
              push @columnNames, $nameRe;
              push @columnNames, $nameIm;
              # remember that this column was expanded so that
              # both real and imaginary parts of the output can 
              # be written to the FD.prn file
              push @expandedColumns, $headerIndex;
            }
            else
            {
              push @columnNames, $name;
            }
          }
        }
      }
    
    }
    foreach $name (@columnNames)
    {
      print OUTPUT "$name  "; 
    }
    print OUTPUT "\n"; 
  }

  if($line =~ m/^#C/) 
  { 
    # on a data line.  grab individual values for output 
    # on one line of a prn file. 
    my $lineDone = 0;
    my @columnValues;
    push @columnValues, $index;
    $index++;
    chomp $line;
    @s = split(" ",$line);
    push( @columnValues, $s[1] );
        
    while ( $lineDone == 0 )
    {
      $line = <INPUT>;
      
      if( $line =~ /^#/ )
      {
        $lineDone = 1;
      }
      else
      {
        chomp $line;
        @vals = split / /,$line;
        foreach $name (@vals)
        {
          if( length( $name ) > 0 )
          {
            @s = split /[\/:]/, $name;
            # $s[0] -- is the real component
            # $s[1] -- is the imaginary component
            # $s[2] -- is the output index 0 .. N
            $foundExpandedColumn=0;
            foreach $colNum (@expandedColumns)
            {
              if( $colNum == $s[2] )
              {
                $foundExpandedColumn=1;
                break;
              }
            }
            push @columnValues, $s[0];
            if ( $foundExpandedColumn == 1 )
            {
              push @columnValues, $s[1];
            }
          }
        }
      }
    
    }
    foreach $name (@columnValues)
    {
      print OUTPUT "$name  "; 
    }
    print OUTPUT "\n"; 
    $needNewLineRead=0;
  }
  if( $line =~ /#;/ )
  {
    $EOF_Found=1;
  }
  elsif( $needNewLineRead == 1 )
  {
    $line = <INPUT>
  }
}
print OUTPUT "End of Xyce(TM) Simulation\n";
close(INPUT);
close(OUTPUT);

# comparison tolerances for ACComparator.pl
$abstol=1e-6;
$reltol=1e-3;  #0.1%
$zerotol=1e-7; # this is a different zerotol than the previous comparison
$freqreltol=1e-6;

#
# call xyce-verify to compare result to generated gold standard.
# lower byte is not relevent here.  xyce_verify's return code is in 
# upper byte so shift it by 8 bits.
#
$CMD="$XYCE_VERIFY $CIRFILE.FD.prn.gs $CIRFILE_PROBE.FD.prn $abstol $reltol $zerotol $freqreltol";
$retval = system($CMD);
if ($retval != 0) 
{ 
  print STDERR "Probe output failed compare to gold standard.\n";
  print "Exit code = 2\n"; 
  exit 2; 
}
print "Exit code = 0\n"; 
exit 0;
