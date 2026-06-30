#
# DCSources
#

# Methods and data for analyzing DC swept sources
# 

package XyceVerify::DCSources;

# This class is not meant to be created anew for each source, but rather
# there should be one object for each netlist --- it will hold all the
# info for all the sources in the netlist.

sub new
{
    my ($class,$SourceLine) = @_;
    $self={};
    bless $self, $class;
    $self->{n_sources}=0;
    if (defined $SourceLine)
    {
        $self->addSource($SourceLine);
    }
    return $self;
}

sub addSource
{
    my ($self,$line)=@_;
    my $name;
    my @split_line=split(' ',$line);

    $name=lc($split_line[0]);
    $self->{sources}[$self->{n_sources}]=$name;
    $self->{pos_node}{$name}=$split_line[1];
    $self->{neg_node}{$name}=$split_line[2];
    $self->{n_sources}++;

    #    print "Got source line $line, name is $name\n";
    if ($name =~ /^i/)
    {
        #there's no variations for current sources
        $self->{print_string}{$name}="i($name)";
        $self->{print_sign}{$name}=1.0;
    }
    else
    {
        if ($self->{pos_node}{$name} ne "0" && 
            $self->{neg_node}{$name} ne "0")
        {
            $self->{print_string}{$name}="v($self->{pos_node}{$name},$self->{neg_node}{$name})";
            $self->{print_sign}{$name}=1.0;
        }
        elsif ($self->{pos_node}{$name} ne "0")
        {
            $self->{print_string}{$name}="v($self->{pos_node}{$name})";
            $self->{print_sign}{$name}=1.0;
        }
        elsif ($self->{neg_node}{$name} ne "0")
        {
            $self->{print_string}{$name}="v($self->{neg_node}{$name})";
            $self->{print_sign}{$name}=-1.0;
        }
    }
        
}

sub dumpOut
{
    my ($self)=@_;

    my $i;
    my $name;

    print " Sources:\n";
    for ($i=0;$i<$self->{n_sources}; $i++)
    {
        $name=$self->{sources}[$i];
        print " $name:\n";
        print "  Pos node: $self->{pos_node}{$name}\n";
        print "  neg node: $self->{neg_node}{$name}\n";
        print "   string: $self->{print_string}{$name}\n";
    }
}

sub expectPrint
{
    my ($self,$source_name)=@_;

# Return array of name and sign.
    return ($self->{print_string}{$source_name},
            $self->{print_sign}{$source_name});
}

sub printSign
{
    #same as above, but don't bother with the print string.
    my ($self,$source_name)=@_;

    return $self->{print_sign}{$source_name};
}

# these are little hacks to allow interim backwards compatible grafting onto
# the xyce_verify script
sub getNumSources
{
    my ($self)=@_;
    return $self->{n_sources};
}

sub getPosNode
{
    my ($self,$name)=@_;
    return $self->{pos_node}{$name};
}

sub getNegNode
{
    my ($self,$name)=@_;
    return $self->{neg_node}{$name};
}


1;
