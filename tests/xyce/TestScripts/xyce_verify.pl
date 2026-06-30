#!/usr/bin/env perl
# -*-perl-*-
# The thing after shebang up there is replaced by the path to perl when
# configure is run.
###########################################################################
#
# File: xyce_verify
#
# Purpose: given a netlist, a "good" output file and a "test" output file,
# check whether the test file agrees with the good one.
#
# Usage: xyce_verify [options] netlist goodfile testfile [plotfile]
#
# Recognized options: --debug and --verbose
#
# Author: Tom Russo, SNL/NM, Electrical and Microsystems Modeling
#         Todd Coffey, SNL/NM, Computational Math/Algorithms
#
# Date: $Date$
# Revision: $Revision$
# Owner: $Author$
###########################################################################
#NOTES:
#The relative error formula is:
#  integral(  (gooddata-testdata)/(reltol*(gooddata)+abstol) ) dx
# where the integral is over "x," the independent variable
#
# An "rms relative error"   is calculated by:
#  sqrt(integral ( ((gooddata-testdata)/(reltol*gooddata+abstol))^2)dx/deltax)
# where deltax is the difference between high and low values of the
# independent variable
#
# All cases where abs(gooddata-testdata) is less than absdifftol are
# counted as "zero" relative error for the purpose of the integration
#
#Integration is done by simplest summation of areas under the
#piecewise linear approximation of the function
#
# "Acceptable" RMS relative error will be less than 1.  That should
# translate to the error being within the percentage specified by
# reltop.

use Carp;
use FindBin;
use Getopt::Long;

use lib "$FindBin::Bin";

use XyceVerify::DCSweep;
use XyceVerify::StepSweep;
use XyceVerify::DCSources;

sub verbosePrint
{
  print @_ if ($verbose);
}

sub debugPrint
{
  print @_ if ($debug);
}

#Read any options
&GetOptions( "goodres=s" => \$goodres,
             "testres=s" => \$testres,
             "printline=s" => \$printlineoverride,
             "verbose!" => \$verbose,
             "debug!" => \$debug,
             "help!" => \$help ) or $help=1;


if ( defined $help )
{
  print "$0 version 0.3\n";
  print "Usage: $0 [options] netlist goodfile testfile [plotfile]\n";
  print "options:\n";
  print "--verbose\n";
  print "--debug\n";
  print "--printline=print-line-type\n";
  print "--goodres=filename\n";
  print "--testres=filename\n";
  print "--help\n";
  exit 0;
}

