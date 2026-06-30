#!/usr/bin/env perl
# Take the two idvdh_dt reference standards and massage them into Xyce
# format

@vgs=(0.15,0.25,0.5,0.75,1.0);
@temps=(27,100);
$vdstart=0;
$vdstop=1.2;
$vdincrement=0.01;

$input_basename="asym_nmos_idvdh_dt_T";
$input_suffix=".standard";

$outname="asym_nmos_idvdh_dt.cir.prn.gs";
open (PRNOUT,">$outname") || die "Cannot open $outname for write";
print PRNOUT "Index       V(D)              V(G)              TEMP              V(DT)\n";


# What this script is going to do is simply loop over the temps, vg, and vd
# (in that order) and annotate the CMC reference data, dumping back out in
# a format that will match Xyce's step and nested DC output

for $temp (@temps)
{
    $inputfile=$input_basename.$temp.$input_suffix;
    open (INPFILE,"<$inputfile") || die "cannot open $inputfile for read";
    $line=<INPFILE>;
    $indx=0;
    for $vg (@vgs)
    {
        for ($vd=$vdstart;$vd<$vdstop+$vdincrement;$vd+=$vdincrement)
        {
            $line = <INPFILE>;
            chomp $line;
            ($vd_in,$vdt_in)=split(" ",$line);
            if ($vd_in - $vd > 0.001)
            {
                die "Mismatch between vd_in ($vd_in) and vd expected ($vd)";
            }
            printf PRNOUT "%-8d %16.8e %16.8e %16.8e %16.8e\n",$indx,$vd,$vg,$temp,$vdt_in;
            $indx++;
        }
    }
    close (INPFILE);
}

print PRNOUT "End of Xyce(TM) Parameter Sweep\n";
