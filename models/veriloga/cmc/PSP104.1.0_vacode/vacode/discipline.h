// *********************************************************************************************************************
//
//     Discipline File for MOSFET model checking : electrical and thermal only
//
// *********************************************************************************************************************

// ***********************************  Electrical  ***********************************

// Current
nature Current
    units          =  "A";
    access         =  I;
    idt_nature     =  Charge;
    `ifdef CURRENT_ABSTOL
        abstol         = `CURRENT_ABSTOL;
    `else
        abstol         =  1.0e-12;
    `endif
endnature

// Charge
nature Charge
    units          =  "coul";
    access         =  Q;
    ddt_nature     =  Current;
    `ifdef CHARGE_ABSTOL
        abstol         = `CHARGE_ABSTOL;
    `else
        abstol         =  1.0e-14;
    `endif
endnature

// Potential in volts
nature Voltage
    units          =  "V";
    access         =  V;
    idt_nature     =  Flux;
    `ifdef VOLTAGE_ABSTOL
        abstol         = `VOLTAGE_ABSTOL;
    `else
        abstol         =  1.0e-6;
    `endif
endnature

// Flux
nature Flux;
    units          =  "Wb";
    access         =  Phi;
    ddt_nature     =  Voltage;
    `ifdef FLUX_ABSTOL
        abstol         = `FLUX_ABSTOL;
    `else
        abstol         =  1.0e-9;
    `endif
endnature

// Conservative discipline
discipline electrical;
    potential Voltage;
    flow Current;
enddiscipline

// Signal flow disciplines
discipline voltage;
    potential Voltage;
enddiscipline

discipline current;
    flow Current;
enddiscipline

// ***********************************  Thermal  ***********************************
// Temperature
nature Temperature;
    units          =  "C";
    access         =  Temp;
    `ifdef TEMPERATURE_ABSTOL
        abstol         = `TEMPERATURE_ABSTOL;
    `else
        abstol         =  1.0e-4;
    `endif
endnature

// Power
nature Power;
    units          =  "W";
    access         =  Pwr;
    `ifdef POWER_ABSTOL
        abstol         = `POWER_ABSTOL;
    `else
        abstol         =  1.0e-9;
    `endif
endnature

// Conservative discipline
discipline thermal;
    potential Temperature;
    flow Power;
enddiscipline