if ($#ARGV < 2)
{
  print STDERR "Usage: $0 [options] netlist goodfile testfile [plotfile]\n";
  exit 1;
}

($netlist, $goodfile, $testfile,$plotfile)=@ARGV;

debugPrint "Hello! n=$netlist g=$goodfile t=$testfile p=$plotfile\n";

if ($plotfile ne "")
{
  $do_plot=1;
  debugPrint "Supposed to do plotting, plotfile is $plotfile\n";
}
else
{
  debugPrint "No plotting, plotfile is $plotfile\n";
  $do_plot=0;
}

# parseNetlist does something I doing with any language --- sets all
# manner of "package variables" instead of returning any values
# directly.  It's perl, it's allowed to be yucky, and I'm letting it
# be.

# In the case of .STEP netlists, a ".res" file is output.  In this first pass,
# we're going to REQUIRE that the good and test res files be specifically
# named.  I will have to monkey with the command line a lot to get this
# appropriately flexible.
$goodres="$netlist".".res.gs" if (!defined($goodres));   #"Gold Standard" res file
$testres="$netlist".".res" if (!defined($testres));

open(NETLIST,$netlist) || croak "Could not open netlist file $netlist";
parseNetlist();

# now figure out which print line to use if there are more than one, or
# if the user told us which one to use.
my %printline;
$print_line="";
if ($printlineoverride)
{
  $print_line=$printline{lc($printlineoverride)};
  debugPrint "Using .print $printlineoverride line for purpose of expectations, by user request\n";
  debugPrint "$print_line\n";
}
elsif ($hb)
{
  # We have an issue here.  Might need the HB_IC line, might want the HB_TD
  # line.  For now, let's pick HB_IC if it exists, HB_TD if that doesn't exist
  # and HB if neither
  #
  # And if a netlist has BOTH, then the user has to add "--printline=" to
  # tell us which one to use.
  #
  $print_line=$printline{"hb_ic"};
  if ($print_line eq "")
  {
    $print_line=$printline{"hb_td"};
  }
  if ($print_line eq "")
  {
    $print_line=$printline{"hb"};
  }
  debugPrint "Using .print hb line for purpose of expectations\n";
  debugPrint "$print_line\n";
}
elsif ($transient)
{
  $print_line=$printline{"tran"};
  debugPrint "Using .print tran line for purpose of expectations\n";
  debugPrint "$print_line\n";
}
elsif ($dcsweep)
{
  $print_line=$printline{"dc"};
  debugPrint "Using .print DC line for purpose of expectations\n";
  debugPrint "$print_line\n";
}
elsif ($ac && $op)
{
  $print_line=$printline{"ac"};
  debugPrint "Using .print AC line for purpose of expectations\n";
}
else
{
  print "Sorry, cannot deduce what print line to use!\n";
  exit 1;
}

if ($theDCSources)
{
  verbosePrint "There are ".$theDCSources->getNumSources().
      " voltage source(s)\n" if (defined $theDCSources);
  $theDCSources->dumpOut() if ($verbose);
}

# if the user told us to use a .print sens line, we have to figure out what
# the expectations must be.  We must therefore also parse the .sens line
# and any .options sensitivity line.

my @sensvars;
my @senstypes;
if ($sens && (lc($printlineoverride) eq "sens"))
{
  debugPrint "Parsing sensitivity because override is set.\n";
  $objfunc=parseSensitivity($sens_line,$sens_options_line,\@sensvars,\@senstypes);
  debugPrint "Parsing sensitivity returned objfunc=$objfunc.\n";
  debugPrint "Parsing sensitivity returned sensvars 0..$#sensvars\n";
  debugPrint "Parsing sensitivity returned senstypes 0..$#senstypes\n";
}

$returncode=parsePrintLine($print_line,\$expectlineref);
DataFileErrorHandle($returncode,$netlist);


close(NETLIST);

#print "We expect to find at least ";
#print $#$expectlineref +1;
#print " elements on print line\n";
#print "those elements should be:\n";
#for ($foobie=0;$foobie<=$#$expectlineref;$foobie++)
#{
#    print "   $$expectlineref[$foobie] ";
#}
#print "\n";

#parseNetlist should have set up an array full of the "*COMP" lines, now
# let's process them --- we defer it to now, because we need to do it as
# a second pass, otherwise we'd have to require those lines at the end of the
# netlist, and I don't want to make that requirement.
$returncode=parseCompArray($expectlineref,\@comparray);
DataFileErrorHandle($returncode,$netlist);

if ($transient)
{
  dumpTRANS();
}

if ($dcsweep)
{
  prepDC();
}

if ($stepsweep)
{
  # form the various arrays controlling the step operation
  prepStep();

  # make sure result file has the right sweep values in it.
  $returncode=checkResFile($goodres);
  DataFileErrorHandle($returncode,$goodres);

  $returncode=checkResFile($testres);
  DataFileErrorHandle($returncode,$testres);

  # if we get this far, we must now split good and test files into
  # individual files for each step and then just loop over the
  # load/comparison This will return an array of good and test filenames
  # to check
  $returncode=splitInput($goodfile, \@goodfilenamearray);
  DataFileErrorHandle($returncode,$goodfile);
  $returncode=splitInput($testfile, \@testfilenamearray);
  $returncode=DataFileErrorHandle($returncode,$testfile);

  # gotta handle plotfiles --- how best? For now, just make one plot file
  # for each step.  Bleah.
  $plotfilebase=$plotfile;
}
else
{
  $goodfilenamearray[0]=$goodfile;
  $testfilenamearray[0]=$testfile;
}

# if a step sweep, this goes in a loop, clearing the data references each
# time through
for (my $j = 0; $j <= $#goodfilenamearray; $j++)
{
  $goodfile=$goodfilenamearray[$j];
  $testfile=$testfilenamearray[$j];


  $plotfile="$plotfilebase".".$j" if ($stepsweep);
  debugPrint "doing sweep step $j of $#goodfilenamearray\n";

  debugPrint "Plotting in $plotfile.\n";

  open(GOODFILEH,$goodfile) || croak "Could not open good file $goodfile";
  $returncode=ReadDataFile(GOODFILEH,\$gooddatalinearrayref,$expectlineref);
  DataFileErrorHandle($returncode,$goodfile);
  close (GOODFILEH);
  debugPrint "Good data has been read\n";

  open(TESTFILEH,$testfile) || croak "Could not open netlist file $testfile";
  $returncode=ReadDataFile(TESTFILEH,\$testdatalinearrayref,$expectlineref);
  DataFileErrorHandle($returncode,$testfile);
  close (TESTFILEH);
  debugPrint "Test data has been read\n";


  open(PLOTFH,"> $plotfile") if ($do_plot == 1);

  debugPrint "Comparing data files\n";
  $returncode=CompareDataFiles($testdatalinearrayref,$gooddatalinearrayref,$expectlineref);
  DataFileErrorHandle($returncode,$netlist);

  close (PLOTFH) if ($do_plot);

  # Undefine the references, so GC can clear the arrays.
  $testdatalinearrayref=undef;
  $gooddatalinearrayref=undef;
}

if ($stepsweep)
{
  #clean up droppings
  for ($i = 0 ; $i <= $#goodfilenamearray; $i++)
  {
    unlink $goodfilenamearray[$i] || croak "Could not unlink $goodfilenamearray[$i]";
  }

  for ($i = 0 ; $i <= $#testfilenamearray; $i++)
  {
    unlink $testfilenamearray[$i] || croak "Could not unlink $testfilenamearray[$i]";
  }
}

# normal exit of main code
exit 0;

sub parseNetlist
{
  my ($i,$line,@split_line,$vsrc,$start,$stop,$step);
  $savedline="";

  $op=0;
  $ac=0;
  $hb=0;
  $sens=0;
  $transient=0;
  $dcsweep=0;
  $stepsweep=0;
  $n_step_vars=0;
  $theDCSources=undef;
  $theDCSweep=undef;

  while ($line=lc(getNetlistLine()))
  {
    # Make all the "FOO = BAR" stuff into "FOO=BAR" so we don't have to do
    # so much work...
    $line =~ s/(\w+)\s*=\s*(\w+)/$1=$2/g;
    #...when we split up the line by cutting where the spaces are
    @split_line=split(' ',$line);
    $opcode=$split_line[0];
    shift(@split_line);
    if ($opcode eq "*comp")
    {
      #	   print "Special compare directive: $line\n";
      $comparray[$#comparray+1]=[@split_line];
    }
    elsif ($opcode eq ".op")
    {
      $op=1;
    }
    elsif ($opcode eq ".ac")
    {
      $ac=1;
    }
    elsif ($opcode eq ".tran")
    {
      $transient=1;
      ($tstep, $tstop, $tstart, $tmax) = @split_line;
      $tstep=modVal2Float($tstep);
      $tstop=modVal2Float($tstop);
      $tstart=modVal2Float($tstart);
      $tmax=modVal2Float($tmax);
    }
    elsif ($opcode eq ".mpde")
    {
      $transient=1;
      ($tstep, $tstop, $tstart, $tmax) = @split_line;
      $tstep=modVal2Float($tstep);
      $tstop=modVal2Float($tstop);
      $tstart=modVal2Float($tstart);
      $tmax=modVal2Float($tmax);
    }
    elsif ($opcode eq ".hb")
    {
      $transient=1;
      $hb=1;
      ($hbfreq) = @split_line;
      $hbfreq=modVal2Float($hbfreq);
    }
    elsif ($opcode eq ".sens")
    {
      $sens=1;
      $sens_line=$line;  #save for processing later
    }
    elsif ($opcode eq ".dc")
    {
      $dcsweep=1;
      $theDCSweep=XyceVerify::DCSweep->new($line);
      $theDCSweep->dumpOut() if ($verbose);
      $n_sweep_vars=$theDCSweep->nSweepVars();
      @sweep_vars=$theDCSweep->sweepVar();
      %sweep_name_to_num=$theDCSweep->getSweepNameToNum();
    }
    elsif ($opcode eq ".step")
    {
      $stepsweep=1;
      if (defined $theSteps)
      {
        $theSteps->addStepLine($line);
      }
      else
      {
        $theSteps=XyceVerify::StepSweep->new($line);
      }
    }
    elsif ($opcode eq ".print")
    {
      $printline{$split_line[0]}=$line;
      debugPrint "Setting printline{$split_line[0]} to $line\n";
    }
    elsif ($opcode =~ /^[vip]/)
    {
      $name=$opcode;
      if (defined $theDCSources)
      {
        $theDCSources->addSource($line);
      }
      else
      {
        $theDCSources=XyceVerify::DCSources->new($line);
      }
    }
    elsif ($opcode eq ".options" && $split_line[0] eq "sensitivity")
    {
      $sens_options_line=$line;
    }
  }
  if (defined $theDCSweep)
  {
    #need to finalize the print signs for the sweep:
    $theDCSweep->setPrint($theDCSources);
  }
  if (defined $theSteps)
  {
    my $stepvar;
    $theSteps->dumpOut() if ($verbose);
    $n_step_vars=$theSteps->nSweepVars();
    @step_vars=$theSteps->sweepVar();
    foreach $stepvar (@step_vars)
    {
      # default tolerances
      $zerotol{$stepvar}=1.0e-12;
      $absdifftol{$stepvar}=1.0e-12;
      $abstol{$stepvar}=1.0e-12;
      $reltol{$stepvar}=.01;
      $acceptnumfail{$stepvar}=0;
    }

  }

}

# Replace all commas not inside matched parentheses with spaces
sub undelimit
{
  my ($string,$delimiter)=@_;
  my $i;
  my $paren_level=0;

  if ($delimiter eq " ")
  {
    $string =~ s/ +/ /g;
  }

  for ($i=0;$i<length($string);$i++)
  {

    $char=substr($string,$i,1);
    if ($char eq $delimiter && $paren_level==0)
    {
      substr($string,$i,1)="\t";

    }

    if ($char eq "(")
    {
      $paren_level++;
    }
    if ($char eq ")")
    {
      $paren_level--;
    }
  }
  return $string;
}

sub parsePrintLine
{
  my ($print_line) = @_;
  my ($loopvar);
  my $doing_sensitivity=0;
  # Make all the "FOO = BAR" into "FOO=BAR" deals
  $print_line =~ s/(\w+)\s*=\s*(\w+)/$1=$2/g;
  chomp($print_line);

  $print_line=undelimit($print_line," ");


  my (@splitline)=split("\t",$print_line);
  my ($returnarrayrefref)=$_[1];

  # get rid of ".print"
  shift(@splitline);

  # next had better be "DC", "TRAN", "AC", or "HB" (which is being treated like tran for now)
  # return -94 if (
  #     !(
  #       (lc($splitline[0]) eq "ac" && $ac && $op) ||
  #       (lc($splitline[0]) eq "dc" && $dcsweep) ||
  #       (lc($splitline[0]) eq "tran" && $transient) ||
  #       (lc($splitline[0]) eq "hb" && $transient)
  #       )
  #     );

  $print_width=17;
  $print_precision=8;

  # Handle case where we are a .sens netlist *AND* are looking at the
  # .print sens print line
  if ($sens && ($splitline[0] eq "sens"))
  {
    $doing_sensitivity=1;
    
    # toss the "sens"
    shift(@splitline);
    
    # throw away FOO=BAR options:
    while ($splitline[0] =~ /(\w+)=(\w+)/)
    {
      shift(@splitline);
    }

    # all that remains should be expected.
    $$returnarrayrefref=["index"];

    # if this is a transient run, also expect "time"
    if ($transient)
    {
      push @$$returnarrayrefref,"time";
    }
  }
  elsif ($transient || ($ac && $op))
  {
    $$returnarrayrefref=["index","time"];
    $printvarhash{'time'}=1;
    #throw away "tran"
    shift(@splitline);

    # throw away FOO=BAR options:
    while ($splitline[0] =~ /(\w+)=(\w+)/)
    {
      if ($1 eq "precision")
      {
        $print_precision=$2;
      }
      if ($1 eq "width")
      {
        $print_width=$2;
      }
      if ($1 eq "file")
      {
        $print_file=$2;
      }
      shift(@splitline);
    }

  }
  elsif ($dcsweep)
  {
    my @expectPrint=$theDCSweep->expectPrint();
    # Need to find sweep variables in print columns
    # For now require that fastest sweep var be first column, second var second
    # column, bitch if not

    # throw away "DC"
    shift (@splitline);

    # throw away FOO=BAR options:
    while ($splitline[0] =~ /(\w+)=(\w+)/)
    {
      if ($1 eq "precision")
      {
        $print_precision=$2;
      }
      if ($1 eq "width")
      {
        $print_width=$2;
      }
      if ($1 eq "file")
      {
        $print_file=$2;
      }
      shift(@splitline);
    }

    debugPrint "Header mismatch: ".lc($splitline[0])." ne ".$expectPrint[0]."\n" if  ( lc($splitline[0]) ne $expectPrint[0]);
    return -95 if ( lc($splitline[0]) ne $expectPrint[0]);

    if ($n_sweep_vars > 1)
    {
      shift (@splitline);
      debugPrint "Header mismatch: ".lc($splitline[0])." ne ".$expectPrint[1]."\n" if  ( lc($splitline[0]) ne $expectPrint[1]);
      return -95 if ( lc($splitline[0]) ne $expectPrint[1]);
      $$returnarrayrefref=["index",@expectPrint];
      $printvarhash{$expectPrint[0]}=1;
      $printvarhash{$expectPrint[1]}=1;
      shift(@splitline);
    }
    else
    {
      $$returnarrayrefref=["index",@expectPrint];
      shift (@splitline);
      $printvarhash{$expectPrint[0]}=1;
    }
  }
  # now the mandatory stuff from the output line is in the "expected output"
  # and the $splitline[0] contains the next one we haven't assumed.
  #  Now let's add the rest of them:
  for ($loopvar=0; $loopvar <= $#splitline; $loopvar++)
  {
    # If a user specifies V(a,b) with any spaces in it, e.g. V(A, B), the code 
    # outputs a parsed, reconstructed version V(A,B), not the verbatim
    # input.  Patch up any v(a,b) with spaces to pull out the spaces.
    $splitline[$loopvar] =~ s/([vV])\(\s*([^,\)\s]*)\s*,\s*([^,\)\s]*)\s*\)/$1($2,$3)/g;

    push @$$returnarrayrefref,lc($splitline[$loopvar]);    
    $printvarhash{$$$returnarrayrefref[$#$$returnarrayrefref]}=1;
  }

  # that was enough for DC and transient, but for sensitivity we have a bunch
  # more
  if ($doing_sensitivity)
  {
    debugPrint "parsePrintLine: Doing sensitivity\n";
    
    @objfuns=split(",",$objfunc);

    foreach $ofunc (@objfuns)
    {
      # right after the last thing on the .print sens line comes the objective
      # function
      push @$$returnarrayrefref,$ofunc;

      # Then come the sensitivities.  Order is as given in the .sens param=
      # list, and within that, first _Dir, _Dir_Adj, then _Adj, then _Adj_scaled.

      foreach $sensVariable (@sensvars)
      {
        debugPrint "parsePrintLine with sensVariable=$sensVariable\n";
        foreach $sensType (@senstypes)
        {
          push @$$returnarrayrefref,"d_$ofunc/d_${sensVariable}_$sensType";
          debugPrint "Pushing d_$ofunc/d_${sensVariable}_$sensType into expect\n";
        }
      }
    }
    
  }
  # Lastly, make a format string out of width and precision
  $truncformat="%".$print_width.".".$print_precision."e";
}

sub dumpTRANS
{
  verbosePrint "We are a transient run.\n";
  verbosePrint "    Time integrator parameters are:\n";
  verbosePrint "       Initial step: $tstep\n";
  verbosePrint "       Stop time:    $tstop\n";
  verbosePrint "       Start time:   $tstart\n" if ($tstart ne "") ;
  verbosePrint "       TMAX:   $tmax\n" if ($tmax ne "") ;
}

sub prepDC
{
  my $i;
  verbosePrint "We are a DC Sweep run with $n_sweep_vars sweep variables.\n";

  if ($n_sweep_vars > 1)
  {
    verbosePrint "There are multiple sweep variables, so we have more than one curve to fit\n";
    if ($n_sweep_vars > 2)
    {
      print STDERR "ASSERT FAILED --- more than two sweep variables\n";
      exit 1;
    }
    $sweep_vars[0]=$theDCSweep->sweepVar(0);
    $sweep_vars[1]=$theDCSweep->sweepVar(1);
    verbosePrint "$sweep_vars[0] should be the faster varying sweep variable.\n";
    verbosePrint "$sweep_vars[1] should be the slower varying sweep variable.\n"
        ;

    $n_sweep_steps[0]=$theDCSweep->nSweepSteps(0);
    $n_sweep_steps[1]=$theDCSweep->nSweepSteps(1);

    verbosePrint "in total there should be ";
    verbosePrint $n_sweep_steps[1]*$n_sweep_steps[0];
    verbosePrint " output points\n";
  }
  else
  {
    $sweep_vars[0]=$theDCSweep->sweepVar(0);
    verbosePrint "There is one sweep variable, so we have one curve to fit\n";
    verbosePrint "$sweep_vars[0] is the sweep variable.\n";
    $n_sweep_steps[0]=$theDCSweep->nSweepSteps(0);
    verbosePrint "There should be $n_sweep_steps[0] values of this variable\n";
  }
}

sub prepStep
{
  my $i;

  verbosePrint "We are a STEP Sweep run with $n_step_vars sweep variables.\n";
  if ($n_step_vars > 1)
  {
    verbosePrint "There are multiple step variables, so we have more than one curve to fit\n";
    if ($n_step_vars > 2)
    {
      print STDERR "ASSERT FAILED --- more than two step variables\n";
      exit 1;
    }
    $step_vars[0]=$theSteps->sweepVar(0);
    $step_vars[1]=$theSteps->sweepVar(1);
    verbosePrint "$step_vars[0] should be the faster varying step sweep variable.\n";
    verbosePrint "$step_vars[1] should be the faster varying step sweep variable.\n"
        ;

    $n_step_steps[0]=$theSteps->nSweepSteps(0);
    $n_step_steps[1]=$theSteps->nSweepSteps(1);

    verbosePrint "in total there should be ";
    verbosePrint $n_step_steps[1]*$n_step_steps[0];
    verbosePrint " output points\n";
    $n_total_step_steps=$n_step_steps[1]*$n_step_steps[0];

  }
  else
  {
    $step_vars[0]=$theSteps->sweepVar(0);
    verbosePrint "There is one step sweep variable, so we have one curve to fit\n";
    verbosePrint "$step_vars[0] is the sweep variable.\n";
    $n_step_steps[0]=$theSteps->nSweepSteps(0);
    verbosePrint "There should be $n_step_steps[0] values of this variable\n";
    $n_total_step_steps=$n_step_steps[0];

  }
}

sub getNetlistLine
{
  my ($line);
  # pull in any saved line
  if ($savedline ne "")
  {
    $line=$savedline;
  }
  else
  {
    $line=<NETLIST>;
    $line =~ s/\r//;
  }
  if ($line ne "")
  {
    while ($nextline=<NETLIST>)
    {
      chomp ($nextline);
      $nextline =~ s/\r//;
      @linearray=split(' ',$nextline);
      #	   skip over comments, but not directives disguized as comments
      if (($linearray[0] =~ /^\*/) && !($linearray[0] =~/^\*COMP/i))
      {
        next;
      }
      #           stop reading if not a continuation or comment
      if (index($nextline,"+")!=0)
      {
        $savedline=$nextline;
        last;
      }
      # otherwise it's a continuation, so tack it on
      if ( $linearray[0] =~ /^\+/)
      {
        chomp ($line);
        $line=$line." ".substr($nextline,1);
      }
    }
    $savedline="" if ($nextline eq "");
  }
  return $line;
}

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


sub ReadDataFile
{
  my ($infile,$filetype,$datastart)=($_[0],"unknown",0);
  my ($fileline,@splitline);
  my (@dataline,@datalinearray);
  my ($returnarrayrefref)=$_[1];
  my ($expectdataref)=$_[2];
  my ($temploop);
  # first we have to skip over any headers, and decide if this is
  # a spice or Xyce output file, or just some data we stuffed in
  # to a file by some other means ("unknown")

  #skip over blank lines
  while ($fileline=<$infile>)
  {
    $fileline=~ s/\r//g;
    chomp($fileline);
    last if ($fileline ne "");
  }

  # We now need to figure out if this is a Spice 3F5 netlist, or a Xyce
  # netlist.  If a Xyce netlist, we need to know the column delimiter.
  # We need to know the delimiter, because if it's comma, we've got our work
  # cut out for us

  $fileline =~ /(Index|Circuit)(.)/;
  if ($1 eq "Index")
  {
    $filetype="xyce";
    $file_delimiter=$2;
  }
  elsif ($1 eq "Circuit")
  {
    $filetype="spice";
    $file_delimiter=" ";  # we don't really use this
    debugPrint "File type is SPICE\n";
  }

  if ($filetype ne "xyce" && $filetype ne "spice")
  {
    printf STDERR ("WARNING!  First word read is neither 'Index' nor 'Circuit', xyce_verify cannot figure out what code produced this file.\n");
    printf STDERR ("          Unable to do header checking.\n");
    printf STDERR ("          Make sure you are not using format=NOINDEX in Xyce.\n");
    printf STDERR ("          xyce_verify is unable to process NOINDEX output.\n");
  }

  if ($filetype eq "xyce" && $file_delimiter ne "\t")
  {
    # Grumble.  Need to split up the banner line on commas that aren't between
    # parentheses, which will take a mighty tricky regexp.  Burn that
    # bridge later, for now just use a cheesy character-at-a-time hack
    $fileline=undelimit($fileline,$file_delimiter);
    @splitline=split("\t",$fileline);
  }
  else
  {
    $fileline =~ s/[\t]/ /g;
    @splitline=split(' ',$fileline);
  }

  return -99 if ($#splitline == -1);

  if ($filetype eq "spice")
  {
    # spice has a lot of verbiage at the top that needs to be skipped over
    while ($splitline[0] ne "Index" && !eof($infile))
    {
      $fileline=<$infile>;
      $fileline=~s/[\t]/ /g;
      @splitline=split(' ',$fileline);
    }
    return -98 if ($splitline[0] ne "Index" || eof($infile));

  }

  # If we're here and either a Xyce or SPICE file, we have a line with
  # "Index" starting it, i.e. the column headers
  # Let's check that it begins with what we expect

  if ($filetype eq "spice" || $filetype eq "xyce")
  {
    for ( $temploop=0; $temploop <=$#$expectdataref; $temploop++)
    {
      # spice puts in this extra one, ignore it:
      shift(@splitline) if (lc($splitline[0]) eq "voltage_sweep" || lc($splitline[0]) eq "v-sweep");
      $splitline[0] =~ s/(\w+)_branch/i($1)/ if ($filetype eq "spice");
      $splitline[0] =~ s/(\w+)#branch/i($1)/ if ($filetype eq "spice");

      debugPrint "Expect: $$expectdataref[$temploop], Got: ".lc($splitline[0])."\n";
      return -93 if (lc($splitline[0]) ne $$expectdataref[$temploop]);
      shift (@splitline);
    }
  }

  # when we get here the "fileline" is the one with Index in it.  If
  # it's a spice file, That's still not far enough, there's a line of
  # dashes, so get it in just so we can ignore it.

  if ( $filetype eq "spice")
  {
    $fileline=<$infile>;
  }

  # if this is an unknown type, it MUST NOT HAVE ANY HEADER DATA!  Only
  # Xyce and spice headers will be groked
  # "unknown" files must only have data, no header

  return -97 if (eof($infile));
  if ( $filetype eq "xyce" || $filetype eq "spice")
  {
    # we haven't gotten to the real data yet if we just finished reading
    # a Xyce or Spice file --- the next line is real data, so get it in
    # and split it
    $fileline=<$infile>;
    $fileline =~ s/[\t,]/ /g;
    @splitline=split(' ',$fileline);
  }

  # when we get here, splitline contains the first line of data from the
  # file.  Start saving it.

  @dataline=@splitline;
  # this shifts out the "Index" column
  shift (@dataline);

  # this shifts out the "Sweep Variable" deal
  shift (@dataline) if ($dcsweep && $filetype eq "spice");

  # Try to detect bad numbers (NaN/NAN/nan) in data file.  Puke if find one.
  foreach $datum (@dataline)
  {
    return -40 if ($datum =~ /nan/mi);
  }

  @dataline=replaceZeros($expectdataref,@dataline);

  #store a reference to an anonymous array containing the elements of dataline
  @datalinearray[0]=[@dataline];

  # OK!  Preliminary stuff is done, we can start looping
  $normend=0;
  while ($fileline=<$infile>)
  {

    #check for the dealie that says we're at the end of the run, and stop if so:
    $normend=1, last if ($filetype eq "xyce" && $fileline =~ /End of Xyce/);
    $normend=1, last if ($filetype eq "spice" && $fileline =~ /Total CPU/);

    #otherwise treat it like a line of data

    $fileline =~ s/[\t,]/ /g;
    @dataline=split(' ',$fileline);

    #throw away any line that gets here and doesn't begin with a digit (i.e.
    # SPICE's "^L" and mid-data verbosity)
    next if ($dataline[0] =~ /^\D/ || $dataline[0]==chr(12));

    #throw away the index
    shift(@dataline);

    # and any "voltage_sweep" column that SPICE stuffed in
    shift (@dataline) if ($dcsweep && $filetype eq "spice");

    foreach $datum (@dataline)
    {
      return -40 if ($datum =~ /nan/mi);
    }
    @dataline=replaceZeros($expectdataref,@dataline);

    # Now, this is an *INCREDIBLE* kludge, but sometimes a simulator
    # will take such tiny time steps that the time column doesn't have
    # enough digits to show the difference.  If this is the case, transient
    # compares break (SLATEC requires strictly monotonic independent variables).
    # What we're going to do is throw away any time step that has exactly the
    # same time as the previous value.

    if ($transient && $dataline[0] == $datalinearray[$#datalinearray][0])
    {
      printf STDERR "WARNING!  Throwing away line with duplicate time value $dataline[0]!\n";
    }
    else
    {
      @datalinearray[$#datalinearray+1]=[@dataline];
    }
  }
  $$returnarrayrefref=[@datalinearray];
  return -1 if ($normend == 0 && ($filetype eq "xyce" || $filetype eq "spice"));
  return 0;

}

sub DataFileErrorHandle
{
  my ($errorcode,$filename)=($_[0],$_[1]);
  return if ($errorcode == 0);

  if ($stepsweep && $errorcode < 0)
  {
    #clean up droppings
    for ($i = 0 ; $i <= $#goodfilenamearray; $i++)
    {
      unlink $goodfilenamearray[$i] || croak "Could not unlink $goodfilenamearray[$i]";
    }

    for ($i = 0 ; $i <= $#testfilenamearray; $i++)
    {
      unlink $testfilenamearray[$i] || croak "Could not unlink $testfilenamearray[$i]";
    }
  }

  if ($errorcode == -99)
  { print STDERR "$filename: End of file encountered before any non-blank lines!\n"; exit -$errorcode;}
  elsif ($errorcode == -98)
  { print STDERR "$filename: End of file encountered within Spice header!\n"; exit -$errorcode;}
  elsif ($errorcode == -97)
  { print STDERR "$filename: End of file encountered before any data!\n"; exit -$errorcode;}
  elsif ($errorcode == -95)
  { print STDERR "$filename: Print line for DC SWEEP does not contain sweep variable(s) as first element(s) in print line!\n"; exit -$errorcode}
  elsif ($errorcode == -94)
  { print STDERR "$filename: Print line type does not agree with analysis type!\n"; exit -$errorcode}
  elsif ($errorcode == -93)
  { print STDERR "$filename: Datafile header indicates different output columns than those specified in netlist!\n"; exit -$errorcode}
  elsif ($errorcode == -92)
  { print STDERR "$filename: *COMP line contains variable not specified in print line!\n"; exit -$errorcode}
  elsif ($errorcode == -90)
  { print STDERR "$filename: Cannot determine analysis type while trying to compare!\n"; exit -$errorcode}
  elsif ($errorcode == -89)
  { print STDERR "$filename: test file does not have all the steps that good file does!\n"; exit -$errorcode}
  elsif ($errorcode == -88)
  { print STDERR "$filename: Unrecognized tolerance in COMP line!\n"; exit -$errorcode}
  elsif ($errorcode == -87)
  { print STDERR "$filename: test file has more steps than good file does!\n"; exit -$errorcode}
  elsif ($errorcode == -86)
  { print STDERR "$filename: Datafiles have number of sweep steps inconsistent with analysis line in netlist!\n"; exit -$errorcode}
  elsif ($errorcode == -85)
  { print STDERR "$filename: Sweep values in \"good\" output do not correspond to those asked for in netlist!\n"; exit -$errorcode}
  elsif ($errorcode == -79)
  { print STDERR "$0: Assert failure, ts_interp did not produce number of interpolated points equal to number of test points!\n"; exit -$errorcode}
  elsif ($errorcode == -60)
  { print STDERR "$filename: Error: Results file header does not have STEP as first element.\n"; exit -$errorcode}
  elsif ($errorcode == -61)
  { print STDERR "$filename: Error: Results file header does not have fast sweep variable as second element.\n"; exit -$errorcode}
  elsif ($errorcode == -62)
  { print STDERR "$filename: Error: Results file header does not have slow sweep variable as third element.\n"; exit -$errorcode}
  elsif ($errorcode == -63)
  { print STDERR "$filename: Error: Results file does not match number of steps requested by netlist..\n"; exit -$errorcode}
  elsif ($errorcode == -64)
  { print STDERR "$filename: Error: Results file line first column does not match line number.\n"; exit -$errorcode}
  elsif ($errorcode == -65)
  { print STDERR "$filename: Error: Step parameter value in results file line  not match value expected.\n"; exit -$errorcode}
  elsif ($errorcode == -40)
  { print STDERR "$filename: Data file contains not-a-number fields.\n"; exit -$errorcode}
  elsif ($errorcode == -30)
  { print STDERR "$filename: Failed compare --- missing data in test file.\n"; exit -$errorcode}
  elsif ($errorcode == -20)
  { print STDERR "$filename: Failed compare.\n"; exit -$errorcode}
  elsif ($errorcode == -4)
  { print STDERR "$0: nonzero exit code from ts_interp.\n"; exit -$errorcode}
  elsif ($errorcode == -3)
  { print STDERR "$0: Cannot find ts_interp program.\n"; exit -$errorcode}
  elsif ($errorcode == -1)
  { print STDERR "$filename: File not terminated by normal termination string.\n";exit -$errorcode}

}

sub parseCompArray
{
  my ($loopindex,$loopindex2);
  my ($expectdataref)=$_[0];
  my ($comparrayref)=$_[1];
  my ($varname);

  # this routine will create five hashes,
  # $zerotol should contain the value below which a value will be called zero

  # $absdifftol is the absolute amount by which two values may differ and
  # be called "the same"

  # $abstol is used to prevent swamping of relative error in small values

  # $reltol is used to calculate relative errors

  # $offset is used to shift both test file and gold standard.  This helps 
  #  deal with zero crossings better.

  # The hash key will be the name of the variable in Xyce notation
  # (e.g. "v(node)" or "i(voltage_source)" or "time" not "voltage_source_branch"
  # a la SPICE.  We take care of that in the parsing of the input data file.

  # first set up the hard-coded defaults, in case nobody put "*COMP" lines
  # into the netlist
  # Make these somewhat unreasonable, to encourage the user to think about
  # what the criteria should be.

  # The first "value" is always the integer index, so require it absolutely
  # identical, and don't fill it in in the loop:
  $zerotol{'index'}=0;
  $abstol{'index'}=0;
  $reltol{'index'}=0;
  $offset{'index'}=0;
  $acceptnumfail{'index'}=0;

  # loop over all the other variables in the print line and set the defaults:

  for ($loopindex=1;$loopindex<=$#$expectlineref;$loopindex++)
  {
    $zerotol{$$expectdataref[$loopindex]}=1.0e-12;
    $absdifftol{$$expectdataref[$loopindex]}=1.0e-12;
    $abstol{$$expectdataref[$loopindex]}=1.0e-12;
    $reltol{$$expectdataref[$loopindex]}=.01;
    $offset{$$expectdataref[$loopindex]}=0.0;
    $acceptnumfail{$$expectdataref[$loopindex]}=0;
  }

  # Now loop over all the "*comp" lines
  # A *comp line looks like:
  # *comp variable [KEYWORD=VALUE]*
  # where keyword is "ABSDIFFTOL," "ABSTOL," "ZEROTOL", "OFFSET" or "RELTOL"

  for ($loopindex=0; $loopindex <= $#$comparrayref; $loopindex++)
  {
    $varname=$$comparrayref[$loopindex][0];
    # there is no need for this test --- it used to cause a fatal error if a
    # *COMP line referenced a variable that wasn't printed.  But there's no reason
    # for that --- these things are stored in a hash and only pulled out
    # if they're needed.  If we don't need them, there's no harm done in
    # having them around
    #    return -92 if ( $printvarhash{$varname} != 1 && $stepvarhash{$varname} != 1);

    for ($loopindex2=1;$loopindex2 <= $#{$$comparrayref[$loopindex]};$loopindex2++)
    {
      #	   print " $$comparrayref[$loopindex][$loopindex2]";
    SWITCH: for($$comparrayref[$loopindex][$loopindex2])
    {
      /zerotol=([0-9.eE+-]*)/ && do {$zerotol{$varname}=modVal2Float($1); debugPrint "zerotol{$varname} gets $zerotol{$varname}\n"; last;};
      /absdifftol=([0-9.eE+-]*)/ && do {$absdifftol{$varname}=modVal2Float($1); last;};
      /abstol=([0-9.eE+-]*)/ && do {$abstol{$varname}=modVal2Float($1); last;};
      /reltol=([0-9.eE+-]*)/ && do {$reltol{$varname}=modVal2Float($1); last;};
      /offset=([0-9.eE+-]*)/ && do {$offset{$varname}=modVal2Float($1); last;};
      /numfail=([0-9]*)/ && do {$acceptnumfail{$varname}=$1; last;};
      return -88;
    }

    }
  }

}

# This function shifts input data by the "offset" specified on a COMP line
# (if any) and if the result is smaller than zerotol for that column, 
# replaces it with exactly zero.
sub replaceZeros
{
  my ($expectdataref)=$_[0];
  shift;
  my (@data)=@_;
  my ($position,$ztol);

  for ($position=0; $position <= $#data; $position++)
  {
    $ztol=$zerotol{$$expectdataref[$position+1]};
    $ofst=$offset{$$expectdataref[$position+1]};
    $data[$position] += $ofst;
    debugPrint "Replacing $data[$position] with 0, data column is $$expectdataref[$position+1]\n" if (abs($data[$position]) <= $ztol);
    $data[$position]=0 if (abs($data[$position]) <= $ztol);
  }
  return @data;
}

sub CompareDataFiles
{
  my ($testdataref,$gooddataref,$expectlineref)=($_[0],$_[1],$_[2]);
  my ($lineno,$colno);
  my ($colname,$abstol,$reltol,$absdifftol,$acceptnumfail);
  my (@integrand);
  my ($errorsum,$rmserror);
  my ($returnval);
  my ($numfails,$nummissing);

  $returnval=0;

  print STDERR "I have no idea what to do!\n", return -90 if (!$dcsweep && !$transient && !($ac && $op));

  if ($transient)
  {

    my @interparray=interpolateTimes($testdataref,$gooddataref);

    my @rtol,@atol,@adtol;
    $rtol[0]=1;
    $atol[0]=1;
    $adtol[0]=1;
    for (my $loopvar=1;$loopvar<=$#{$expectlineref}-1;$loopvar++)
    {
      $colname=$$expectlineref[$loopvar+1];
      $rtol[$loopvar]=$reltol{$colname};
      $atol[$loopvar]=$abstol{$colname};
      $adtol[$loopvar]=$absdifftol{$colname};
    }

    my @error_new=errNorm($testdataref,\@interparray,\@rtol,\@atol,\@adtol);

    # Now do a trace-by-trace compare of the interpolated good and test data.
    for ($colno=1; $colno<=$#$expectlineref -1 ; $colno++)
    {
      $colname=$$expectlineref[$colno+1];
      $rmserror = $rtol[$colno]*($error_new[$colno])*100;
      verbosePrint "RMS relative error in $colname is $rmserror%\n";
      if ($rmserror > $rtol[$colno]*100)
      {
        my $reltol=$rtol[$colno]*100;
        $returnval=-20;
        printf STDERR ("Column %s failed compare, tolerance is %.4g%%, integrated error is %.4g%%.\n",$colname,$reltol,$rmserror);
      }
    }

    if ($do_plot)
    {
      #     output for plotting
      printf PLOTFH "%17s\t","Time";
      for ($colno=2;$colno <= $#$expectlineref;$colno++)
      {
        printf PLOTFH "%17s\t%17s\t%17s\t%17s\t","$$expectlineref[$colno](test)","$$expectlineref[$colno](interpolated_good)","Difference","Relative_Error(%)";
      }
      print PLOTFH "\n";
      for ($lineno=0;$lineno<=$#interparray;$lineno++)
      {
        printf PLOTFH "%17.12g\t",$interparray[$lineno][0];
        for ($colno=1;$colno <= $#$expectlineref-1;$colno++)
        {

          printf PLOTFH "%17.12g\t%17.12g\t%17.12g\t%17.12g\t",
          $$testdataref[$lineno][$colno],
          $interparray[$lineno][$colno],
          $interparray[$lineno][$colno]-
              $$testdataref[$lineno][$colno],
              100*$rtol[$colno]*((abs($interparray[$lineno][$colno]-
                                      $$testdataref[$lineno][$colno])
                                  <$adtol[$colno])
                                 ? 0 : (($interparray[$lineno][$colno]-
                                         $$testdataref[$lineno][$colno])
                                        /($rtol[$colno]*abs($interparray[$lineno][$colno])
                                          +$atol[$colno])));
        }
        printf PLOTFH "\n";
      }
    }
  }

  if ($dcsweep)
  {
    if ($n_sweep_vars == 1)
    {

      $numfails=0;
      # We are a 1-variable DC sweep.  There should be the same number of
      # sweep steps in the good and test files, so error out if the test data
      # file has a different number elements than the good data file, or the
      # good data file doesn't have as many steps as the netlist said it should.

      debugPrint "testfile has $#$testdataref, good has $#$gooddataref\n" if  ($#$testdataref < $#$gooddataref);
      return -89 if  ($#$testdataref < $#$gooddataref);
      return -87 if  ($#$testdataref > $#$gooddataref);
      return -86 if  ($n_sweep_steps[0] != $#$gooddataref+1);

      # so we have the same number of sweep steps.  Now compare the two files
      # step by step

      #First stage is to check that the independent variable is actually
      #running through the full range requested in the DC line
      $theDCSweep->initializeSweep();
      for ($lineno=0;$lineno <= $#$goodataref; $lineno++)
      {
        ($v_sweep)=$theDCSweep->getNextSweepStep();
        $v_sweep=sprintf($truncformat,$v_sweep);

        debugPrint " row $lineno: value of $$expectlineref[1] should be $v_sweep, is $$gooddataref[$lineno][0]\n" if
                     (abs($v_sweep-$$gooddataref[$lineno][0]) >
                      $absdifftol{$$expectlineref[1]});
        return -85 if (abs($v_sweep-$$gooddataref[$lineno][0]) >
                       $absdifftol{$$expectlineref[1]});

      }

      #OK, we know the sweep vars are correct in the good file, now
      # we go ahead and compare the test and good files column by column,
      # using the tolerances specified, including the sweep variable

      # we don't need to worry about the zerotol, because we took care of that
      # when we read in the file

      # we use "expectline[col+1]" because expectline also includes "index"
      # as the 0 column.
      my @rtol,@atol,@adtol;
      $rtol[0]=1;
      $atol[0]=1;
      $adtol[0]=1;
      for (my $loopvar=1;$loopvar<=$#{$expectlineref}-1;$loopvar++)
      {
        $colname=$$expectlineref[$loopvar+1];
        $rtol[$loopvar]=$reltol{$colname};
        $atol[$loopvar]=$abstol{$colname};
        $adtol[$loopvar]=$absdifftol{$colname};
      }

      my @error_new=errNorm($testdataref,$gooddataref,\@rtol,\@atol,\@adtol);

      for ($colno=0; $colno <= $#$expectlineref - 1; $colno++)
      {
        $colname=$$expectlineref[$colno+1];
        my $reltol=$rtol[$colno];
        $rmserror = $reltol*$error_new[$colno]*100;
        verbosePrint "RMS relative error in $colname is $rmserror%\n";

        if ($rmserror > $reltol*100)
        {
          $returnval=-20;
          $reltol*=100;
          printf STDERR ("Column %s failed compare, tolerance is %5.3f\%, integrated error is %5.3f\%.\n",$colname,$reltol,$rmserror);
        }
      }
      if ($numfails>0)
      {
        print STDERR "There were $numfails values exceeding the specified relative tolerances.\n";
      }
    }
    else
    {
      # we are in a 2-variable DC Sweep, and need to do curve-by-curve comparisons
      # where each curve is for a single value of the slow variable, with the
      # fast variable as the abscissa
      # The slow variable name is in $sweep_vars[1], the fast is in $sweep_vars[0]
      # and the numbers of steps were placed in @n_sweep_steps by prepDC()
      my ($faststart,$slowstart)=($theDCSweep->sweepStart(0),
                                  $theDCSweep->sweepStart(1));
      my ($faststop,$slowstop)=($theDCSweep->sweepStop(0),
                                $theDCSweep->sweepStop(1));
      my ($nfaststep,$nslowstep)=($n_sweep_steps[0],$n_sweep_steps[1]);
      my ($fastval, $slowval);
      my ($fastindex,$index);
      my ($curgoodline,$curtestline);
      my ($absdifftolfast,$absdifftolslow);
      my ($colnamefast, $colnameslow);
      my ($colindex,$rowindex);
      my ($found);

      my (@integrands);

      $curgoodline=0; $curtestline=0;
      $numfails=0;
      $nummissing=0;

      # Now loop over the values, checking only whether every expected line is there.
      $theDCSweep->initializeSweep();
      $fastindex=0;
      for ($index=0; $index < $nslowstep*$nfaststep; $index++)
      {
        $colnameslow=$$expectlineref[2];
        $absdifftolslow = $absdifftol{$colnameslow};
        ($fastval,$slowval)=$theDCSweep->getNextSweepStep();
        # To guard against failed compares, must truncate to correct precision
        $slowval=sprintf($truncformat,$slowval);
        $fastval=sprintf($truncformat,$fastval);
        verbosePrint "Slow value should be $slowval, good file has $$gooddataref[$curgoodline][1], testfile has $$testdataref[$curtestline][1]\n" if (abs($$gooddataref[$curgoodline][1]-$slowval)>$absdifftolslow || abs($$testdataref[$curtestline][1]-$slowval)>$absdifftolslow);
        print STDERR "Could not sync good file with netlist --- could not find value of $colnameslow=$slowval where it belonged!\n" if ( abs($$gooddataref[$curgoodline][1]-$slowval) > $absdifftolslow);

        $colnamefast=$$expectlineref[1];
        $absdifftolfast = $absdifftol{$colnamefast};

        verbosePrint "Line $curtestline,$curgoodline: Fast value should be $fastval, good file has $$gooddataref[$curgoodline][0], testfile has $$testdataref[$curtestline][0]\n" if (abs($$gooddataref[$curgoodline][0]-$fastval)>$absdifftolfast || abs($$testdataref[$curtestline][0]-$fastval)>$absdifftolfast);


        # For these purposes, failing to find the right value in the good file
        # is a fatal error.  We must have a good good file.

        print STDERR "Failed to find $colnamefast = $fastval on line $curgoodline of good file, get a better \"good\" file!\n" if ( abs($$gooddataref[$curgoodline][0]-$fastval) > $absdifftolfast);
        return -85 if ( abs($$gooddataref[$curgoodline][0]-$fastval) > $absdifftolfast);

        print STDERR "Compare failure: cannot find line in test file matching \n".
            "$colnamefast=$fastval and $colnameslow=$slowval!, looking on $curgoodline \n".
            "in good file, $curtestline in test file\n     The values actually found are \n".
            "$$testdataref[$curtestline][0] and $$testdataref[$curtestline][1]\n"
            if ( abs($$testdataref[$curtestline][0]-$fastval) > $absdifftolfast);
        return -30 if ( abs($$testdataref[$curtestline][0]-$fastval) > $absdifftolfast);
        $curgoodline++; $curtestline++;
      }

      # we only get here if the sweep steps requested are in both good and test files

      $curgoodline=0; $curtestline=0;
      my @good,@test,@atol,@rtol,@adtol;
      $rtol[0]=1;
      $atol[0]=1;
      $adtol[0]=1;
      for (my $loopvar=1;$loopvar<=$#{$expectlineref}-1;$loopvar++)
      {
        $colname=$$expectlineref[$loopvar+1];
        $rtol[$loopvar]=$reltol{$colname};
        $atol[$loopvar]=$abstol{$colname};
        $adtol[$loopvar]=$absdifftol{$colname};
      }

      $fastindex=0;
      for ($index=0; $index < $nslowstep*$nfaststep; $index++)
      {
        $good[$fastindex]=$$gooddataref[$curgoodline];
        $test[$fastindex]=$$testdataref[$curtestline];

        $curtestline++;
        $curgoodline++;
        $fastindex++;

        # We must conclude the integration and start over when we hit the end of
        # a fast sweep.
        if ($fastindex == $nfaststep)
        {
          my @error_new=errNorm(\@test,\@good,\@rtol,\@atol,\@adtol);

          for ( $colindex=0; $colindex <= $#$expectlineref - 1; $colindex++)
          {
            $colname=$$expectlineref[$colindex+1];
            $reltol = $rtol[$colindex];

            $returnval=-20, printf STDERR ("For curve with value of %s = %f, Column %s failed compare, tolerance is %5.3f percent, integrated error is %f percent.\n",$colnameslow,$good[0][1],$colname,$reltol*100,$error_new[$colindex]*$reltol*100) if ($error_new[$colindex] > 1);
            verbosePrint "$colnameslow=$good[0][1]: RMS relative error in $colname is "; verbosePrint $error_new[$colindex]*$reltol*100; verbosePrint "%\n";
          }
          # and start the accumulation over
          $fastindex=0;
        }
      }
    }
    if ($do_plot)
    {
      #     output for plotting
      printf PLOTFH "%17s\t","Index";
      for ($colno=1;$colno <= $#$expectlineref;$colno++)
      {
        printf PLOTFH "%17s\t%17s\t%17s\t%17s\t","$$expectlineref[$colno](test)","$$expectlineref[$colno](good)","Difference","Relative_Error(%)";
      }
      print PLOTFH "\n";
      for ($lineno=0;$lineno<=$#$gooddataref;$lineno++)
      {
        printf PLOTFH "%8d\t",$lineno;
        for ($colno=0;$colno <= $#$expectlineref-1;$colno++)
        {
          $colname=$$expectlineref[$colno+1];
          $absdifftol = $absdifftol{$colname};
          $abstol = $abstol{$colname};
          $reltol = $reltol{$colname};

          printf PLOTFH "%17.12g\t%17.12g\t%17.12g\t%17.12g\t",
          $$testdataref[$lineno][$colno],
          $$gooddataref[$lineno][$colno],
          $$gooddataref[$lineno][$colno]-
              $$testdataref[$lineno][$colno],
              100*$reltol*((abs($$gooddataref[$lineno][$colno]-
                                $$testdataref[$lineno][$colno])
                            <$absdifftol)
                           ? 0 : (($$gooddataref[$lineno][$colno]-
                                   $$testdataref[$lineno][$colno])
                                  /($reltol*abs($$gooddataref[$lineno][$colno])
                                    +$abstol)));
        }
        printf PLOTFH "\n";
      }
    }
  }
  return $returnval;

}

sub log_base {
  my $base = shift;
  my $n = shift;
  return log($n)/log($base);
}


sub splitInput
{
  my $basename = $_[0];
  my $fname_arrayref=$_[1];
  my $i;
  my $headerline;
  my $inputline;
  my @splitline;

  # We're going to open up the file in $basename, extract the first line
  # simply ASSuming it's a valid Xyce output file, then loop around
  # making files called "$basename$$"."_split".$n which have the header
  # dumped into them first, and then all the lines of the input file
  # up until the index restarts at 0.
  open(TOSPLIT,$basename) || croak "Could not open good file $basename";
  debugPrint " splitting $basename\n";

  #  get the first line.  There had better be one.
  $headerline=<TOSPLIT>;
  return -99 if (eof(TOSPLIT));

  $i=-1;
  while ($inputline=<TOSPLIT>)
  {
    @splitline=split(' ',$inputline);
    if ($splitline[0] eq "0")                 # beginning new step, start new
    {
      # clean up currently open file
      print NEWFILE "End of Xyce(TM) Simulation\n" if ($i >= 0);  #fake
      close (NEWFILE) if ($i >= 0);

      $i++;

      #Start new file
      $$fname_arrayref[$i]="$basename"."$$"."_split.".$i;
      debugPrint "  creating $$fname_arrayref[$i]\n";
      open(NEWFILE,"> $$fname_arrayref[$i]") ||
          croak "Could not open output splitfile $$fname_arrayref[$i]";
      print NEWFILE $headerline;
    }
    print NEWFILE $inputline;
    if ($inputline =~ /^End of Xyce/)
    {
      close NEWFILE;  # coz that means there's no more
      last;           # don't bother looping anymore
    }
  }
  close (TOSPLIT);
}

sub checkResFile
{
  my $resfile=$_[0];
  my $inline;
  my @splitline;
  my @filelines;
  my $lineno;
  my $slowindex, $fastindex;
  my $slowval, $fastval;
  my $p_sweep;

  @filelines=();   # make sure nothing there.

  debugPrint "I am supposed to check the file $resfile\n";

  open (RESFILE, $resfile) || croak " Could not open result file $resfile\n";
  $inline=<RESFILE>;
  # parse out the header
  @splitline=split(' ',$inline);
  # we must have: STEP, followed by fast step var, followed by slow step var.
  return -60 if (lc($splitline[0]) ne "step");
  debugPrint "found $splitline[0], it is STEP\n";

  return -61 if (lc($splitline[1]) ne $step_vars[0]);
  debugPrint "found $splitline[1], it is $step_vars[0]\n";

  return -62 if ($n_step_vars>1 && lc($splitline[2]) ne $step_vars[1]);
  debugPrint "found $splitline[2], it is $step_vars[1]\n" if ($n_step_vars>1);


  # Now go through all the sweep steps we expect from the netlist, and
  # verify that the lines in the RES file match.
  $lineno=0;
  while ($inline=<RESFILE>)
  {
    if ($inline =~ /^End of Xyce/)
    {
      close RESFILE;
    }
    else
    {
      $filelines[$lineno]=$inline;
      $lineno++;
    }
  }

  # we now have the entire res file (excepting the header and footer) in
  # filelines.  It damned well better have the same number of lines as we
  # expect there to be sweep steps.
  return -63 if ($n_total_step_steps != $#filelines+1);

  debugPrint "Result file $resfile matches number of lines per netlist request\n";

  # just as with DC, we'll have two distinct cases for sweep=1 or sweep=2 var.
  $theSteps->initializeSweep();
  if ($n_step_vars == 1)
  {
    for ($lineno=0;$lineno <= $#filelines; $lineno++)
    {

      @splitline=split(' ',$filelines[$lineno]);

      return -64 if ($splitline[0] != $lineno);

      ($p_sweep)=$theSteps->getNextSweepStep();
      $p_sweep=sprintf($truncformat,$p_sweep);

      debugPrint "Row $lineno: Expect value for $step_vars[0] of $p_sweep, have $splitline[1]\n";

      # now bitch if they don't agree to within tolerance:
      if (abs($p_sweep-$splitline[1]) > $absdifftol{$step_vars[0]})
      {
        print STDERR "Result file $resfile mismatch at step $lineno, looking for $p_sweep, found $splitline[1]\n";
        return -65;
      }
      debugPrint "OK, $resfile matches all parameter steps we expected.\n";
    }
  }
  else
  {
    for ($lineno=0;$lineno <= $#filelines; $lineno++)
    {
      ($fastval,$slowval)=$theSteps->getNextSweepStep();
      # To guard against failed compares, must truncate to correct precision
      $slowval=sprintf($truncformat,$slowval);
      $fastval=sprintf($truncformat,$fastval);
      @splitline=split(' ',$filelines[$lineno]);

      return -64 if ($splitline[0] != $lineno);

      # now check that fastval lives on first column, slow in second

      if ( abs($splitline[1]-$fastval) > $absdifftol{$step_vars[0]} ||
           abs($splitline[2]-$slowval) > $absdifftol{$step_vars[1]})
      {
        print STDERR "Result file $resfile mismatch at step $lineno, looking for $fastval, $slowval, found $splitline[1], $splitline[2]\n";

        return -65;
      }
    }
    debugPrint "OK, $resfile matches all parameter steps we expected.\n";
  }
}


sub errNorm
{

  my ($fref,$gref,$rtolref,$atolref,$adtolref)=@_;
  my @f=@{$fref};
  my @g=@{$gref};
  my @rtol=@{$rtolref};
  my @atol=@{$atolref};
  my @adtol=@{$adtolref};
  my @erroroutput;
  my @last_integrand,@integrand,@errorsums;
  my $i,$x,$col;
  my $old_perc=0;
  for ($col=0;$col<=$#{$f[$i]};$col++)
  {
    $errorsums[$col]=0.0;
  }

  if ($#f==0)
  {
    # only one row, so no integration
    $erroroutput[0]=0;  #no error in independent variable
    for ($col=1;$col<=$#{$f[0]};$col++)
    {
      $erroroutput[$i] = abs(($g[0][$col]-$f[0][$col])
                             /($rtol[$col]*abs($g[0][$col])+$atol[$col]));
    }
  }
  else
  {
    for ($i=0;$i<=$#f;$i++)
    {
      my $percent_comp=int($i/$#f*100);
      if ($percent_comp != $old_perc)
      {
        #        debugPrint " integrating... $percent_comp %\n";
        $old_perc=$percent_comp;
      }
      for ($col=1;$col<=$#{$f[$i]};$col++)
      {

        if (abs($g[$i][$col]-$f[$i][$col])<$adtol[$col])
        {
          $integrand[$col]= 0;
        }
        else
        {
          $integrand[$col]=($g[$i][$col]-$f[$i][$col])
              /($rtol[$col]*abs($g[$i][$col])+$atol[$col]);
        }

        if (abs($integrand[$i][$col])>1)
        {
          my $junk=$integrand[$col]*$rtol[$col];
          verbosePrint "At independent var value $x column $col has relative error of $junk\n";
        }
        $integrand[$col] *= $integrand[$col];

        if ($i>0)
        {
          $errorsums[$col] += 0.5*($integrand[$col]+$last_integrand[$col])
              *abs($f[$i][0]-$f[$i-1][0]);
        }
      }
      @last_integrand=@integrand;
    }

    $erroroutput[0]=0;   # no error in independent variable;
    for ($col=1;$col<=$#{$f[0]};$col++)
    {
      $erroroutput[$col] = sqrt($errorsums[$col]/abs($f[$#f][0]-$f[0][0]));
    }
  }

  return @erroroutput;
}


#Do linear interpolation of the data in array "G" to the times in array "T", return
# results in an array.
# Meant to work with the "good" and "test" data arrays directly, without having to
# pull out the times manually
sub interpolateTimes
{
  my ($tref,$gref)=@_;
  my @returnArray;
  my @t=@{$tref};
  my @g=@{$gref};
  my $tgoodhi,$tgoodlow;
  my $i,$j;

  if ($g[0][0]>$t[0][0])
  {
    print STDERR "Fatal error: Good series starts later than test series\n";
    exit 1;
  }

  if ($g[$#g][0]<$t[$#t][0])
  {
    print STDERR "Fatal error: Good series ends before test series\n";
    exit 1;
  }

  $tgoodhi=0;

  for ($i=0;$i<=$#t;$i++)
  {
    # look forward in the tin array to find the first time that's
    # later than tinterp[i], starting at the last one we used
    for ($j=$tgoodhi;$j<=$#g;$j++)
    {
      if ($g[$j][0]>=$t[$i][0])
      {
        $tgoodhi=$j;
        $tgoodlow=$j-1;   # This must, by construction, be the last lower one
        last;
      }
    }

    $returnArray[$i][0]=$t[$i][0];

    # now do the interpolation of all the columns
    for ($j=1;$j<=$#{$t[$i]};$j++)
    {
      if ($g[$tgoodhi][0] == $t[$i][0])    # no interpolation necessary
      {
        $returnArray[$i][$j] = $g[$tgoodhi][$j];
      }
      else
      {
        $returnArray[$i][$j]=($g[$tgoodhi][$j]-$g[$tgoodlow][$j])/
            ($g[$tgoodhi][0]-$g[$tgoodlow][0])
            * ($t[$i][0]-$g[$tgoodlow][0])
            + $g[$tgoodlow][$j];
      }
    }
  }
  return @returnArray;
}

# given a .sens line and any .options sensitivity line, figure out what
# our objective function is, what variables we're differentiating
# with respect to, and whether to expect direct or adjoint (or both).
sub parseSensitivity
{
  my ($sensline,$sensoptline,$varsref,$typesref)=@_;
  my $objfunc="";
  
  if ($sensline eq "")
  {
    print "parseSensitivity called, and there is no .sens line?\n";
    exit 1;
  }

  # Format of "param" string is to have comma-separated var names following
  # "param=".  But 
  $sensline =~ s/, */,/g;
  
  my (@splitsens)=split(' ',$sensline);

  debugPrint "ParseSensitivity of '$sensline' \n";

  # get rid of ".sens"
  shift(@splitsens);
  
  for (my $i=0;$i<=$#splitsens;$i++)
  {
    if ($splitsens[$i] =~ /(\w+)=(.*)/)
    {
      if ($1 eq "objfunc")
      {
        debugPrint "ParseSensitivity found 'objfunc=$2' \n";
        $objfunc=$2;
      }
      if ($1 eq "param")
      {
        debugPrint "ParseSensitivity found 'param=$2' \n";
        my (@params)=split(',',$2);
        for (my $j=0; $j<=$#params;$j++)
        {
          # remove whitespaces
          $params[$j] =~ s/\s//g;
          push @$varsref,$params[$j];
        }
      }
    }
  }

  debugPrint "ParseSensitivity got params 0..$#$varsref \n";
  
  # we now know what variables the user has requested sensitivity w.r.t., and
  # have stored them in the caller's array.
  # Now let's see what .options sensitivity we have.  This will determine
  # what output we expect to see in the .SENS.prn file.

  my $do_adjoint=1;
  my $do_direct=0;
  my $do_scaled=0;

  if ($sensoptline ne "")
  {
    my (@splitsensopt)=split(' ',$sensoptline);

    # get rid of ".options sensitivity"
    shift(@splitsensopt);
    shift(@splitsensopt);
    
    for (my $i=0; $i <= $#splitsensopt; $i++)
    {
      if ($splitsensopt[$i] =~ /(\w+)=(.*)/)
      {
        if ($1 eq "direct" and $2 eq "1")
        {
          $do_direct=1;
        }
        if ($1 eq "adjoint" && $2 eq "0")
        {
          $do_adjoint=0;
        }
        if ($1 eq "scaled" && $2 eq "0")
        {
          $do_scaled=0;
        }
      }
    }
  }
  
  debugPrint "parseSensitivy: do_direct=$do_direct, do_adjoint=$do_adjoint\n";
  if ($do_direct == 1)
  {
    push @$typesref,"dir";
    if ($do_scaled==1)
    {
      push @$typesref,"dir_scaled";
    }
  }
  if ($do_adjoint == 1)
  {
    push @$typesref,"adj";
    if ($do_scaled == 1)   
    {
      push @$typesref,"adj_scaled";
    }
  }

  return $objfunc;
}

# Local Variables:
# perl-indent-level: 2
# End:
