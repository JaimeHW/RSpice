#
# This is a set of tools for use in regression testing 
# 

package XyceRegression::ToolsTestSetUp;
use strict;
use Cwd;

use POSIX "sys_wait_h";

# variables defined in this class:
my ($controlC);

my %status2MessageHash = ( 
    0 => "passed", 
    1 => "SHELL SCRIPT EXITED WITH ERROR",
    2 => "FAILED",
   10 => "EXITED WITH ERROR",
   11 => "TEST REQUIRES MANUAL VERIFICATION",
   12 => "STALLED",
   13 => "XYCE CRASHED",
   14 => "NO PRN FILE",
   15 => "TIME LIMIT",
   16 => "CONTROL-C",
   17 => "NO MEASURE FILE",
   18 => "DAKOTA NOT FOUND"
   );

#Constructor
sub new
{
    my ($class) = @_;
    my $self={
     'childwait' => 1800, # 30 minutes for total job 
     'childprogress' => 1800, # 30 minutes between updates to output file
     'dofork' => 0,
     'verbose' => undef,
     'debug' => undef,
     'quiet' => undef,
     'indent' => "",
     'resultfile' => "",
     'thread_id' => 0,
     'exitcodeworkaround' => undef,
     'wdir' => undef
    };  
    # initialize working directory so that child processes 
    # that make a Tools object to call wrapXyce or runXyce 
    # are set up correctly.
    $self->{wdir}= getcwd;
    bless $self, $class;
    return $self;
}

sub setVerbose
{
  my ($self,$verboseFlag) = @_;
  if (defined($verboseFlag)) { $self->{verbose} = 1; }
  else { undef $self->{verbose}; }
}

sub setDebug
{
  my ($self,$debugFlag) = @_;
  if (defined($debugFlag)) { $self->{debug} = 1; }
  else { undef $self->{debug}; }
}

sub setQuiet
{
  my ($self,$quietFlag) = @_;
  if (defined($quietFlag)) { $self->{quiet} = 1; }
  else { undef $self->{quiet}; }
}

sub setFork
{
  my ($self,$forkFlag) = @_;
  if (defined($forkFlag)) { $self->{dofork} = 1; }
  else { undef $self->{dofork}; }
}

sub setResultFile
{
  my ($self,$result) = @_;
  $self->{resultfile} = $result;
  open(resultfile, ">$self->{resultfile}") or die "ERROR cannot open $self->{resultfile}\n";
}

sub closeResultFile
{
  my ($self) = @_;
  close(resultfile);
}

sub setIndent
{
  my ($self,$indentflag) = @_;
  $self->{indent} = $indentflag;
}

#sub setThreadId
#{
#  my ($self, $id) = @_;
#  $self->{thread_id} = $id;
#}

#sub getThreadId
#{
#  my ($self) = @_;
#  return $self->{thread_id};
#}

sub setWorkDir
{
  my ($self,$dir) = @_;
  $self->{wdir} = $dir;
}

sub getWorkDir
{
  my ($self) = @_;
  return $self->{wdir};
}

sub setExitCodeWorkAround
{
  my ($self,$exitcodeFlag) = @_;
  if (defined($exitcodeFlag)) { $self->{exitcodeworkaround} = 1; }
  else { undef $self->{exitcodeworkaround}; }
}

sub setChildWait
{
  my ($self,$waittime) = @_;
  $self->{childwait} = $waittime;
}

sub getChildWait 
{
  my ($self) = @_;
  return $self->{childwait};
}

sub setChildProgress
{
  my ($self,$waittime) = @_;
  $self->{childprogress} = $waittime;
}

sub getChildProgress
{
  my ($self) = @_;
  return $self->{childprogress};
}

sub defaultPrint
{
  my ($self) = @_;
  shift @_;
  printf @_;
  $self->resultPrint(@_);
}

sub resultPrint
{
  my ($self) = @_;
  shift @_;
  if (defined($self->{resultfile}))
  {
    printf resultfile @_;
  }
}

sub iPrint
{
  my ($self) = @_;
  shift @_;
  printf "$self->{indent}";
  $self->defaultPrint(@_);
}

sub verbosePrint
{
  my ($self) = @_;
  shift @_;
  $self->iPrint(@_) if ($self->{verbose} or $self->{debug});
}

sub debugPrint
{
  my ($self) = @_;
  shift @_;
  $self->iPrint(@_) if ($self->{debug});
}

sub quietPrint
{
  my ($self) = @_;
  shift @_;
  $self->iPrint(@_) if (not $self->{quiet});
}

$controlC = 0;
$SIG{INT} = sub { $controlC++; };
sub getControlC
{
  my ($self)= @_;
  return $controlC;
}

sub setControlC
{
  my ($self,$val) = @_;
  $controlC = $val;
}

sub checkControlExit
{
  my ($self) = @_;
  if ($controlC > 0) {
    return 1;
  }
  return undef;
}

sub userControlExit
{
  my ($self) = @_;
  if ($self->checkControlExit()) {
    print "\nQuit Now? (Y/N)";
    my $quit = <STDIN>;
    chomp($quit);
    if ($quit =~ m/[yY]/) {
      print "Exiting immediately!\n";
      exit 0;
    } else {
      print "Continuing...\n\n";
      $controlC=0;
    }
  }
}

