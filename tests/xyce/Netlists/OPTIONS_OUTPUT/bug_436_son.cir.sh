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
# output from comparison to go into $CIRFILE.prn.out and the STDERR output from
# comparison to go into $CIRFILE.prn.err.

# This test does not require anything by the Xyce binary and the circuit name
$XYCE=$ARGV[0];
#$XYCE_VERIFY=$ARGV[1];
#$XYCE_COMPARE=$ARGV[2];
$CIRFILE=$ARGV[3];
#$GOLDPRN=$ARGV[4];

#clean out old data, just in case
system("rm -f $CIRFILE.prn $CIRFILE.err");

#Run Xyce
$CMD="$XYCE $CIRFILE > $CIRFILE.out 2>$CIRFILE.err";
if (system($CMD) != 0) {
    `echo "Xyce EXITED WITH ERROR! on $CIRFILE" >> $CIRFILE.err`;
    $xyceexit=1;
}
if (defined ($xyceexit)) {print "Exit code = 10\n"; exit 10;}

#Check existence of output
if ( !(-f "$CIRFILE.prn")) {
    print STDERR "Missing output file $CIRFILE.prn\n";
    $xyceexit=14;
}
if (defined ($xyceexit)) {print "Exit code = $xyceexit\n"; exit $xyceexit;}


#The test is just this:  
# The circuit file says to run for 11ms, outputting ever 1ms for the first 9ms
# every .1ms from t=9 to t=10, then every .01ms (10us) after that.
# We need to see if it is doing that.
# I could hard-code these here, but I'll try to be smarter, and get the
# data from the netlist itself.
open(NETLIST,"<$CIRFILE") || die "Cannot open $CIRFILE for read.";
while(<NETLIST>)
{
    @fields=split(" ");
    if ($fields[0] =~ /\Q.tran/i)
    {
        $end_time=$fields[2];
    }
    elsif ($fields[0] =~ /\Q.options/i && $fields[1] =~ /\Qoutput/i)
    {
        shift(@fields);
        shift(@fields);
        if ($fields[0] =~ /initial_interval=([^ ]*)/i)
        {
            $initial_interval=modVal2Float($1);
            shift(@fields);

            while ($#fields > 0)
            {
                push(@times,modVal2Float($fields[0]));
                push(@intervals,modVal2Float($fields[1]));
                shift(@fields);
                shift(@fields);
            }
        }
    }
}
close(NETLIST);

# We now know what the netlist says should be the final time, and what
# output intervals were requested.
$t=0;
$interval=$initial_interval;

open(PRNFILE,"<$CIRFILE.prn") || die "Cannot open $CIRFILE.prn for read.";
while(<PRNFILE>)
{
    @fields=split(" ");

    if (!($fields[0] eq "Index" || $fields[0] eq "End"))
    {
        
        $time=$fields[1];
        
        
        if (abs($time - $t) > 1e-16)
        {
            print STDERR "Index $fields[0]: Expecting time $t, got time $time, difference is ";
            print STDERR $time-$t;
            print STDERR "\n";
            print "Exit code = 2\n"; exit 2;
        }
        
        if ($#times >= 0 )
        {
            if (abs($t - $times[0]) < 1e-16)
            {
                $interval = $intervals[0];
                shift(@times);
                shift(@intervals);
            }
        }
        
        $t += $interval;
    }
}

print "Exit code = 0\n";
exit 0;


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
