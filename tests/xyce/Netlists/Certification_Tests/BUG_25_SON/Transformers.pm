#
# This module library defines two classes, IsolationTransformer and 
# TappedTransformer.  Each class takes parameters defining a circuit
# fragment to its constructor, and provides methods to generate a netlist
# fragment that implements that circuit, and to generate the analytic solution
# to that circuit.
#
{
    package IsolationTransformer;

# Simulation and analytic solution for a circuit fragment containing
# a simple transformer.
#   
#    2       R1     3      4     R2        5
#    +----/\/\/\----+      +---/\/\/\/\----+
#    |              |      |               |
#  +--+             _) || (_              +--+
#  |  |V1=0         _) || (_              |  |  V2=0 
#  +--+             _) || (_              +--+
#    + 1            _) || (_               |
#  +---+        Lp  _) || (_  Ls           |
#  | ^ |            _) || (_               |
#  | | | I1         _) || (_               |
#  +---+            _) || (_               |
#    |              |       |              |
#    +--------------+-------+--------------+
#    |
#  -----
#   ---
#    -
#
#
# In this circuit, I1 is an imposed sinusoidal current source of amplitude I0,
# and a solution is required for the current in the secondary loop, I2 and
# the voltage at node 4.  This is done by solving the differential 
# equation:
#
#  dI2/dt + (R2/Ls)*I2 = -(MPS/Ls)*dI1/dt
#
#  where MPS is the mutual inductance between primary and secondary, and 
#  which is given by MPS = KPS*sqrt(Lp*Ls), with KPS being the coupling 
#  coefficient, and I1=I0*sin(2*pi*f*t)
# 
#  The solution to this problem is 
#  I2 = MPS*I0*2*pi*f/(Ls*((2*pi*f)**2+(R2/Ls)**2))*
#       (-(R2/Ls)*cos(2*pi*f*t) - 2*pi*f*sin(2*pi*f) + (R2/Ls)*exp(-(R2/Ls)*t)) 
#
#  The voltage at node 4 is obtained by noting that 
#    V(4)-V(0) = -Ls*dI2/dt - MPS*dI1/dt
#  which solution can be had after I2 is solved for.
#
    # Constructor:
    # Takes arguments:
    #   new(I0,f,R1,Lp,R2,Ls, nodeMin)
    #  I0 is amplitude of sinusoidal current source in primary loop
    #  f is frequency of current source
    # R1 is resistance in primary loop
    # Lp is inductance of primary winding
    # R2 is resistance in secondary loop
    # Ls is inductance of secondary winding
    # nodeMin is minimum node number to use for netlisting
    # ID is a unique ID for this transformer instance
    sub new
    {
        my $class=shift;
        my ($I0,$f,$R1,$Lp,$R2,$Ls,$nodeMin,$ID)=@_;
        my $self={
            ID=>$ID,
            I0=>$I0,
            f=>$f,
            R1=>$R1,
            Lp=>$Lp,
            R2=>$R2,
            Ls=>$Ls,
            nodeMin=>$nodeMin,
            nodeMax=>$nodeMin+4,
            KPS=>1.0,
        };
        bless $self,$class;
        return $self;
    }

    # generate a netlist fragment for the circuit defined by our parameters.
    sub generateNetlist
    {
        my $self=shift;
        my @netlist;
        my $node=$self->{nodeMin};
        my $tempStr;
        # Primary loop
        push @netlist,"I1_$self->{ID} 0 $node sin ( 0 $self->{I0} $self->{f})";
        $tempStr="V1_$self->{ID} $node ";
        $node++;
        $tempStr .= " $node 0 ";
        push @netlist,$tempStr;
        $tempStr  ="R1_$self->{ID} $node ";
        $node++;
        $tempStr .=" $node $self->{R1}";
        push @netlist,$tempStr;
        push @netlist,"Lp_$self->{ID} $node 0 $self->{Lp}";
        $node++;
        
        #secondary loop
        push @netlist,"Ls_$self->{ID} $node 0 $self->{Ls}";
        
        $tempStr  ="R2_$self->{ID} $node ";
        $node++;
        $tempStr .=" $node $self->{R2}";
        push @netlist,$tempStr;
        $tempStr="V2_$self->{ID} 0 $node 0";
        push @netlist,$tempStr;

        # The K device is always the last one:
        push @netlist,"K1_$self->{ID} Lp_$self->{ID} Ls_$self->{ID} $self->{KPS}";
        return @netlist;
    }

    # Return an array of strings telling what values we'll provide
    # from either netlist or analytic solutions
    sub getPrintTerms
    {
        my $self=shift;
        my @returnArray;
        my $tempStr;
        push @returnArray,"I(I1_$self->{ID})";
        push @returnArray,"I(V2_$self->{ID})";
        $tempStr="V(";
        $tempStr .= $self->{nodeMax}-1;
        $tempStr .= ")";
        push @returnArray,$tempStr;
        return @returnArray;
    }

    # This function takes a list of time points and evaluates the analytic
    # solution of this transformer problem.  Returns the current in primary
    # (just the driving source), the current in secondary, and the voltage
    # at the positive node of the secondary winding.
    sub evaluateAnalytic
    {
        my $self=shift;
        my @timepoints=@_;
        my @results;
        my $time,$I1,$I2,$V;
        my $pi=3.14159265358979;
        my $R2=$self->{R2};
        my $Ls=$self->{Ls};
        my $f=$self->{f};
        my $I0=$self->{I0};
        my $MPS=$self->{KPS}*sqrt($self->{Lp}*$self->{Ls});
        my $denom=$Ls*((2*$pi*$f)**2+($R2/$Ls)**2);
        my $coef=$MPS*$I0*2*$pi*$f/$denom;
        my $V, $I1,$I2;
        foreach $time (@timepoints)
        {
            
            $I1=$I0*sin(2*$pi*$f*$time);
            $I2=$coef*(-($R2/$Ls)*cos(2*$pi*$f*$time) -
                       2*$pi*$f*sin(2*$pi*$f*$time) +
                       $R2/$Ls*exp(-($R2/$Ls)*$time));
            # Voltage across secondary is -LdI2/dt-MPSdI1/dt.  Sign difference
            # here due to sense of current in Xyce.
            $V=-(-$Ls*$coef*(($R2/$Ls)*2*$pi*$f*sin(2*$pi*$f*$time) -
                           (2*$pi*$f)**2*cos(2*$pi*$f*$time) +
                           -($R2/$Ls)**2*exp(-($R2/$Ls)*$time))
                - $MPS*$I0*2*$pi*$f*cos(2*$pi*$f*$time));

            $results[$#results+1] = [$time,$I1,$I2,$V];
        }
        return @results;
    }
    sub getMaxNode
    {
        my $self=shift;
        return $self->{nodeMax};
    }

}


{
# Simulation and analytic solution for a circuit fragment containing
# a tapped transformer.
#   
#    2       R1     3      4     R2        5
#    +----/\/\/\----+      +---/\/\/\/\----+
#    |              |      |               |
#  +--+             _) || (_              +--+
#  |  |V1=0         _) || (_  Ls1         |  |  V2=0 
#  +--+             _) || (_              +--+
#    + 1            _) ||  |               |
#  +---+        Lp  _) ||  +  6            |
#  | ^ |            _) || (_               |
#  | | | I1         _) || (_  Ls2          |
#  +---+            _) || (_               |
#    |              |       |              |
#    +--------------+-------+--------------+
#    |
#  -----
#   ---
#    -
#
#
# In this circuit, I1 is an imposed sinusoidal current source of amplitude I0,
# and a solution is required for the current in the secondary loop, I2 and
# the voltage at nodes 4 and 6.  This is done by solving the differential 
# equation:
#
#  dI2/dt + (R2/(Ls1+Ls2))*I2 = -((MPS1+MPS2)/(Ls1+Ls2))*dI1/dt
#
# or
#    dI2/dt + (R2/(LsTot))*I2 = -((MPSTot)/(LsTot))*dI1/dt
#  where MPS1 and MPS2 are the mutual inductances between primary and
#  each of the secondary transformers, and 
#  which is given by MPSi = KPSi*sqrt(Lp*Lsi), with KPSi being the coupling 
#  coefficient, and I1=I0*sin(2*pi*f*t).  LsTot and MPSTot are just the sums
#  of the individual L and M terms.
# 
#  The solution to this problem is 
#  I2 = MPSTot*I0*2*pi*f/(LsTot*((2*pi*f)**2+(R2/LsTot)**2))*
#       (-(R2/LsTot)*cos(2*pi*f*t) - 2*pi*f*sin(2*pi*f) 
#        + (R2/LsTot)*exp(-(R2/LsTot)*t)) 
#
#  The voltage at nodes 4 and 6 are obtained from
#   V(4)-V(6) = -Ls1*dI2/dt - MPS1*dI1/dt
#   V(6)-V(0) = -Ls2*dI2/dt - MPS2*dI1/dt
#
    package TappedTransformer;

    # Constructor:
    # Takes arguments:
    #   new(I0,f,R1,Lp,R2,Ls1,Ls2, KPS1, KPS2,nodeMin)
    #  I0 is amplitude of sinusoidal current source in primary loop
    #  f is frequency of current source
    # R1 is resistance in primary loop
    # Lp is inductance of primary winding
    # R2 is resistance in secondary loop
    # Ls1 and Ls2 are the inductance of secondary windings
    # KPS1 and KPS2 are the coupling coefficients between primary winding
    #      and secondary windings.
    # nodeMin is minimum node number to use for netlisting
    # ID is a unique ID for this transformer instance
    sub new
    {
        my $class=shift;
        my ($I0,$f,$R1,$Lp,$R2,$Ls1,$Ls2,$KPS1,$KPS2,$nodeMin,$ID)=@_;
        my $self={
            ID=>$ID,
            I0=>$I0,
            f=>$f,
            R1=>$R1,
            Lp=>$Lp,
            R2=>$R2,
            Ls1=>$Ls1,
            Ls2=>$Ls2,
            nodeMin=>$nodeMin,
            nodeMax=>$nodeMin+5,
            KPS1=>$KPS1,
            KPS2=>$KPS2,
        };
        bless $self,$class;
        return $self;
    }

    # generate a netlist fragment for the circuit defined by our parameters.
    sub generateNetlist
    {
        my $self=shift;
        my @netlist;
        my $node=$self->{nodeMin};
        my $tempStr;
        # Primary loop
        push @netlist,"I1_$self->{ID} 0 $node sin ( 0 $self->{I0} $self->{f})";
        $tempStr="V1_$self->{ID} $node ";
        $node++;
        $tempStr .= " $node 0 ";
        push @netlist,$tempStr;
        $tempStr  ="R1_$self->{ID} $node ";
        $node++;
        $tempStr .=" $node $self->{R1}";
        push @netlist,$tempStr;
        push @netlist,"Lp_$self->{ID} $node 0 $self->{Lp}";
        $node++;
        
        #secondary loop
        my $node_top=$node;
        my $node_middle=$node+2;
        my $node_res=$node+1;
        push @netlist,"Ls1_$self->{ID} $node_top $node_middle $self->{Ls1}";
        push @netlist,"Ls2_$self->{ID} $node_middle 0 $self->{Ls2}";
        
        $tempStr  ="R2_$self->{ID} $node_top ";
        $tempStr .=" $node_res $self->{R2}";
        push @netlist,$tempStr;
        $tempStr="V2_$self->{ID} 0 $node_res 0";
        push @netlist,$tempStr;

        # The K devices are always the last ones:
        push @netlist,"K1_$self->{ID} Lp_$self->{ID} Ls1_$self->{ID} $self->{KPS1}";
        push @netlist,"K2_$self->{ID} Lp_$self->{ID} Ls2_$self->{ID} $self->{KPS2}";


        return @netlist;
    }

    # Return an array of strings telling what values we'll provide
    # from either netlist or analytic solutions
    sub getPrintTerms
    {
        my $self=shift;
        my @returnArray;
        my $tempStr;
        push @returnArray,"I(I1_$self->{ID})";
        push @returnArray,"I(V2_$self->{ID})";
        $tempStr="V(";
        $tempStr .= $self->{nodeMax}-2;
        $tempStr .= ")";
        push @returnArray,$tempStr;
        push @returnArray,"V($self->{nodeMax})";
        return @returnArray;
    }

    # This function takes a list of time points and evaluates the analytic
    # solution of this transformer problem.  Returns the current in primary
    # (just the driving source), the current in secondary, and the voltages
    # at the positive node of the secondary windings.
    sub evaluateAnalytic
    {
        my $self=shift;
        my @timepoints=@_;
        my @results;
        my $time,$I1,$I2,$V;
        my $pi=3.14159265358979;
        my $R2=$self->{R2};
        my $LsTot=$self->{Ls1}+$self->{Ls2};
        my $Ls1 = $self->{Ls1};
        my $Ls2 = $self->{Ls2};
        my $f=$self->{f};
        my $I0=$self->{I0};
        my $MPS1=$self->{KPS1}*sqrt($self->{Lp}*$self->{Ls1});
        my $MPS2=$self->{KPS2}*sqrt($self->{Lp}*$self->{Ls2});
        my $MPSTot=$MPS1+$MPS2;
        my $denom=$LsTot*((2*$pi*$f)**2+($R2/$LsTot)**2);
        my $coef=$MPSTot*$I0*2*$pi*$f/$denom;
        my $V_top,$V_mid,$V,$I1,$I2,$dI1dt,$dI2dt;
        foreach $time (@timepoints)
        {
            
            $I1=$I0*sin(2*$pi*$f*$time);
            $dI1dt=$I0*2*$pi*$f*cos(2*$pi*$f*$time);
            $I2=$coef*(-($R2/$LsTot)*cos(2*$pi*$f*$time) -
                       2*$pi*$f*sin(2*$pi*$f*$time) +
                       $R2/$LsTot*exp(-($R2/$LsTot)*$time));
            $dI2dt=$coef*(($R2/$LsTot)*(2*$pi*$f)*sin(2*$pi*$f*$time) -
                          (2*$pi*$f)**2*cos(2*$pi*$f*$time) +
                          -($R2/$LsTot)**2*exp(-($R2/$LsTot)*$time));
            # Voltage across secondary windings are -LdI2/dt-MPSdI1/dt.  
            # Sign differences here due to sense of current in Xyce.
            # $V_mid is the voltage at the tap.  $V_top is the voltage
            # drop across the top inductor.  Nodal voltage at positive node
            # of the top inductor is determined from $V_top and $V_mid.

            $V_mid = -(-$Ls2*$dI2dt-$MPS2*$dI1dt);
            $V_top = -(-$Ls1*$dI2dt-$MPS1*$dI1dt);
            $V=$V_top+$V_mid;

            $results[$#results+1] = [$time,$I1,$I2,$V,$V_mid];
        }
        return @results;
    }

    sub getMaxNode
    {
        my $self=shift;
        return $self->{nodeMax};
    }
}
1;

        
    