sub modVal2Float
{
  my ($self) = @_;
  shift @_;
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

## WRAPPER FUNCTIONS
## These three tools, wrapXyce, wrapXyceAndVerify, and wrapXyceAndVerifyPLGS
## are the primary functions of the Tools class that script writers should
## use in scripts to run and test Xyce.
#
## wrapXyce
## Arguments:
##   - path to Xyce
##   - netlist file  
## Optional third argument: output file name produced by Xyce.  Defaults to
##    $ckt.prn
##
## This wrapper runs Xyce, checks that the output file is produced, and
## returns an appropriate error code.  See the %status2MessageHash for the
## meaning of return codes
##
## This wrapper is complicated because it has to deal with both serial and
## parallel runs.
##  
## If we run Xyce in serial, then we can trust the exit code returned by system.
## In parallel, mpirun can lie.
## This wrapper will detect if we are running in parallel or serial and figure
## out the right exit code in the parallel case.
## parallel strategy:
##   1.  If "*** Xyce Abort ***" is in $ckt.err then we exit with 10
##   2.  good: "End of Xyce(TM)" is in $ckt.out 
##   3.  good: "End of Xyce(TM)" is the last line in $ckt.prn
##   Problem:  There is the last exit in Xyce.C which checks that memory was
##   deleted correctly and all that.  This is where I want to print out
##   something useful just on the "output" processor.
##
## MODDED 20 April 2012: Added optional third argument.  If present, this
## is used as the name of the "prn" file to test for rather than the
## "circuit.prn" default name.
#sub wrapXyce
#{
#  my ($self,$XYCEPATH,$ckt,$cktprn)=@_;
#  #$self->debugPrint("XyceRegression::Tools::wrapXyce:  \$XYCEPATH = $XYCEPATH\n");
#  my ($pid,$exitcode,$time0,$time1,$dead_pid,$filesize,$filesize_old,$line,$retval);
#  my ($timestart);
#  my $id = $self->getThreadId();
#  my $wdir= $self->getWorkDir();
#  my $wdirPrefix="";
#  if( defined($self->getWorkDir()) ) {
#    $wdirPrefix = $self->getWorkDir() . "/";
#    $XYCEPATH = "cd $wdir; " . $XYCEPATH;
#  }
#  
#  # the following is to deal with $ckt="path with spaces/diode.cir"
#  my $cktnoquotes=$ckt;
#  
#  
#  if ($cktnoquotes =~ s/"//g)
#  {
#    $self->debugPrint("Removed quotes from incoming circuit name\n");
#  }
#  $cktprn = "$cktnoquotes.prn" if (!defined($cktprn));
#  my $ckterr = "$cktnoquotes.err";
#  my $cktout = "$cktnoquotes.out";
#  $self->debugPrint("\$cktprn = >>$cktprn<<\n");
#  $self->debugPrint("\$ckterr = >>$ckterr<<\n");
#  $self->debugPrint("\$cktout = >>$cktout<<\n");
#
#  `rm -f "$ckterr" "$cktprn" "$cktout"`;
#  if (defined($self->{dofork}))
#  {
#    $pid = $self->runXyce($ckt,$XYCEPATH);
#    $retval = -1;
#    #debugPrint "pid = $pid\n";
#    # how do I get exit code in serial?
#    # how long should I wait for dead processes?
#    #    How about this:  Wait 2 minutes, then start looking at the .prn file.
#    #    If it does not grow in size over the next 2 minutes, then call it quits.
#    #    Otherwise keep checking it every 2 minutes.  
#    #  Here's how you check the stats on a file 
#    #    ($dev,$ino,$mode,$nlink,$uid,$gid,$rdev,$size,$atime,$mtime,$ctime,$blksize,$blocks) = stat($filename)
#    $exitcode = undef;
#    $time0 = time;
#    $timestart = $time0;
#    my $killjob = undef;
#    while (not defined $exitcode)
#    {
#      $dead_pid = waitpid($pid,&WNOHANG); # == $pid on completion
#      if ($dead_pid == $pid) 
#      { 
#        $self->debugPrint("pid $pid exited\n");
#        $exitcode = 0; 
#      }
#      $time1 = time;
#      my $elapsed_time = $time1-$time0;
#      $self->debugPrint("elapsed time = $elapsed_time\n");
#      my $total_time = $time1 - $timestart;
#      $self->debugPrint("total time = $total_time\n");
#      if ($total_time >= $self->{childwait}) 
#      {
#        $self->debugPrint("Killing $pid because total alloted time ($self->{childwait}) exceeded!\n");
#        $exitcode = 15;
#        $killjob = 1;
#      }
#      elsif ($elapsed_time >= $self->{childprogress})
#      {
#        $time0 = time;
#        if ( -f "$wdirPrefix$cktprn" )
#        {
#          $filesize = (stat("$wdirPrefix$cktprn"))[7];
#          $self->debugPrint("$cktprn size = $filesize\n");
#          if (defined ($filesize_old) and ($filesize <= $filesize_old))
#          {
#            $self->debugPrint("Killing $pid because no new lines added to $cktprn in $self->{childprogress} seconds\n");
#            $exitcode = 12;
#            $killjob = 1;
#          }
#          else
#          {
#            $self->debugPrint("Still waiting for finish, $cktprn is created or growing.\n");
#          }
#          $filesize_old = $filesize;
#        }
#        else
#        {
#          $self->debugPrint("Killing $pid because no $cktprn file created in $self->{childprogress} seconds\n");
#          $exitcode = 14;
#          $killjob = 1;
#        }
#      }
#      if ($self->checkControlExit())
#      {
#        $self->debugPrint("Killing $pid because Control-C pressed.\n");
#        $exitcode = 16;
#        $killjob = 1;
#        $controlC--;
#      }
#      if ($killjob) 
#      {
#        kill -15, $pid; # TERM -> process group
#        $dead_pid = waitpid($pid,0);
#      }
#      select(undef, undef, undef, 0.01); # this is a non-cpu intensive sleep of 10ms
#    }
#    # We have now exited the Xyce run.  To handle cases where mpich's mpirun lets
#    # child processes sit around, lets kill the whole process group.
#    kill -15, $pid;
#  }
#  else
#  {
#    #my $CMD = "$XYCEPATH $ckt > /dev/null 2> $ckt.err";
#    my $CMD = "$XYCEPATH $ckt > \"$cktout\" 2> \"$ckterr\"";
#    $self->debugPrint("XyceRegression::Tools::wrapXyce:  \$CMD = >>$CMD<<\n");
#    $retval = system("$CMD");
#    $self->debugPrint("XyceRegression::Tools::wrapXyce:  Finished running Xyce, retval = $retval\n");
#    $exitcode = 0;
#  }
#  # This is where we determine the exit code.
#  if ($exitcode != 0) { return $exitcode }
#  if (($XYCEPATH =~ m/mpi/i) or defined($self->{dofork})) 
#  { 
#    $self->debugPrint("Using workaround for exitcodes.\n");
#    $self->{exitcodeworkaround} = 1; 
#  }
#  if (not defined($self->{exitcodeworkaround})) 
#  { 
#    $retval = $retval >> 8 ;
#    if ($retval != 0) {
#      $retval = 10;
#    }
#    return($retval); 
#  } 
#  $exitcode = -10;
#  $retval = -10;
#  if ( -f "$wdirPrefix$ckterr" )
#  {
#    open(errfile,"$wdirPrefix$ckterr") or warn "WARNING:  1 Cannot open $wdirPrefix$ckterr\n";
#    while ($line = <errfile>)
#    {
#      if ($line =~ m/Xyce Abort/)
#      {
#        $exitcode = 10;
#        $self->debugPrint("Found \"Xyce Abort\" in $ckterr, exiting with 10\n");
#      }
#    }
#    close(errfile);
#  }
#  if ( ($exitcode == -10) and (-f "$wdirPrefix$cktout") )
#  {
#    open(outfile,"$wdirPrefix$cktout") or warn "WARNING:  2 Cannot open $wdirPrefix$cktout\n";
#    while (($exitcode == -10) and ($line = <outfile>))
#    {
#      if ($line =~ m/Xyce Abort/)
#      {
#        $exitcode = 10;
#        $self->debugPrint("Found \"Xyce Abort\" in $cktout, exiting with 10\n");
#      }
#      elsif ($line =~ m/SIGSEGV/)
#      {
#        $exitcode = 13;
#        $self->debugPrint("Found SIGSEGV in $cktout, exiting with 13\n");
#      }
#      elsif ($line =~ m/Devel fatal/)
#      {
#        # Note:  if "Xyce Abort" is not caught above, and we find "Dev Fatal" here, THEN the code had a segmentation fault which is not printed to the screen.
#        $exitcode = 13;
#        $self->debugPrint("Found Dev Fatal in $cktout and did NOT find Xyce Abort, exiting with 13\n");
#      }
#    }
#    close(outfile);
#  }
#  if ($exitcode == -10) # no "Xyce Abort" message in $ckterr or $cktout
#  {
#    if ( (-f "$wdirPrefix$cktprn") )
#    {
#      # Note:  This only works on standard output format, it fails for PROBE format.
#      my $prnstatus = `grep "End of Xyce[(]TM[)]" "$wdirPrefix$cktprn"`;
#      my $prnstatus2 = `grep "#;" "$wdirPrefix$cktprn"`;
#      if ( $prnstatus or $prnstatus2 )
#      {
#        $exitcode = 0;
#        $self->debugPrint("Found \"End of Xyce(TM)\" or \"#;\" in $cktprn, exiting with 0\n");
#      }
#      else
#      {
#	# I should check for a complete PROBE format output file here.
#        $exitcode = 13; # Xyce produced .prn file but did not finish writing to it.
#        $self->debugPrint("Found $cktprn, but no \"End of Xyce(TM)\" or \"#;\" at the end, exiting with 13\n");
#      }
#    }
#    else
#    {
#      my $wd=`pwd`;
#      $self->debugPrint("No $cktprn file found in $wd, exiting with 14\n");
#      $exitcode = 14; # did not create $cktprn file
#    }
#  }
#  $self->debugPrint ("wrapXyce returning $exitcode\n");
#  return $exitcode;
#}
#
## wrapXyceAndVerify
##
## Arguments:
##   - path to Xyce executable
##   - path to comparator
##   - path to gold standard file
##   - path to netlist
##
## Optional fifth argument:  output file name.  Default is "$ckt.prn"
##
##   Runs the given Xyce executable on the given netlist and checks that
##   the output file is produced, by calling wrapXyce, then compares it to the 
##   given gold standard with the given comapartor.
#sub wrapXyceAndVerify {
#  my ($self,$XYCEPATH,$XYCEVERIFY,$GOLDPRN,$ckt,$cktprn)=@_;
#  my $retval = 1;
#  my $id = $self->getThreadId();
#  my $wdir=$self->getWorkDir();
#  my $wdirPrefix="";
#  if( defined($self->getWorkDir()) ) {
#    $wdirPrefix = $self->getWorkDir() . "/";
#  }
#  
#  $cktprn="$ckt.prn" if (!defined($cktprn));
#
#  $retval = $self->wrapXyce($XYCEPATH,$ckt,$cktprn);
#  if ($retval == 0) { 
#    my $CMD="$XYCEVERIFY $wdirPrefix$ckt $GOLDPRN $wdirPrefix$cktprn > $wdirPrefix$ckt.prn.out 2> $wdirPrefix$ckt.prn.err";
#    $retval = system("$CMD");
#    if ($retval != 0)
#    {
#        $retval = 2;  # xyce_verify returns tons of codes when it fails, none
#                      # are what we are expected to return.
#    }
#  }
#  return $retval;
#}
#
## wrapXyceAndVerifyPLGS
## Similar to wrapXyceAndVerify, but uses a gold-standard-generation script
## to create the gold standard.
##
## Arguments:
##  - Xyce path
##  - comparator path
##  - path to gold-standard-generating script
##  - netlist path
##
## Optional argument: output file name, defaults to $ckt.prn
##
## In this script, $GOLDPL is a provided gold standard creation script which
## optionally takes the name of the prn file produce by Xyce on the tested
## netlist.  Normally this script is named "$ckt.prn.gs.pl" and it produces a
## file named "$ckt.prn.gs".
##
## Runs Xyce and checks for output file using wrapXyce.  Then runs the
## gold standard script and checks using the given comparator. 
#sub wrapXyceAndVerifyPLGS {
#  my ($self,$XYCEPATH,$XYCEVERIFY,$GOLDPL,$ckt,$cktprn)=@_;
#  my $id = $self->getThreadId();
#  my $wdir=$self->getWorkDir();
#  my $wdirPrefix="";
#  if( defined($self->getWorkDir()) ) {
#    $wdirPrefix = $self->getWorkDir() . "/";
#  }
#  
#  my $retval = 1;
#  $cktprn="$ckt.prn" if (!defined($cktprn));
#
#  $retval = $self->wrapXyce($XYCEPATH,$ckt,$cktprn);
#  if ($retval == 0) {
#    my $CMD = "$GOLDPL $cktprn";
#    $retval = system("$CMD");
#    if ($retval == 0) {
#      my $CMD="$XYCEVERIFY $ckt $ckt.prn.gs $cktprn > $ckt.prn.out 2>$ckt.prn.err";
#      if( defined($self->getWorkDir()) ) {
#        $CMD= "cd $wdirPrefix; " . $CMD;
#      }
#      $retval = system("$CMD");
#      if ($retval != 0)
#      {
#          $retval = 2;  # xyce_verify returns tons of codes when it fails, none
#          # are what we are expected to return.
#      }
#    }
#  }
#  return $retval;
#}
#
## runXyce
##
## THIS HELPER FUNCTION IS NOT INTENDED FOR DIRECT USE IN TESTING SCRIPTS.
## It is a low-level function used by the various "wrapXyce*" functions.
#sub runXyce
#{
#  my ($self,$ckt,$XYCEPATH) = @_;
#  my $pid;
#  my $id = $self->getThreadId();
#  my $wdir=$self->getWorkDir();
#  my $wdirPrefix="";
#  if( defined($self->getWorkDir()) ) {
#    $wdirPrefix = $self->getWorkDir() . "/";
#  }
#  
#  
#  use Errno qw(EAGAIN);
#  $self->debugPrint("XyceRegression::Tools::runXyce \$ckt = >>$ckt<<\n");
#  fork:
#  {
#    if ($pid=fork) 
#    {
#      $self->debugPrint("This is the parent, returning pid = $pid\n");
#      return $pid
#    }
#    elsif (defined $pid)
#    {
#      $self->debugPrint("Child $$:  This is the child with pid $$.\n");
#      #close(STDOUT);
#      #close(STDERR);
#      setpgrp(0,0); # This sets process group so I can kill this + children
#      $self->debugPrint("Child $$:  \$ckt = >>$ckt<<\n");
#      my $cktnoquotes = $ckt;
#      $cktnoquotes =~ s/"//g;
#      $self->debugPrint("Child $$:  Redirecting STDOUT to $cktnoquotes.out\n");
#      open STDOUT, ">$cktnoquotes.out" or die "Child $$:  Can't redirect STDOUT: $!";
#      $self->debugPrint("Child $$:  Redirecting STDERR to $cktnoquotes.err\n");
#      open STDERR, ">$cktnoquotes.err" or die "Child $$:  Can't redirect STDERR: $!";
#      if( defined($self->getWorkDir()) ) {
#        exec "cd $wdir;$XYCEPATH $ckt";
#      } else {
#        exec "$XYCEPATH $ckt";
#      }
#      exit 0; # this is for when exec fails
#    }
#    elsif ($! == EAGAIN)
#    {
#      sleep 3;
#      redo fork;
#    }
#    else
#    {
#      # We could not fork for some reason
#      return -1;
#    }
#  }
#}
#
sub wrapShellScript
{
  my ($self,$SHELLFILE,$XYCEPATH,$XYCE_VERIFY,$XYCE_COMPARE,$ckt,$GOLDPRNFILE,$globaltaglist) = @_;
  my $wdir=$self->getWorkDir();
  my $execstr = "(cd $wdir; $SHELLFILE \'$XYCEPATH\' $XYCE_VERIFY $XYCE_COMPARE $ckt $GOLDPRNFILE $globaltaglist)";
  system("chmod +x $SHELLFILE");
  #print( "wrapShellScript execstr is $execstr\n" );
  return $self->wrapExec($execstr,"$wdir/$ckt.stdout","$wdir/$ckt.stderr",$self->{childwait});
}
#
## 05/08/08 tscoffe:  Seriously consider using Proc::Background to do the background jobs.
## my $proc = Proc::Background->new($CMD,$arg1,$arg2);
## my $pid = $proc->pid;
## my $exitcode;
## while ($proc->alive) {
##   my $kill;
##   runtime too long { 
##     $kill = 1;
##     $exitcode = 15;
##   }
##   if control-C pressed {
##     $kill = 1;
##     $exitcode = 16;
##   }
##   if (defined $kill) {
##     kill -15, $pid; # TERM -> process group
##     $proc->die; 
##     $proc->wait;
##   }
##   select(undef,undef,undef,0.1); # sleep for a bit
## }
## if (not defined $exitcode) {
##   $exitcode = $proc->wait;
## }
##
sub wrapExec
{
  my ($self,$execstr,$stdout_file,$stderr_file,$alloted_time) = @_;
  my ($pid,$exitcode,$time0,$time1,$dead_pid,$filesize,$filesize_old,$line,$retval);
  my $killjob = undef;
  my $id = $self->getThreadId();
  my $wdir=$self->getWorkDir();
  $pid = $self->forkCommand($execstr,$stdout_file,$stderr_file);
  $retval = -1;
  #  Here's how you check the stats on a file 
  #    ($dev,$ino,$mode,$nlink,$uid,$gid,$rdev,$size,$atime,$mtime,$ctime,$blksize,$blocks) = stat($filename)
  $exitcode = undef;
  $time0 = time;
  while (not defined $exitcode)
  {
    $dead_pid = waitpid($pid,&WNOHANG); # == $pid on completion
    if ($dead_pid == $pid) 
    { 
      $self->debugPrint("pid $pid exited\n");
      $exitcode = 0; 
    }
    $time1 = time;
    my $total_time = $time1 - $time0;
    $self->debugPrint("total time = $total_time\n");
    if ($total_time >= $alloted_time) 
    {
      $self->debugPrint("Killing $pid because total alloted time ($alloted_time) exceeded!\n");
      $exitcode = 15;
      $killjob = 1;
    }
    if ($self->checkControlExit())
    {
      $self->debugPrint("Killing $pid because Control-C pressed.\n");
      $exitcode = 16;
      $killjob = 1;
      $controlC--;
    }
    if ($killjob)
    {
      kill -15, $pid; # TERM -> process group
      $dead_pid = waitpid($pid,0);
    }
    select(undef, undef, undef, 0.01); # this is a non-cpu intensive sleep of 10ms
  }
  # To handle cases where mpich's mpirun lets child processes sit around, lets kill the whole process group.
  kill -15, $pid;
  if ($exitcode != 0) { return $exitcode; }
  return $self->returnExitCode($stdout_file,$stderr_file);
}


sub returnExitCode
{
  my ($self,$stdout_file,$stderr_file) = @_;
  my $exitcode = undef;
  $exitcode = $self->findExitCode($stderr_file);
  if (defined $exitcode) 
  {
    return $exitcode;
  }
  $exitcode = $self->findExitCode($stdout_file);
  if (defined $exitcode)
  {
    return $exitcode;
  }
  # Now we have to deal with no provided exit code.
  return 1;
}

# Find the string:
# Exit code = [0-9]+
# in the output file to determine the exit code.
sub findExitCode
{
  my ($self,$file) = @_;
  my $exitcode = undef;
  open(OUTFILE,"$file");
  while (my $line = <OUTFILE>)
  {
    if ($line =~ s/\s*[Ee]xit\s+[Cc]ode\s*=\s*([0-9]+)\s*$/\1/)
    {
      $exitcode = $line;
    }
  }
  close(OUTFILE);
  return $exitcode;
}

sub forkCommand
{
  my ($self,$execstr,$stdout_file,$stderr_file) = @_;
  my $pid;
  #my $id = $self->getThreadId();
  my $wdir=$self->getWorkDir();
  
  use Errno qw(EAGAIN);
  #$self->debugPrint("$id XyceRegression::Tools::forkCommand \$execstr = >>$execstr<<\n");
  #$self->debugPrint("$id XyceRegression::Tools::forkCommand \$stdout_file = >>$stdout_file<<\n");
  #$self->debugPrint("$id XyceRegression::Tools::forkCommand \$stderr_file = >>$stderr_file<<\n");
  fork:
  {
    if ($pid=fork) 
    {
      #$self->debugPrint("$id XyceRegression::Tools::forkCommand:  This is the parent, returning pid = $pid\n");
      return $pid
    }
    elsif (defined $pid)
    {
      #$self->debugPrint("$id XyceRegression::Tools::forkCommand:  Child $$:  This is the child with pid $$.\n");
      #close(STDOUT);
      #close(STDERR);
      #setpgrp($pid, $self->$id); # This sets process group so I can kill this + children
      setpgrp(0,0); # This sets process group so I can kill this + children
      #$self->debugPrint("$id XyceRegression::Tools::forkCommand:  Child $$:  \$execstr = >>$execstr<<\n");
      #$self->debugPrint("$id XyceRegression::Tools::forkCommand:  Child $$:  Redirecting STDOUT to $stdout_file\n");
      #$self->debugPrint("$id XyceRegression::Tools::forkCommand:  Child $$:  Redirecting STDERR to $stderr_file\n");
      open STDOUT, ">$stdout_file" or die "Child $$:  Can't redirect STDOUT: $!";
      open STDERR, ">$stderr_file" or die "Child $$:  Can't redirect STDERR: $!";
      #chdir $wdir;
      exec "$execstr";
      exit 0; # this is for when exec fails
    }
    elsif ($! == EAGAIN)
    {
      sleep 3;
      redo fork;
    }
    else
    {
      # We could not fork for some reason
      return -1;
    }
  }
}

#
#sub runAndCheckWarning
#{
#  my ($self,$ckt,$XYCEPATH,@searchlist) = @_;
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckWarning:  \$ckt = $ckt\n");
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckWarning:  \$XYCEPATH = $XYCEPATH\n");
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckWarning:  \@searchlist = @searchlist\n");
#  my ($retval,$retval1,$retval2);
#  $retval=$self->wrapXyce($XYCEPATH,$ckt);
#  $self->debugPrint("XyceRegression::Tools::runAndCheckWarning: \$retval = $retval\n");
#  if ($retval != 0) { return 2; } # In this case, Xyce should NOT EXIT WITH ERROR
#
#  print "Checking for strings in $ckt.err\n";
#  $retval1 = $self->checkError("$ckt.err",@searchlist);
#  print "Checking for strings in $ckt.out\n";
#  $retval2 = $self->checkError("$ckt.out",@searchlist);
#  if (($retval1 == 0) or ($retval2 == 0)) { $retval = 0; } else { $retval = 2; }
#  return $retval;
#}
#
#sub runAndCheckGroupedWarning
#{
#  my ($self,$ckt,$XYCEPATH,@searchlist) = @_;
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckGroupedWarning:  \$ckt = $ckt\n");
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckGroupedWarning:  \$XYCEPATH = $XYCEPATH\n");
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckGroupedWarning:  \@searchlist = @searchlist\n");
#  my ($retval,$retval1,$retval2);
#  $retval=$self->wrapXyce($XYCEPATH,$ckt);
#  $self->debugPrint("XyceRegression::Tools::runAndCheckGroupedWarning: \$retval = $retval\n");
#  if ($retval != 0) { return 2; } # In this case, Xyce should NOT EXIT WITH ERROR
#
#  print "Checking for strings in $ckt.err, using checkGroupedWarning()\n";
#  $retval1 = $self->checkGroupedError("$ckt.err",@searchlist);
#  print "Checking for strings in $ckt.out, using checkGroupedWarning()\n";
#  $retval2 = $self->checkGroupedError("$ckt.out",@searchlist);
#  if (($retval1 == 0) or ($retval2 == 0)) { $retval = 0; } else { $retval = 2; }
#  return $retval;
#}
#
#sub runAndCheckError
#{
#  my ($self,$ckt,$XYCEPATH,@searchlist) = @_;
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckError:  \$ckt = $ckt\n");
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckError:  \$XYCEPATH = $XYCEPATH\n");
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckError:  \@searchlist = @searchlist\n");
#  my ($retval,$retval1,$retval2);
#  $retval=$self->wrapXyce($XYCEPATH,$ckt);
#  $self->debugPrint("XyceRegression::Tools::runAndCheckError: \$retval = $retval\n");
#  if ($retval == 0) { return 2; } # In this case, Xyce should EXIT WITH ERROR
#
#  print "Checking for strings in $ckt.err\n";
#  $retval1 = $self->checkError("$ckt.err",@searchlist);
#  print "Checking for strings in $ckt.out\n";
#  $retval2 = $self->checkError("$ckt.out",@searchlist);
#  if (($retval1 == 0) or ($retval2 == 0)) { $retval = 0; } else { $retval = 10; }
#  return $retval;
#}
#
#sub runAndCheckGroupedError
#{
#  my ($self,$ckt,$XYCEPATH,@searchlist) = @_;
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckGroupedError:  \$ckt = $ckt\n");
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckGroupedError:  \$XYCEPATH = $XYCEPATH\n");
#  #$self->debugPrint("XyceRegression::Tools::runAndCheckGroupedError:  \@searchlist = @searchlist\n");
#  my ($retval,$retval1,$retval2);
#  $retval=$self->wrapXyce($XYCEPATH,$ckt);
#  $self->debugPrint("XyceRegression::Tools::runAndCheckGroupedError: \$retval = $retval\n");
#  if ($retval == 0) { return 2; } # In this case, Xyce should EXIT WITH ERROR
#
#  print "Checking for strings in $ckt.err, using checkGroupedError()\n";
#  $retval1 = $self->checkGroupedError("$ckt.err",@searchlist);
#  print "Checking for strings in $ckt.out, using checkGroupedError()\n";
#  $retval2 = $self->checkGroupedError("$ckt.out",@searchlist);
#  if (($retval1 == 0) or ($retval2 == 0)) { $retval = 0; } else { $retval = 10; }
#  return $retval;
#}
#
#sub checkError
#{
#  # this function makes the implicit assumption that the error messages come out in
#  # the same order on all OS's.
#  my ($self,$errfile,@searchlist) = @_;
#  $self->debugPrint("XyceRegression::Tools::checkError:  \$errfile = $errfile\n");
#  $self->debugPrint("XyceRegression::Tools::checkError:  \@searchlist = @searchlist\n"); 
#  if ( not @searchlist ) { return 2; }
#  my ($line);
#  my $exitcode = -1;
#  my $found = 0;
#  if ( not -f $errfile ) 
#  { 
#    $self->verbosePrint("No $errfile file produced!\n");
#    return 2; 
#  } 
#  open(ERR,$errfile);
#  while ($line = <ERR>)
#  {
#    $self->verbosePrint("$line");
#    if ($line =~ "$searchlist[$found]")
#    { 
#      print "Found matching line >>$searchlist[$found]<<\n";
#      $found++; 
#    }
#    if ($found == $#searchlist+1) 
#    { close(ERR); }
#  }
#  close(ERR);
#  if ($found == $#searchlist+1)
#  {
#    print("Test Passed!\n");
#    return 0;
#  }
#  else
#  {
#    my $n;
#    for ($n=$found;$n<=$#searchlist+1;$n++)
#    {
#      print "NOT FOUND:  >>$searchlist[$n]<<\n";
#    }
#    print("Test Failed!\n");
#    return 2;
#  }
#}
#
#sub checkGroupedError
#{
#  # This version does not assume that the error messages come out in the same order
#  # on all OS's.  It does assume that the error messages come in "groups" though, and the
#  # search list must be defined as a 2D array.  An example with a "groupd size" of 2 is:
#  #    @searchstrings = (["Netlist error in file test.cir at or near line 18",
#  #                       "Device instance Bogo: Invalid parameter given."]);
#  #
#  # In general, any size group is allowed.  And there may be different groupd sizes
#  # within the @searchstrings array.
#  #
#  # This function might be useful when testing error messages from device constructors, 
#  # where the test netlist has multiple instance lines with errors on them. 
#  my ($self,$errfile,@searchlist) = @_;
#  $self->debugPrint("XyceRegression::Tools::checkGroupedError:  \$errfile = $errfile\n");
#  $self->debugPrint("XyceRegression::Tools::checkGroupedError:  \@searchlist = @searchlist\n"); 
#  if ( not @searchlist ) { return 2; }
#  my ($line);
#  my (@lines);
#  my (@searchListRow);
#  my $idx = 0; 
#  my $lineIdx = 0; 
#  my $searchIdx = 0;
#  my $exitcode = -1;
#  my $found = 0;
#  my $matchFound=0;
#  my $matchCount=0;
#
#  if ( not -f $errfile ) 
#  { 
#    $self->verbosePrint("No $errfile file produced!\n");
#    return 2; 
#  } 
#
#  # read the info in the errfile into an array for later comparison against
#  # the @searchlist array.
#  open(ERR,$errfile);
#  $idx=0;
#  while ($line = <ERR>)
#  {
#    $lines[$idx]=$line;
#    $idx++;
#  }
#  close(ERR);
#
#  for $searchIdx (0 .. $#searchlist)
#  {
#    # @searchlist is a 2-D array with a variable number of columns in each row.
#    # So, extract the current row
#    @searchListRow = @{$searchlist[$searchIdx]};
#
#    # will be used later to determine if the group of lines in the searchlist was
#    # found in the @lines array
#    $matchFound=0;
#    $matchCount=0;
#
#    # iterate through the @lines array read in from the errfile
#    for $lineIdx (0 .. $#lines)
#    {
#      # ($@searchListRow+1) is the number of consecutive lines that
#      # we are trying to match against consecutive lines in the @lines arrays
#      for my $idx (0 .. $#searchListRow)
#      {
#        if ( $lines[$lineIdx+$idx] =~ "$searchlist[$searchIdx][$idx]" ) 
#        {
#	  $matchCount++;
#        }
#        else
#        {
#          # break out of this loop if no match is found.
#	  last;
#        }   
#      }
#      
#      # break out of the search through the @lines array if we found the
#      # correct number of consecutive matching rows
#      if ($matchCount ==  $#searchListRow +1)
#      {
#        print "Found matching line      >>$searchlist[$searchIdx][0]<<\n";
#        for my $idx (1 .. $#searchListRow)
#        {
#          print "  and expected next line >>$searchlist[$searchIdx][$idx]<<\n";
#        }
#        $matchFound=1; 
#
#        # Now remove the "found set of lines" from the @lines array.  This is
#        # needed because multiple Xyce packages (e.g, Device and IO) may emit
#        # warning messages for the same netlist line with a message content like:
#        #     Netlist warning in file Bogo.cir at or near line 12
#        #       More warning message text ...
#        #
#        # These warning messages (from different Xyce packages) might not be
#        # adjacent in the Xyce output.
#        splice @lines,$lineIdx,$matchCount;
#
#        # skip to the of the @lines array
#        last;
#      }
#    }
#
#    # since the error messages may be re-ordered on various OS's,
#    # don't stop checking if one missing error message is found.
#    if ($matchFound > 0)
#    {
#      $found++;
#    }
#    else
#    {
#      print "No set of matches found for >>$searchlist[$searchIdx][0]<<\n";
#      for my $idx (1 .. $#searchListRow)
#      {
#        print "  and expected next line    >>$searchlist[$searchIdx][$idx]<<\n";
#      }
#    }
#  }
#
#  # declare success if the number of found matches is equal to
#  # the number of lines in the search list.
#  if ($found == $#searchlist+1)
#  {
#    print("Test Passed!\n");
#    return 0;
#  }
#  else
#  {
#    print("Test Failed!\n");
#    return 2;
#  }
#}
#
# This routine takes a list of times from "time" and converts them to 
# "TIME-DATE" format where TIME = "HH:MM::SS AM" and DATE= "Mar 13, 2006"
sub getTimeString
{
  my ($self,@seconds) = @_;
  my (@TIMELIST,%month,$TIME,$DATE,$S,$M,$H,$d,$m,$y,$AMPM,$sec);
  %month = ( 1  =>"Jan", 2  =>"Feb", 3  =>"Mar", 4  =>"Apr", 5  =>"May", 6  =>"Jun", 
             7  =>"Jul", 8  =>"Aug", 9  =>"Sep", 10 =>"Oct", 11 =>"Nov", 12 =>"Dec");
  foreach $sec (@seconds)
  {
    ($S,$M,$H,$d,$m,$y) = (localtime($sec)) [0..5];
    $y %= 100; $y += 2000;
    $m += 1;
    if ($H < 12) { $AMPM = "AM" } else { $AMPM = "PM"; if ($H > 12) { $H -= 12; } }
    # if it's 12 am, then H=0.  change it to 12 so it is what we expect
    if ($H == 0) { $H = 12 } 
    $TIME = sprintf("%2.2d:%2.2d:%2.2d %s",$H,$M,$S,$AMPM);
    $DATE = sprintf("%s %2.2d, %4.4d",$month{$m},$d,$y);
    $self->verbosePrint("    TIME  = $TIME\n");
    $self->verbosePrint("    DATE  = $DATE\n");
    push @TIMELIST, "$TIME-$DATE";
  }
  return @TIMELIST;
}

# tscoffe 07/18/07:  This routine is for testing run_xyce_regression and its tags features
sub checkTags
{
  my ($self,$tags,@args) = @_;
  my ($test_name,$test_home);
  my ($run_xyce_regression,$xyce_regression_home);
  my ($run_xyce_regression_out,$run_xyce_regression_gold);
  my ($retval);
  my ($wc);

  $self->debugPrint("\n\@args = @args\n");
  $self->debugPrint("\$tags = $tags\n");

  my $XYCE=$args[0];
  my $XYCE_VERIFY=$args[1];
  #my $XYCE_COMPARE=$args[2];
  my $CIRFILE=$args[3];
  #my $GOLDPRN=$args[4];

  $test_name = $CIRFILE;
  $test_name =~ s/\.cir//;

  $self->debugPrint("\$test_name = $test_name\n");

  $test_home = `pwd`; chomp($test_home);

  $self->debugPrint("\$test_home = $test_home\n");

  $run_xyce_regression = $XYCE_VERIFY;
  $run_xyce_regression =~ s/xyce_verify\.pl/run_xyce_regression/;

  $self->debugPrint("\$run_xyce_regression = $run_xyce_regression\n");

  $xyce_regression_home = $XYCE_VERIFY;
  $xyce_regression_home =~ s-xyce_verify.pl--;

  $self->debugPrint("\$xyce_regression_home = $xyce_regression_home\n");

  `rm -f ${test_name}_list.out`;
  `rm -f ${test_name}_list.tmp1`;
  `rm -f ${test_name}_list.tmp2`;

  chdir "$xyce_regression_home";

  $run_xyce_regression_out = "$test_home/${test_name}_list.out";
  $run_xyce_regression_gold = "$test_home/${test_name}_list.gs";

  $self->debugPrint("\$run_xyce_regression_out = $run_xyce_regression_out\n");
  $self->debugPrint("\$run_xyce_regression_gold = $run_xyce_regression_gold\n");

  my $run_xyce_regression_args = "--notest $tags --writelist=$run_xyce_regression_out $XYCE";
  $retval = system("$run_xyce_regression $run_xyce_regression_args");
  if ($retval != 0) { return 1; }
  $retval = system("$run_xyce_regression --notest --testlist=$run_xyce_regression_out --excludelist=$run_xyce_regression_gold --writelist=$test_home/${test_name}_list.tmp1 $XYCE");
  $wc = `cat $test_home/${test_name}_list.tmp1 | grep -v "^#" | wc -c`; 
  if ("$wc" != "0") { return 2; }
  $retval = system("$run_xyce_regression --notest --excludelist=$run_xyce_regression_out --testlist=$run_xyce_regression_gold --writelist=$test_home/${test_name}_list.tmp2 $XYCE");
  $wc = `cat $test_home/${test_name}_list.tmp2 | grep -v "^#" | wc -c`; 
  if ("$wc" != "0") { return 2; }
  return 0;
}

# Note:  we'll have to deal with the version text that gets printed out when
# Xyce runs.  E.g.  "This is version DEVELOPMENT-200801231101" which will
# change between builds because it includes the date.
sub fullDiff
{
  my ($self,$goldpath,$newpath,$fulldiffregex) = @_;
  my $status = 0;
  my (@filelist,$name);
  chdir "$goldpath" or die "ERROR, cannot chdir to $goldpath\n";
  @filelist = <*>;
  foreach $name (@filelist) {
    if (-d "$name") {
      $self->verbosePrint("fullDiff:  Skipping directory $name\n");
      next;
    }
    if ($name =~ m/\.clean$/) {
      $self->verbosePrint("fullDiff:  Skipping clean file $name\n");
      next;
    }
    if ($name =~ m/\.raw$/) {
      # 01/24/08 tscoffe:  raw files contain the date
      $self->verbosePrint("fullDiff:  Skipping raw file $name\n");
      next;
    }
    if ($name =~ m/bug_742.cir.prn$/) {
      # 01/24/08 tscoffe:  bug_742.cir.prn is random data each run
      $self->verbosePrint("fullDiff:  Skipping bug_742.cir.prn\n");
      next;
    }
    if (-f "$newpath/$name") {
      $status += $self->cleanDiff("$goldpath/$name","$newpath/$name",$fulldiffregex);
    } else {
      $self->verbosePrint("\nfullDiff:  file:  $name does not exist in path:  $newpath\n");
      #$status += 1;
    }
  }
  if ($status != 0) {
    $status = 2;
  }
  return $status;
}

sub cleanDiff
{
  my ($self,$goldfile,$newfile,$diffstringsfile) = @_;
  my (@searchstrings,$string);
  my ($CMD);
  system("cp $goldfile $goldfile.clean");
  system("cp $newfile $newfile.clean");
  system("perl -pi -n $diffstringsfile $goldfile.clean $newfile.clean");
  return system("diff $goldfile.clean $newfile.clean > $newfile.diff");
}

sub sedLikeFunction
{
  my ($self,$expr,$filein,$fileout) = @_;
  my ($line);
  open(REGEX,"$filein") or die "Error, cannot open input file $filein!\n";
  open(OUT,">$fileout") or die "Error, cannot open output file $fileout!\n";
  while ($line = <REGEX>) {
    $line =~ s/$expr//g;
    print OUT $line;
  }
  close(REGEX);
  close(OUT);
}

sub modifyTest {
  my ($self,$CIRFILE,@optionList) = @_;
  my (%optionHash,$option,@oo);
  my ($tempfile,$basename);
  %optionHash = { };
  foreach $option (@optionList) {
    @oo = split('=',$option);
    $optionHash{$oo[0]} = $oo[1];
  }
  my $notemp;
  $tempfile = `mktemp ${CIRFILE}_tmpXXX` or $notemp=1;
  chomp($tempfile);
  if ($notemp == 1) {
    print "NO Temp file created!\n";
    print "Exit code = 1\n";
    exit 1;
  }
  open(CIR,"$CIRFILE");
  open(CIROUT,">$tempfile");
  while (my $line = <CIR>) {
    if ($line =~ m/^\s*\*/) { 
      print CIROUT $line;
      next; 
    }
    foreach $option (keys %optionHash) {
      my $val = $optionHash{$option};
      $line =~ s/$option\s*=\s*[0-9a-zA-Z\.\+\-\{\}\'\"]+/$option=$val/;
    }
    print CIROUT $line;
  }
  close(CIR);
  close(CIROUT);
  return($tempfile);
}

sub lookupStatus
{
  my ($self,$status) = @_;
  my $message = "";
  if (!($message = $status2MessageHash{$status})) {
    $message = "UNKNOWN ERROR";
  }
  return($message);
}

# This is a function to determine if a particular tag is contained in this list.
# UnitTested 
sub containsTag {
  my ($self,$tag,@taglist) = @_;
  my ($t);
  my $success = undef;
  foreach $t (@taglist) {
    if ("$t" =~ m/^$tag$/) {
      $success = 1;
    }
  }
  return $success;
}

# This function converts "+tagA-tagB?tagC" to three sets of lists containing
# the required tags, the excluded tags, and the optional tags.
# UnitTested
sub parseTagList {
  my ($self,$tagstring) = @_;
  my (@withtags,@withouttags,@withoptionaltags);
  $tagstring =~ s/\s+//g; # Remove all whitespace
  $self->debugPrint("parseTagList:  \$tagstring = >>$tagstring<<\n");

# Note: This doesn't really work correctly:
#  my @taglist = split(/([+-?])/,$tagstring); # split on + - ?
# It incorrectly splits tags that contain numbers.  I (TVR) do not understand
# why, but it has something to do with the special meanings of + and ? in
# regexps.  The line below accomplishes what was *intended* by that line,
# but by using the ASCII hex equivalents of + (\x2b), - (\x2d) and ? (\x3f)
  my @taglist = split(/([\x2b\x2d\x3f])/,$tagstring); # split on + - ?
  if ($taglist[0] == '') {
    shift @taglist;  # the delimiter at the beginning creates a null list entry 
  } else {
    $self->verbosePrint("parseTagList:  ERROR, >>$taglist[0]<< requires an identifier [+-?].\n");
    my (@emptywithtags,@emptywithouttags,@emptywithoptionaltags);
    return(\@emptywithtags,\@emptywithouttags,\@emptywithoptionaltags);
  }
  $self->debugPrint("parseTagList:  \@taglist = @taglist\n");
  foreach my $item (@taglist) {
    $self->debugPrint("parseTagList:  \@taglist item = $item\n");
  }
  while (@taglist) {   
    my $identifier = shift(@taglist);
    my $tag = shift(@taglist);
    $self->debugPrint("parseTagList:  \$identifier = >>$identifier<<\n");
    $self->debugPrint("parseTagList:  \$tag = >>$tag<<\n");
    if ($identifier =~ m/[+]/) {
      push(@withtags,$tag);
    } elsif ($identifier =~ m/[-]/) {
      push(@withouttags,$tag);
    } elsif ($identifier =~ m/[?]/) {
      push(@withoptionaltags,$tag);
    } else {
      $self->verbosePrint("parseTagList:  ERROR, >>$identifier<< is not a valid identifier for taglist;  valid identifiers are [+-?].\n");
      my (@emptywithtags,@emptywithouttags,@emptywithoptionaltags);
      return(\@emptywithtags,\@emptywithouttags,\@emptywithoptionaltags);
    }
  }
  @withtags = $self->uniqueSet(@withtags);
  @withouttags = $self->uniqueSet(@withouttags);
  @withoptionaltags = $self->uniqueSet(@withoptionaltags);
  return (\@withtags,\@withouttags,\@withoptionaltags);
}

# This function removes all duplicate entries from a list
# UnitTested
sub uniqueSet {
  my ($self,@inputList) = @_;
  my %inputHash = ();
  my @outputList = ();
  foreach my $item (@inputList) {
    if (not defined $inputHash{$item}) {
      $inputHash{$item} = 1;
      push(@outputList,$item);
    }
  }
  return @outputList;
}

# This function determines if two lists have the exact same items (independent of sorting)
# UnitTested
sub setEqual {
  my ($self,$listAref,$listBref) = @_;
  my @listA = sort(@$listAref);
  my @listB = sort(@$listBref);
  $self->debugPrint("setEqual:  \@listA = @listA\n");
  $self->debugPrint("setEqual:  \@listB = @listB\n");
  my $retval = 1;
  if ($#listA != $#listB) { return 1; }
  my $N = $#listA+1;
  my $i;
  my $correct = 0;
  for ($i = 0 ; $i<$N ; $i++) {
    my $substrA = $listA[$i];
    my $substrB = $listB[$i];
    $self->debugPrint("setEqual:  \$substrA = $substrA\n");
    $self->debugPrint("setEqual:  \$substrB = $substrB\n");
    if ($substrA eq $substrB) { $correct++; }
  }
  if ($correct == $N) { $retval = 0; }
  $self->debugPrint("setEqual:  \$retval = $retval\n");
  return $retval;
}

# This function simply returns the date in a specific format.
# UnitTested
sub getDate {
  my ($self,$d1,$d2,$d3) = @_;
  my ($S,$M,$H,$d,$m,$y) = (localtime) [0..5];
  $y%=100;
  $m+=1;
  my $DATE = sprintf("%2.2d$d1%2.2d$d1%2.2d$d2%2.2d$d3%2.2d$d3%2.2d",$m,$d,$y,$H,$M,$S);
  return $DATE;
}

# This function parses "options" and "foo.cir.options" files
# UnitTested
sub parseOptionsFile {
  my ($self,$filename) = @_;
  my $wdir= $self->getWorkDir();
  my %options;
  open(OPTIONS,"<$wdir/$filename");
  while (my $line = <OPTIONS>) {
    $line =~ s/[\s]*=[\s]*/=/g;  # Remove spaces around =
    $line =~ s/[\s]+/ /g; # Remove extra spaces
    my @matches = split(/[\s,]+/,$line);
    my $matchstr = join("!",@matches);
    $self->debugPrint("parseOptionsFile:  \@matches = >>$matchstr<<\n");
    foreach my $match (@matches) {
      my @expr = split(/[=]/,$match);
      $options{$expr[0]} = $expr[1];
    }
  }
  close(OPTIONS);
  return \%options;
}

# This function creates a temporary file and returns its name.  It also writes a string to the file.
# UnitTested
sub createTempFile {
  my ($self,$string) = @_;
  my $tmpfile = `mktemp run_xyce_regression_tempfile.XXXXX`;
  $self->debugPrint("createTempFile:  \$string = >>$string<<\n");
  $self->debugPrint("createTempFile:  \$tmpfile = $tmpfile\n");
  chomp($tmpfile);
  open(TMPFILE,">$tmpfile");
  print TMPFILE "$string";
  close(TMPFILE);
  return $tmpfile;
}

# This function parses "tags" and "foo.cir.tags" files
# UnitTested
sub parseTagsFile {
  my ($self,$dir,$filename,$excludenotagsflag) = @_;
  my ($line,$t,@localtags);
  my (%testtags,%requiredtesttags);
  my $skipchecks = undef;
  %testtags = ();
  %requiredtesttags = ();
  if ( -f "$filename" ) {
    open(TAGS,"$filename") or warn "Cannot open tags file >$filename< in directory $dir\n";
    $self->debugPrint("Parsing tag file: $filename\n");
    # This file has the following format:
    # "#" produce comments
    # serial,parallel,nightly,weekly,rad,exclude
    # required:noverbose,required:dakota,required:windows,required:nwcc,required:klu
    # valgrind,olddae,newdae
    while ($line = <TAGS>) {
      chomp($line);
      if ( $line =~ m/^\s*[\#]/ ) { next; } # remove full line comments
      $line =~ s/[\#].*//; # Remove trailing comments
      if ( $line =~ m/^\s*$/ ) { next; } # remove empty lines
      $line =~ s/\s//g; # remove all white-space
      $line =~ tr/[A-Z]/[a-z]/; # change to lower case
      $line =~ s/[,]+/,/g; # Remove extraneous ","s
      $line =~ s/^[,]//; # Remove leading ","
      $self->debugPrint("\$line = $line\n");
      @localtags = split(',',$line);
      foreach $t (@localtags) {
        if ($t =~ m/[:]/) {
          my @t_split = split(':',$t);
          if ($t_split[0] =~ m/required/) {
            $requiredtesttags{"$t_split[1]"} = 1;
            $testtags{"$t_split[1]"} = 1;
          } else {
            my $path=`pwd`; chomp($path);
            die "ERROR:  Unrecognized tag qualifier: >$t_split[0]< for tag >$t_split[1]< in file $path/$filename\n";
          }
        } else {
          $testtags{"$t"} = 1;
        }
      }
    }
  } else {
    $self->debugPrint("No tags file named $filename\n");
    if (not defined $excludenotagsflag) {
      $skipchecks = 1;
    }
  }
  foreach my $key ( keys %testtags ) {
    $self->debugPrint("testtags += $key\n");
  }
  foreach my $key (keys %requiredtesttags ) {
    $self->debugPrint("requiredtesttags += $key\n");
  }
  return (\%testtags,\%requiredtesttags,$skipchecks);
}

# This function converts a list into a hash where each entry in the list is used a key pointing to "1"
# UnitTested
sub listToHash {
  my ($self,@INlist) = @_;
  my %OUThash;
  foreach my $item ( @INlist ) {
    $OUThash{"$item"} = 1;
  }
  return %OUThash;
}

# This function determines which test tags match the user provided tags
# UnitTested
sub checkTestTags {
  my ($self,$dir,$filename,$localtaglist,$excludenotagsflag) = @_;
  my ($testtagsref);
  my ($withtagsmatch,$wt,$failure);
  my ($withouttagsmatch,$wot);
  my ($requiredtesttagsref,$requiredtagsmatch);
  my $skipchecks = undef;
  my ($withtagsref,$withouttagsref,$withoptionaltagsref) = $self->parseTagList($localtaglist);
  my @withtags = @$withtagsref;
  my @withouttags = @$withouttagsref;
  my @withoptionaltags = @$withoptionaltagsref;
  my %withtagshash = $self->listToHash(@withtags);
  my %withoptionaltagshash = $self->listToHash(@withoptionaltags);
  ($testtagsref,$requiredtesttagsref,$skipchecks) = $self->parseTagsFile($dir,"$filename",$excludenotagsflag);
  my @testTags = keys(%$testtagsref);
  $withtagsmatch = 1;
  if ( not defined $skipchecks and @withtags ) {
    $self->debugPrint("Checking [with] flags\n");
    $withtagsmatch = undef;
    $failure = undef;
    foreach $wt (@withtags) {
      if (not defined $testtagsref->{"$wt"}) {
        $failure = 1;
        $self->verbosePrint("with tag: $wt not found in test tags\n");
      }
    }
    if (not defined $failure) {
      $withtagsmatch = 1;
    }
  }
  $withouttagsmatch = 1;
  if ( not defined $skipchecks and @withouttags ) {
    $self->debugPrint("Checking [without] flags\n");
    $withouttagsmatch = undef;
    $failure = undef;
    foreach $wot (@withouttags) {
      if (defined $testtagsref->{"$wot"}) {
        $failure = 1;
        $self->verbosePrint("without tag: $wot found in test tags\n");
      }
    }
    if (not defined $failure) {
      $withouttagsmatch = 1;
    }
  }
  $requiredtagsmatch = 1;
  if ( not defined $skipchecks ) {
    $self->debugPrint("Checking test's required flags\n");
    $requiredtagsmatch = undef;
    $failure = undef;
    foreach my $rt (keys %$requiredtesttagsref) {
      if (not defined $withtagshash{"$rt"} and not defined $withoptionaltagshash{"$rt"}) {
        $failure = 1;
        $self->verbosePrint("required test tag: $rt not found in optional or with tags\n");
      }
    }
    if (not defined $failure) {
      $requiredtagsmatch = 1;
    }
  }
  return ($withtagsmatch,$withouttagsmatch,$requiredtagsmatch,\@testTags);
}

# This function creates a hash from an exclude file
# UnitTested
sub getExcludeHash {
  my ($self,$dir,$excludefile) = @_;
  my %excludehash;
  my ($line);
  if ( -f "$excludefile" ) {
    open(EXCLUDE,"$excludefile") or warn "Cannot open $excludefile in directory $dir\n";
    # Make the parsing of this file a little bit more robust
    while ($line = <EXCLUDE>) {
      chomp($line);
      if ( $line =~ m/^\s*[\#]/ ) { next; } # remove full line comments
      $line =~ s/[\#].*//; # Remove trailing comments
      if ( $line =~ m/^\s*$/ ) { next; } # remove empty lines
      $line =~ s/\s//g; # remove all white-space
      $self->debugPrint("getExcludeHash:  \$line = >>$line<<\n");
      $excludehash{$line} = 1;
    }
    close(EXCLUDE);
  }
  return %excludehash;
}

# This function enables tests based on their tags and the user provided tags.
# UnitTested
sub enableTestByTags {
  my ($self,$dir,$ckt,$excludefile,$dirtagsfile,$cirtagsfile,$localtaglist,$excludenotagsflag) = @_;
# Order of precendence:
# excluded in exclude file
# non-matching with,without,required in "$ckt.tags"
# non-matching with,without,required in "tags" file
  my %excludehash = $self->getExcludeHash($dir,$excludefile);
  my ($testTagsRef1,$testTagsRef2, @testTagsList1, @testTagsList2);
  my ($withtagsmatch,$withouttagsmatch,$requiredtagsmatch);
  ($withtagsmatch,$withouttagsmatch,$requiredtagsmatch,$testTagsRef1) = $self->checkTestTags($dir,$dirtagsfile,$localtaglist,$excludenotagsflag);
  @testTagsList1 = @$testTagsRef1;
  my $success = 1;
  my $done = undef;
  my ($localwithtagsmatch,$localwithouttagsmatch,$localrequiredtagsmatch);
  if (defined $excludehash{$ckt}) { 
    $self->verbosePrint("$ckt removed from list due to exclude file\n"); 
    $success = undef;
    $done = 1;
  }
  if ( (not defined $done) and (-f "$cirtagsfile") ) {
    ($localwithtagsmatch,$localwithouttagsmatch,$localrequiredtagsmatch,$testTagsRef2) = $self->checkTestTags($dir,$cirtagsfile,$localtaglist,$excludenotagsflag);
    @testTagsList2 = @$testTagsRef2;
    if ( not defined $localwithtagsmatch ) {
      $self->verbosePrint("$ckt removed from list due to local non-matching 'with' tags\n");
      $success = undef;
    } elsif ( not defined $localwithouttagsmatch ) {
      $self->verbosePrint("$ckt removed from list due to local non-matching 'without' tags\n");
      $success = undef;
    } elsif ( not defined $localrequiredtagsmatch ) {
      $self->verbosePrint("$ckt removed from list due to local non-matching 'required' tags\n");
      $success = undef;
    }
    # if $ckt.tags exists and the tags match, then add it.
    $done = 1;
  }
  if (not defined $done) {
    if ( not defined $withtagsmatch ) {
      $self->verbosePrint("$ckt removed from list due to non-matching 'with' tags\n");
      $success = undef;
    } elsif ( not defined $withouttagsmatch ) {
      $self->verbosePrint("$ckt removed from list due to non-matching 'without' tags\n");
      $success = undef;
    } elsif ( not defined $requiredtagsmatch ) {
      $self->verbosePrint("$ckt removed from list due to non-matching 'required' tags\n");
      $success = undef;
    }
  }
  push(@testTagsList1,@testTagsList2);
  return ($success,\@testTagsList1);
}

# This function converts a string list of tags to a string list of unique tags
# UnitTested
sub uniqueTags {
  my ($self,$inputTags) = @_;
  my $outputTags;
  $inputTags =~ s/\s+//g;
  $inputTags =~ s/([+-?])/ $1/g;
  my @inputList = split(/ /,$inputTags);
  @inputList = $self->uniqueSet(@inputList);
  $outputTags = join("",@inputList);
  return $outputTags;
}

# This function combines the depreciated tag specification and the new tag specification into one taglist string
# Unit Tested
sub combineTags {
  my ($self,$withtagsref,$withouttagsref,$withoptionaltagsref,$localtaglist) = @_;
  my @withtags = @$withtagsref;
  my @withouttags = @$withouttagsref;
  my @withoptionaltags = @$withoptionaltagsref;
  my $taglisttemp;
  $self->debugPrint("combineTags:  \$#withtags = $#withtags\n");
  $self->debugPrint("combineTags:  \$#withouttags = $#withouttags\n");
  $self->debugPrint("combineTags:  \$#withoptionaltags = $#withoptionaltags\n");
  if ((@withtags) or (@withouttags) or (@withoptionaltags)) {
    $self->quietPrint("Note:  The method of specifying tags has changed.  \n");
    my ($withtaglist,$withouttaglist,$withoptionaltaglist);
    if ($#withtags >= 0) {
      $withtaglist = join('+',@withtags); $withtaglist = "+$withtaglist";
    }
    if ($#withouttags >= 0) {
      $withouttaglist = join('-',@withouttags); $withouttaglist = "-$withouttaglist";
    }
    if ($#withoptionaltags >= 0) {
      $withoptionaltaglist = join('?',@withoptionaltags); $withoptionaltaglist = "?$withoptionaltaglist";
    }
    $taglisttemp = "$withtaglist$withouttaglist$withoptionaltaglist";
    my $oldwithtag = "";
    foreach my $tag (@withtags) {
      $oldwithtag = "$oldwithtag --withtag=$tag";
    }
    my $oldwithouttag = "";
    foreach my $tag (@withouttags) {
      $oldwithouttag = "$oldwithouttag --withouttag=$tag";
    }
    my $oldwithoptionaltag = "";
    foreach my $tag (@withoptionaltags) {
      $oldwithoptionaltag = "$oldwithoptionaltag --withoptionaltag=$tag";
    }
    my $localtaglistprint = "";
    if (defined $localtaglist) {
      $localtaglistprint = "--taglist=\"$localtaglist\"";
    }
    # Add any $localtaglist options to taglisttemp and uniqueify them
    $taglisttemp = "$taglisttemp$localtaglist";
    $taglisttemp = $self->uniqueTags($taglisttemp);
    $self->quietPrint("The current specification:\n");
    $self->quietPrint("$oldwithtag $oldwithouttag $oldwithoptionaltag $localtaglistprint\n");
    $self->quietPrint("can now be specified:\n");
    $self->quietPrint("--taglist=\"$taglisttemp\"\n");
  } else {
    $taglisttemp = $localtaglist;
  }
  return $taglisttemp;
}

# This function sets up default tags if none are specified
# UnitTested
sub setDefaultTags {
  my ($self,$localtaglist) = @_;
# Set up default tags if none are specified.
# This is primarily for when developers use run_xyce_regression directly 
  if ($localtaglist eq "") {
    $localtaglist = "+serial+nightly-exclude";
    $self->verbosePrint("Default tags are nightly, serial.\n");
  } 
# The exclude tag is slightly special in that we don't want the developers to
# _have_ to specify --withouttag=exclude if they specify any tags.  To
# facilitate this, if you don't _ask_ for exclude tags with --withtag=exclude
# or --withoptionaltag=exclude, then it will be added by default to
# withouttags.
  my ($localwithtagsref,$localwithouttagsref,$localwithoptionaltagsref) = $self->parseTagList($localtaglist);
  my @localwithtags = @$localwithtagsref;
  my @localwithouttags = @$localwithouttagsref;
  my @localwithoptionaltags = @$localwithoptionaltagsref;
  if (not ($self->containsTag("exclude",@localwithtags) or $self->containsTag("exclude",@localwithoptionaltags) or $self->containsTag("exclude",@localwithouttags))) {
    $self->verbosePrint("Adding -exclude tag since exclude was not specified\n");
    $localtaglist = "$localtaglist-exclude";
  }
  $self->debugPrint("\$localtaglist = $localtaglist\n");
  return($localtaglist);
}

# This function sets run options from a hash
# UnitTested
sub setRunOptions {
  my ($self,$optionsHashRef) = @_;
  my %newOptions = %$optionsHashRef;
  if ( defined $newOptions{'timelimit'} ) {
    $self->debugPrint("Setting timelimit to $newOptions{'timelimit'}\n");
    $self->setChildWait($newOptions{'timelimit'});
  }
  if ( defined $newOptions{'progresslimit'} ) {
    $self->debugPrint("Setting progresslimit to $newOptions{'progresslimit'}\n");
    $self->setChildProgress($newOptions{'progresslimit'});
  }
}

# This function reads options files and sets them on the Tools object.
# UnitTested
sub readAndSetRunOptions {
  my ($self,$dirOptions,$cktOptions) = @_;
  my $dirOptionsRef = $self->parseOptionsFile($dirOptions);
  my $cktOptionsRef = $self->parseOptionsFile($cktOptions);
  my $optionsHashRef = $self->mergeHash($dirOptionsRef,$cktOptionsRef);
  $self->setRunOptions($optionsHashRef);
}

# This function merges two hashes into one.
# UnitTested
sub mergeHash {
  my ($self,$dirRef,$cktRef) = @_;
  my %out = %$dirRef;
  my %ckt = %$cktRef;
  foreach my $op (keys %ckt) {
    $out{$op} = $ckt{$op};
  }
  return \%out;
}

# This function determines if two hashes are identical
# UnitTested
sub hashEqual {
  my ($self,$hash1Ref,$hash2Ref) = @_;
  my $result = 1;
  my %hash1 = %$hash1Ref;
  my %hash2 = %$hash2Ref;
  my @hash1Keys = keys(%hash1);
  my @hash2Keys = keys(%hash2);
  my $correct = 0;
  my $totalNumber = $#hash1Keys+2;
  if ( $self->setEqual(\@hash1Keys,\@hash2Keys) eq 0 ) {
    $correct++;
  }
  foreach my $hash1key (@hash1Keys) {
    if ($hash1{$hash1key} eq $hash2{$hash1key} ) {
      $correct++;
    }
  }
  if ($correct eq $totalNumber) { $result = 0; }
  return $result;
}

# This function gets run options from Tools
# UnitTested
sub getRunOptions {
  my ($self) = @_;
  my %optionsHash;
  $optionsHash{'timelimit'} = $self->getChildWait();
  $optionsHash{'progresslimit'} = $self->getChildProgress();
  return \%optionsHash;
}

# This function figures out where run_xyce_regression is.
# UnitTested
sub configureXyceTestHome {
  my ($self,$xyce_test,$ENVref) = @_;
  my $XYCE_TEST_HOME;
  my %ENV = %$ENVref;
# Order of precedence:
# 1.  command line option
# 2.  Environment variable
# 3.  current directory
  if (defined($xyce_test)) {
    $XYCE_TEST_HOME=$xyce_test;
  } elsif (defined ($ENV{XYCE_TEST_HOME})) {
    $XYCE_TEST_HOME=$ENV{XYCE_TEST_HOME};
  } else {
# tscoffe 01/19/06:  we should check that we are in Xyce_Test tree here.
    $XYCE_TEST_HOME=`pwd`;
    chomp($XYCE_TEST_HOME);
    if ($XYCE_TEST_HOME =~ s-/TestScripts$--) {
      $self->verbosePrint("Using current directory to find Xyce_Test\n");
    } else {
      $self->iPrint("ERROR:  XYCE_TEST_HOME environment variable not set and this\n");
      $self->iPrint("script is not being run from the TestScripts subdirectory of\n");
      $self->iPrint("a Xyce_Test checkout\n");
      $XYCE_TEST_HOME = undef;
    }
  }
  $self->verbosePrint("XYCE_TEST_HOME set to $XYCE_TEST_HOME\n");
  return $XYCE_TEST_HOME;
}

# This function copies files from source to dest, it only copies files that appear in the CVS/Entries file
sub setupCVSSandbox {
  use File::Copy 'cp';
  # First argument = source directory to copy from
  # Second argument = destination directory to copy to
  my ($self,$sourcedir,$destinationdir,$ckt) = @_;
  my ($line,@files,$file);
  $self->debugPrint("setupCVSSandbox:  \$sourcedir = $sourcedir\n");
  $self->debugPrint("setupCVSSandbox:  \$destinationdir = $destinationdir\n");
  if ($destinationdir eq $sourcedir) {
    $self->debugPrint("setupCVSSandbox:  \$destinationdir == \$sourcedir, no copying.\n");
    $self->debugPrint("setupCVSSandbox:  Setting execute bit on .sh and .pl files.\n");
    chdir "$sourcedir";
    `chmod +x *.sh *.pl 2>/dev/null`;
    return;
  } 
  if (not -d "$destinationdir") {
    $self->debugPrint("setupCVSSandbox:  Creating directory:  $destinationdir\n");
    `mkdir -p $destinationdir`;
  }
  
    
  # need to do two things here:
  # (1) if there wasn't a manifest file. then make it an populate it with what was
  # read from ENTRIES.
  # (2) if there is both a manifest and CVS/Entries then verify they are the same.
  # (3) if there was a manifest file than use that rather than the CVS Entries.
  if (not open(MFILE,"$sourcedir/Manifest.txt")) {
    $self->debugPrint("setupCVSSandbox:  Warning, cannot open file $sourcedir/Manifest.txt\n");
    $self->debugPrint("\nTypically this means that a new directory has been created for a new test but\nthe files required for the test have not been listed in the Manifest.txt file.  The run_xyce_regression script\ndetermines which files to copy to a sandbox area by looking in the Manifest.txt\nfile.  This allows it to separate previous run's output from the source test\nfiles.\nPlease add the Manifest.txt file and retry.\n\n");
  }
  else {
    # read from the Manifest file
    while ($line = <MFILE>) {
      chomp $line;
      my @split_line = split(' ',$line);
      if ($split_line[0] =~ m/^D$/i) { next; } # Skip directories
      if ($split_line[0] =~ m/^\s*$/) { next; } # Skip empty strings
      $self->debugPrint("setupCVSSandbox:  Adding file \"$split_line[0]\" to sandbox list from Manifest.txt\n");
      @files[$#files+1] = $split_line[0];
    }
    close(MFILE);
    # Here, I'd like to make sure $ckt is in the list of files to copy.  If it
	# is not, then there is another CVS issue to resolve.  11/19/08 tscoffe
	my $found = 1;
	foreach my $f (@files) {
	  if ("$f" =~ m/^$ckt$/) {
		$found = 1;
	  }
	}
	if ($found == 0) {
	  $self->iPrint("setupCVSSandbox:  Warning, cannot find file \"$ckt\" in Manifest.txt file\n");
	  $self->iPrint("\nTypically this means a new test was added to an existing directory but this\ntest has not been added to the Manifest.txt file.  The run_xyce_regression script determines\nwhich files to copy to a sandbox area by looking in the Manifest.txt file.  This\nallows it to separate previous run's output from the source test files.\nPlease add the file $ckt to the Manifest.txt file and retry.\n\n");
	}
  }

  if( not -f "$sourcedir/Manifest.txt" ) {
    # didn't find a Manifest so fall back on the CVS file if its there
    if (not open(ENTRIES,"$sourcedir/CVS/Entries")) {
      $self->debugPrint("setupCVSSandbox:  Warning, cannot open file $sourcedir/CVS/Entries\n");
      $self->debugPrint("\nTypically this means that a new directory has been created for a new test but\nthat directory has not been added to CVS yet.  The run_xyce_regression script\ndetermines which files to copy to a sandbox area by looking in the CVS/Entries\nfile.  This allows it to separate previous run's output from the source test\nfiles.\nPlease add the directory $sourcedir to cvs and retry.\n\n");
    }
    while ($line = <ENTRIES>) {
      my @split_line = split('/',$line);
      if ($split_line[0] =~ m/^D$/i) { next; } # Skip directories
      if ($split_line[1] =~ m/^\s*$/) { next; } # Skip empty strings
      $self->debugPrint("setupCVSSandbox:  Adding file \"$split_line[1]\" to sandbox list\n");
      @files[$#files+1] = $split_line[1];
    }
    close(ENTRIES);
 
	# Here, I'd like to make sure $ckt is in the list of files to copy.  If it
	# is not, then there is another CVS issue to resolve.  11/19/08 tscoffe
	my $found = 0;
	foreach my $f (@files) {
	  if ("$f" =~ m/^$ckt$/) {
		$found = 1;
	  }
	}
	if ($found == 0) {
	  $self->iPrint("setupCVSSandbox:  Warning, cannot find file \"$ckt\" in CVS/Entries file\n");
	  $self->iPrint("\nTypically this means a new test was added to an existing directory but this\ntest has not been added to CVS yet.  The run_xyce_regression script determines\nwhich files to copy to a sandbox area by looking in the CVS/Entries file.  This\nallows it to separate previous run's output from the source test files.\nPlease add the file $ckt to cvs and retry.\n\n");
	}
  }
  
  # make a manifest file 
  if( not -f "$sourcedir/Manifest.txt" )
  {
    $self->iPrint("setupSVSSandbox: No Manifest.txt file. Trying to make one.\n");
    # no Manifest file so make one 
    if (not open( MFILE, ">$sourcedir/Manifest.txt" ) ) {
      $self->iPrint("setupSVSSandbox: Warning couldn't make a manifest file\n");
    }
    else {
      foreach $file (@files) {
        print MFILE "$file\n" ;
      }
      close(MFILE);
    }
    
  }
  
  foreach $file (@files) {
    my $slashidx = rindex($file,'/');
    if ($slashidx > 0)
    {
      my $subdirpath = substr($file,0,$slashidx);
      if (!-d "$destinationdir/$subdirpath")
      {
        $self->debugPrint("setupCVSSandbox:  Creating directory:  $destinationdir/$subdirpath\n");
        `mkdir -p $destinationdir/$subdirpath`;
      }
    }

    $self->debugPrint("setupCVSSandbox:  Copying file \"$sourcedir/$file\" to $destinationdir/$file\n");
    cp("$sourcedir/$file","$destinationdir/$file");
    $self->debugPrint("setupCVSSandbox:  Setting execute bit on .sh and .pl files.\n");
    if (($file =~ m/\.sh$/) or ($file =~ m/\.pl$/)) { `chmod +x $destinationdir/$file`; }
  }
}

# This function converts an exclude list into a hash with the added bonus of converting spaces to "/".
# UnitTested
sub excludeListToHash {
  my ($self,@excludelist) = @_;
  my %excludehash;
  foreach my $exc (@excludelist) {
    $exc =~ s- -/-; # change space back to "/" for this.
    $excludehash{$exc} = 1;
    $self->debugPrint("\$excludehash{\"$exc\"} = 1\n");
  }
  return %excludehash;
}

# This subroutine determines how to truncate a filename to fit inside 38 characters.
# UnitTested
# If we can print the whole dir/name, then do so.
# Shorten Certification_Tests -> C_T,
#         SandiaTests -> ST, and 
#         try to get the whole ckt name in.
sub figureCirName {
  my ($self,$dir,$ckt,$colwidth) = @_;
  my ($printCirName,$dotlength,$dots,$len);
  $ckt =~ s/\.cir//; # remove ".cir" at end
  my $cktlen=length($ckt);
  my $dirlen=length($dir);
  #my $colwidth = 41;
  my $mindots = 3;
  my $supershortdir = $dir;
  $supershortdir =~ s/Certification_Tests/C_T/;
  $supershortdir =~ s/SandiaTests/ST/;
  my $supershortdirlen = length($supershortdir);
  my $shortdir = $dir;
  $shortdir =~ s/Certification_Tests/C_T/;
  my $shortdirlen = length($shortdir);
  if ($dirlen+1+$cktlen <= $colwidth-$mindots) {
    # This is best case scenario, print everything:
    # We need an extra character for the "/"
    $dotlength=$colwidth-$dirlen-1-$cktlen;
    $dots="." x $dotlength;
    $printCirName = "$dir/$ckt$dots";
  } elsif ($shortdirlen+1+$cktlen <= $colwidth-$mindots) {
    # Use the shortened dir name and the whole circuit name
    $dotlength=$colwidth-$shortdirlen-1-$cktlen;
    $dots="." x $dotlength;
    $printCirName = "$shortdir/$ckt$dots";
  } elsif ($supershortdirlen+1+$cktlen <= $colwidth-$mindots) {
    # Use the super shortened dir name and the whole circuit name
    $dotlength=$colwidth-$supershortdirlen-1-$cktlen;
    $dots="." x $dotlength;
    $printCirName = "$supershortdir/$ckt$dots";
  } elsif ($supershortdirlen+1+2 <= $colwidth-$mindots) {
    # This prints the first few characters of the $ckt name:
    # We need 1 char for "/" and 1 char for "+" and at least 1 char of name
    $len = $colwidth-$mindots-$supershortdirlen-1-1;
    $self->debugPrint("len = $len\n");
    $ckt = substr($ckt,0,$len);
    $self->debugPrint("ckt shortened to >>$ckt<<\n");
    $cktlen = length($ckt);
    $dotlength=$colwidth-$supershortdirlen-1-$cktlen-1;
    $dots="." x $dotlength;
    $printCirName = "$supershortdir/$ckt+$dots";
  } elsif ($dirlen <= $colwidth-$mindots) {
    # This just print the full directory name:
    $dotlength=$colwidth-$dirlen;
    $dots="." x $dotlength;
    $printCirName = "$dir$dots";
  } elsif ($shortdirlen <= $colwidth-$mindots) {
    # This just print the short directory name:
    $dotlength=$colwidth-$shortdirlen;
    $dots="." x $dotlength;
    $printCirName = "$shortdir$dots";
  } elsif ($supershortdirlen <= $colwidth-$mindots) {
    # This just print the super short directory name:
    $dotlength=$colwidth-$supershortdirlen;
    $dots="." x $dotlength;
    $printCirName = "$supershortdir$dots";
  } else {
    # This prints the first few characters of the $supershortdir name:
    $len = $colwidth-$mindots-1;
    $supershortdir = substr($supershortdir,0,$len);
    $supershortdirlen = length($supershortdir);
    $dotlength = $colwidth-$supershortdirlen-1;
    $dots = "." x $dotlength;
    $printCirName = "$supershortdir+$dots";
  }
  $self->debugPrint("printCirname = >>$printCirName<<\n");
  return $printCirName;
}

# This function reads in the testlist file and creates a testlist
# UnitTested
sub parseTestList {
  my ($self,$testlist,$ignoreparsewarnings) = @_;
  my ($parseError, $linenum, $line, @split_line, @cktlist);
  $self->debugPrint("Attempting to read file: $testlist\n");
  open(TESTLIST, "$testlist") or warn "Cannot open file:  $testlist\n";
  # Make the parsing of this file a little bit more robust
  $parseError = 0;
  $linenum = 0;
  while ($line = <TESTLIST>) {
    $linenum++;
    chomp($line);
    if ( $line =~ m/^\s*[\#]/ ) { next; } # remove full line comments
    $line =~ s/[\#].*//; # Remove trailing comments
    if ( $line =~ m/^\s*$/ ) { next; } # remove empty lines
    $line =~ s/^\s+//; # Remove space at beginning of line
    $line =~ s/\s+$//; # Remove space at end of line
    $line =~ s-^\./+--; # remove "./" at beginning of line
    #$line =~ s-/- -g; # Change / to spaces.
    $line =~ s-[/]+-/-g; # Change multiple / to a single /
    $line =~ s/\s+/\//g; # Change spaces to /.
    $line =~ s/[\/]([^\/]+[.]cir)/ $1/; # Change last / to a space.
    $self->debugPrint("\$line = $line\n");
    @split_line = split(/\s+/,$line);
    if ( $#split_line == 1 ) {
      $line = "$split_line[0] $split_line[1]";
      push(@cktlist,$line);
      $self->debugPrint("Adding $line to list\n");
    } else {
      $self->iPrint("WARNING:  parsing error in file $testlist on line $linenum.\n");
      $self->iPrint("This line should only have a directory and a .cir name on it.\n");
      print results "WARNING:  parsing error in file $testlist on line $linenum.\n";
      print results "This line should only have a directory and a .cir name on it.\n";
      $parseError = 1;
      next;
    }
  }
  close(TESTLIST);
  if ($parseError == 1 and not defined $ignoreparsewarnings) { exit -1 }
  return @cktlist
}

# Check Xyce version string for a given text string
# if found return 1 else return -1
sub checkXyceVersionStringForText {
  my ($self, $XyceBinary, $targetText) = @_;
  my $retValue=-1;
  my $version=`$XyceBinary -v`;
  $retValue=1 if ($version =~ /$targetText/);
  return $retValue;
}

1;
