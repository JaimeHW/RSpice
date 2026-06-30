# StepSweep
# Class for managing set of Step sweeps from a netlist.
#
#  Given .Step lines provide facilities for re-creating step values.
# The .step line can appear multiple times, so we are slightly different
# from a DC sweep, which can only appear once.

package XyceVerify::StepSweep;

#Constructor
sub new
{
    my ($class,$StepLine) = @_;
    $self={};
    bless $self, $class;
    $self->{n_sweep_vars}=0;
    $self->{sweep_list}=undef;
    if (defined $StepLine)
    {
        $self->addStepLine($StepLine);
    }
    return $self;
}


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


#Basic parsing of step line.  We assume the caller has already downcased the
# line
sub addStepLine
{
    my ($self,$StepLine)=@_;

    my $split_line,$opcode;
    my $dc_nonlin_sweep,$sweep_base;

    @split_line=split(' ',$StepLine);
    #shift away the .Step bit and sanity check:
    $opcode=shift @split_line;
    if ($opcode ne ".step")
    {
        print 
            "Panic --- improper use of StepSweep class has called addStepLine ".
            "with other\n".
            "than something that starts with .STEP\n";
        print "Opcode is $opcode\n";
        exit 1;
    }

    # OK, we have a .step line, and @split_line has its bits

    while ($#split_line gt 0)
    {
        if ($split_line[0] eq "oct")
        {
            $step_nonlin_sweep=1;
            $sweep_base=2.0;
            shift (@split_line);
        }
        elsif ($split_line[0] eq "dec")
        {
            $step_nonlin_sweep=1;
            $sweep_base=10.0;
            shift (@split_line);
        }
        elsif ($split_line[0] eq "lin")
        {
            $step_nonlin_sweep=0;
            shift (@split_line);
        }
        elsif ($split_line[0] eq "list")
        {
            $step_nonlin_sweep=2;
            shift (@split_line);
        }
        else
        {
            $step_nonlin_sweep=0;
            if ($split_line[1] eq "list")
            {
                #Fake it out --- set type, then shift away the list keyword
                # so the param is back in the 0 element
                # We have to do this because list is the odd bird that
                # puts the keyword AFTER the param.
                $step_nonlin_sweep=2;
                my $param=shift(@split_line);
                shift(@split_line);
                unshift (@split_line,$param);
            }
        }

        # Now, if we have a dec, oct, or lin, generate a list of
        # sweep steps

        if ($step_nonlin_sweep != 2)
        {
            my($param, $start, $stop, $step)=@split_line;
            my ($sweep_start,$sweep_stop,$sweep_step);
            shift(@split_line);
            shift(@split_line);
            shift(@split_line);
            shift(@split_line);

            
            $self->{sweep_vars}[$self->{n_sweep_vars}]=$param;
            $self->{sweep_name_to_num}{$self->{sweep_vars}[$self->{n_sweep_vars}]}=
                $self->{n_sweep_vars};
            $sweep_start=modVal2Float($start);
            $sweep_stop= modVal2Float($stop);
            $sweep_step=modVal2Float($step);

            # Now create the list


            if ($step_nonlin_sweep == 0)
            {
                # linear sweep
                # need to take care when going *down* in value
                my $i, $val,$sign;
                $val=$sweep_start;
                $self->{sweep_list}[$self->{n_sweep_vars}]=[];
                ${$self->{sweep_list}[$self->{n_sweep_vars}]}[0]=$val;
                $val += $sweep_step;
                $i=1;
                $sign=($sweep_step<0)?-1:1;
                my $zerofuzz = abs($sweep_step)/1000;
                while ( $sign*($val - $sweep_stop) <= $zerofuzz )
                {
                    ${$self->{sweep_list}[$self->{n_sweep_vars}]}[$i]=$val;
                    $i++;
                    $val = $sweep_start+$i*$sweep_step;
                }

            }
            else
            {
                my $i, $val, $sweep_stepmult;
                # Don't need to be especially careful here, because
                # start value is *always* positive and less than 
                # stop value for these nonlinear sweeps.

                $sweep_stepmult=$sweep_base**(1/$sweep_step);
                $val=$sweep_start;
                $self->{sweep_list}[$self->{n_sweep_vars}]=[];
                ${$self->{sweep_list}[$self->{n_sweep_vars}]}[0]=$val;

                $i=1;
                $val=$sweep_start*($sweep_stepmult**$i);
                while ($val <= $sweep_stop)
                {
                    ${$self->{sweep_list}[$self->{n_sweep_vars}]}[$i]=$val;
                    $i++;
                    $val=$sweep_start*($sweep_stepmult**$i);
                }
  
            }
        }
        else
        {
            # we have a list in the netlist already.  step forward and
            # stuff the things into the array until we find the end of the
            # list.
            # Assumption: the end of the list will be when we hit the
            # end of the line, the keywords OCT, DEC, LIN, or something
            # that starts with a V or an I.
            my ($next_field,$done,$i,$param);
            $done=0;
            
            $param=shift(@split_line);
            $self->{sweep_vars}[$self->{n_sweep_vars}]=$param;
            $self->{sweep_name_to_num}{$self->{sweep_vars}[$self->{n_sweep_vars}]}= $self->{n_sweep_vars};
            $self->{sweep_list}[$self->{n_sweep_vars}]=[];
            $i=0;
            while ($#split_line >= 0 && $done==0)
            {
                $next_field=$split_line[0];
                if ($next_field =~ /^[a-z]/)
                {
                    $done=1;
                }
                else
                {
                    $val=modVal2Float($next_field);
                    ${$self->{sweep_list}[$self->{n_sweep_vars}]}[$i]=$val;
                    $i++;
                    shift(@split_line);
                }
            }
        }
        $self->{n_sweep_vars}++;
    }
}

sub expectPrint
{
    # return array of what we expect to see on the print line
    # -- for a Step sweep this is the same as the thing on the .step line,
    # unlike a DC sweep.

    my ($self)=@_;

    my @array,$i;

    for ($i=0;$i<=$#{$self->{sweep_vars}};$i++)
    {
        $array[$i]=$self->{sweep_vars}[$i];
    }
    return @array;
}

sub dumpOut
{
    my ($self)=@_;
    my $i;
    print "We have $self->{n_sweep_vars} step sweep variables.\n";

    for ($i=0;$i<=$#{$self->{sweep_vars}};$i++)
    {
        print "Sweep variable $i is parameter $self->{sweep_vars}[$i]\n";
        print "@{$self->{sweep_list}[$i]}\n";
        print "The first element is ${$self->{sweep_list}[$i]}[0]\n";
        print "The last element is ".
            "${$self->{sweep_list}[$i]}[$#{$self->{sweep_list}[$i]}]\n";
    }
}

sub nSweepVars
{
    my ($self)=@_;
    return $self->{n_sweep_vars};
}


sub nSweepSteps
{
    my ($self,$i)=@_;
    return $#{$self->{sweep_list}[$i]}+1;
}

sub sweepVar
{
    my ($self,$i)=@_;
    return (wantarray)?@{$self->{sweep_vars}}:$self->{sweep_vars}[$i];
}

sub sweepStart
{
    my ($self,$i)=@_;
    return ${$self->{sweep_list}[$i]}[0];
}

sub sweepStop
{
    my ($self,$i)=@_;
    return ${$self->{sweep_list}[$i]}[$#{$self->{sweep_list}[$i]}];
}

sub initializeSweep
{
    my ($self)=@_;

    my $i;

    for ($i=1;$i<$self->{n_sweep_vars};$i++)
    {
        $self->{sweep_state}[$i]=0;
    }
    $self->{sweep_state}[0]=-1;
}

sub getNextSweepStep
{
    my ($self)=@_;
    my @valArray,$i,$done;

    $done=0;
    for ($i=0;$i<$self->{n_sweep_vars}&&$done==0; $i++)
    {
        $self->{sweep_state}[$i]++;
        if ($self->{sweep_state}[$i] <= $#{$self->{sweep_list}[$i]})
        {
            $done=1;
        }
        else
        {
            $self->{sweep_state}[$i]=0;
        }
    }

    if ($done==1)
    {
        for ($i=0;$i<$self->{n_sweep_vars};$i++)
        {
            $valArray[$i]=${$self->{sweep_list}[$i]}[$self->{sweep_state}[$i]];
        }
    }
    else
    {
        undef @valArray;
    }

    return @valArray;
}

1;
