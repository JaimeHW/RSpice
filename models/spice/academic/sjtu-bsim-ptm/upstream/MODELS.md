# SPICE Model Parameters Reference

This document provides a comprehensive reference table of SPICE model parameters organized by model card type. Each section clearly indicates which SPICE model level or card the parameters belong to, along with their equivalents across NGSPICE, HSPICE, and SPECTRE simulators.

## Table of Contents

### [SPICE Simulator Comparison](#spice-simulator-comparison)
- [Overview](#overview)
- [NGSPICE](#ngspice)
- [HSPICE](#hspice)
- [SPECTRE](#spectre)
- [Key Differences Summary](#key-differences-summary)
- [Syntax Differences](#syntax-differences)
- [Model Selection Guidelines](#model-selection-guidelines)

### [Model Card Organization](#model-card-organization)
- [MOSFET Model Cards](#mosfet-model-cards)
- [Other Transistor Models](#other-transistor-models)
- [Bipolar Transistor Models](#bipolar-transistor-models)
- [Diode Models](#diode-models)
- [Passive Component Models](#passive-component-models)

### [MOSFET Model Parameters](#mosfet-model-parameters)
- [Level 1 MOSFET Model (LEVEL=1)](#level-1-mosfet-model)
- [Level 2 MOSFET Model (LEVEL=2)](#level-2-mosfet-model)
- [Process Variation and Delta Parameters](#process-variation-parameters)
- [Level 3 MOSFET Model (LEVEL=3)](#level-3-mosfet-model)
- [Parameter Name Evolution](#parameter-name-evolution)
  - [Historical Development of SPICE Model Parameters](#historical-development-of-spice-model-parameters)
- [BSIM1 Model (LEVEL=4, VERSION=1)](#bsim1-model)
- [BSIM2 Model (LEVEL=5, VERSION=2)](#bsim2-model)
- [BSIM3v1 Model (LEVEL=8, VERSION=3.1)](#bsim3v1-model)
- [BSIM3v2 Model (LEVEL=8, VERSION=3.2)](#bsim3v2-model)
- [BSIM3v3 Model (LEVEL=8, VERSION=3.3)](#bsim3v3-model)
- [Advanced BSIM Parameters](#advanced-bsim-parameters)
- [BSIM4.0 Model (LEVEL=14, VERSION=4.0)](#bsim4-model)
- [BSIM4.1 Model (LEVEL=14, VERSION=4.1)](#bsim41-model)
- [BSIM4.2 Model (LEVEL=14, VERSION=4.2)](#bsim42-model)
- [BSIM4.3 Model (LEVEL=14, VERSION=4.3)](#bsim43-model)
- [BSIM4.4 Model (LEVEL=14, VERSION=4.4)](#bsim44-model)
- [BSIM4.5 Model (LEVEL=14, VERSION=4.5)](#bsim45-model)
- [BSIM4.6 Model (LEVEL=14, VERSION=4.6)](#bsim46-model)
- [BSIM4.7 Model (LEVEL=14, VERSION=4.7)](#bsim47-model)
- [BSIM4.8 Model (LEVEL=14, VERSION=4.8)](#bsim48-model)
- [SOI MOSFET Models](#soi-mosfet-models)
- [FinFET Models](#finfet-models)
- [High Voltage MOSFET Models](#high-voltage-mosfet-models)

### [Other Transistor Models](#other-transistor-models-1)
- [JFET Model](#jfet-model)
- [MESFET Model](#mesfet-model)

### [Bipolar Transistor Models](#bipolar-transistor-models-1)
- [Gummel-Poon BJT Model](#gummel-poon-bjt-model)

### [Diode Models](#diode-models-1)
- [SPICE Diode Model](#spice-diode-model)
- [Advanced Diode Parameters](#enhanced-diode-models)

### [Passive Component Models](#passive-component-models-1)
- [Resistor Models](#resistor-models)
  - [Linear Resistor Parameters](#linear-resistor-parameters)
  - [Advanced Resistor Parameters](#advanced-resistor-parameters)
- [Capacitor Models](#capacitor-models)
  - [Linear Capacitor Parameters](#linear-capacitor-parameters)

### [Additional Model Parameters](#additional-model-parameters)
- [Resistor Model Parameters](#resistor-model-parameters)
- [Capacitor Model Parameters](#capacitor-model-parameters)
- [BJT Model Parameters](#bjt-model-parameters)
- [Diode Model Parameters](#diode-model-parameters)
- [Inductor Model Parameters](#inductor-model-parameters)
- [Transmission Line Parameters](#transmission-line-parameters)
- [MOSFET Noise Parameters](#mosfet-noise-parameters)
- [Resistor Noise Parameters](#resistor-noise-parameters)
- [BJT Noise Parameters](#bjt-noise-parameters)
- [Process Variation Parameters](#process-variation-parameters)
- [Mismatch Parameters](#mismatch-parameters)
- [Monte Carlo Parameters](#monte-carlo-parameters)
- [S-Parameter Model Parameters](#s-parameter-model-parameters)
- [Varactor Model Parameters](#varactor-model-parameters)
- [RF MOSFET Parameters](#rf-mosfet-parameters)

### [References](#references)

## SPICE Simulator Comparison

### Overview

This section provides a comprehensive comparison of the three SPICE simulators covered in this reference: NGSPICE, HSPICE, and SPECTRE. Understanding their differences is crucial for selecting the right simulator for your design needs.

### NGSPICE
**Open-source SPICE simulator (free)**

**Key Characteristics:**
- **Origin**: Derived from Berkeley SPICE 3f5, maintained by the open-source community
- **Licensing**: BSD license (completely free)
- **Focus**: Educational, research, and basic analog/mixed-signal design
- **Performance**: Good for small to medium circuits (<100K transistors)

**Strengths:**
- ✅ **Zero cost** - Ideal for education and startups
- ✅ **Source code access** - Can be modified and extended
- ✅ **Berkeley SPICE compatibility** - Standard SPICE syntax
- ✅ **Active development** - Regular updates and bug fixes
- ✅ **Cross-platform** - Runs on Linux, Windows, macOS

**Limitations:**
- ❌ **Limited scalability** - Struggles with very large circuits
- ❌ **Basic RF capabilities** - Limited high-frequency analysis
- ❌ **Commercial support** - Community-based support only
- ❌ **Advanced reliability modeling** - Limited statistical analysis

**Best for:**
- Academic research and teaching
- Small analog designs and verification
- Learning SPICE fundamentals
- Basic mixed-signal simulations

### HSPICE
**Commercial SPICE simulator (Synopsys)**

**Key Characteristics:**
- **Origin**: Industry-standard simulator from Synopsys (formerly Meta-Software)
- **Licensing**: Commercial license (expensive)
- **Focus**: Advanced analog, RF, and mixed-signal design
- **Performance**: Excellent for large circuits (millions of transistors)

**Strengths:**
- ✅ **Industry standard** - Used by most semiconductor companies
- ✅ **Advanced models** - Supports BSIM4, BSIM-CMG, PSP, HiSIM, etc.
- ✅ **RF excellence** - Superior high-frequency and noise analysis
- ✅ **Statistical analysis** - Monte Carlo, mismatch, process variation
- ✅ **Large circuit handling** - Optimized for massive designs
- ✅ **Commercial support** - 24/7 support from Synopsys
- ✅ **Integration** - Seamless integration with Synopsys design flow

**Limitations:**
- ❌ **High cost** - Expensive licensing fees
- ❌ **Proprietary** - No source code access
- ❌ **Learning curve** - Complex setup and optimization

**Best for:**
- Production IC design and verification
- Advanced RF/microwave circuits
- Large-scale mixed-signal SoCs
- Statistical yield analysis
- Commercial semiconductor development

### SPECTRE
**Commercial RF/mixed-signal simulator (Cadence)**

**Key Characteristics:**
- **Origin**: Cadence's flagship simulator, part of Virtuoso platform
- **Licensing**: Commercial license (expensive)
- **Focus**: RF, microwave, high-speed, and mixed-signal design
- **Performance**: Excellent for RF and timing-critical circuits

**Strengths:**
- ✅ **RF supremacy** - Best-in-class RF/microwave analysis
- ✅ **Mixed-signal integration** - Seamless analog-digital co-simulation
- ✅ **High-speed timing** - Advanced timing and jitter analysis
- ✅ **Virtuoso integration** - Native integration with Cadence tools
- ✅ **Advanced modeling** - Comprehensive device model support
- ✅ **Parallel processing** - Multi-core simulation acceleration
- ✅ **Commercial support** - Full Cadence support infrastructure

**Limitations:**
- ❌ **High cost** - Expensive licensing fees
- ❌ **Proprietary** - No source code access
- ❌ **Learning curve** - Complex integration with Virtuoso

**Best for:**
- RF/microwave IC design
- High-speed SerDes and transceivers
- Mixed-signal SoCs with Virtuoso
- PLLs, ADCs/DACs, and timing circuits
- Advanced RF analysis (S-parameters, noise figure, harmonics)

## Key Differences Summary

| Aspect | NGSPICE | HSPICE | SPECTRE |
|--------|---------|--------|---------|
| **Cost** | Free | $$$$ | $$$$ |
| **License** | Open-source | Commercial | Commercial |
| **Primary Focus** | Education/Basic analog | Advanced analog | RF/Mixed-signal |
| **Circuit Size** | Small-Medium | Any size | Any size |
| **RF Capabilities** | Basic | Excellent | Superior |
| **Statistical Analysis** | Limited | Excellent | Good |
| **Syntax Style** | Berkeley SPICE | Extended SPICE | Cadence-specific |
| **Parameter Case** | Case-sensitive | Case-insensitive | Case-insensitive |
| **Model Support** | BSIM1-3 | All models | All models |
| **Support** | Community | Synopsys | Cadence |
| **Integration** | Standalone | Synopsys flow | Virtuoso platform |

## Syntax Differences

### Model Declaration
```spice
# NGSPICE
.MODEL mymos NMOS LEVEL=1

# HSPICE
.MODEL mymos NMOS LEVEL=1

# SPECTRE
model mymos mos type=n level=1
```

### Parameter Names
```spice
# NGSPICE (case-sensitive)
VTO = 0.7
KP  = 50e-6

# HSPICE (case-insensitive)
vth0 = 0.7
u0   = 600.0

# SPECTRE (lowercase)
vth  = 0.7
kp   = 50e-6
```

<a id="model-selection-guidelines"></a>

## Model Selection Guidelines

1. **Level 1**: Simple, fast, good for basic analog design
2. **BSIM3v3**: Industry standard for deep submicron technologies (130nm-90nm)
3. **BSIM4.0-4.3**: Advanced models for nanometer technologies (65nm-45nm)
4. **BSIM4.4-4.6**: Enhanced RF and noise modeling (32nm-22nm)
5. **BSIM4.7-4.8**: FinFET and advanced 3D transistor modeling (14nm and below)
6. **PSP/HiSIM**: Surface potential based models for advanced nodes

## BSIM Model Evolution

| Model Version | Technology Node | Key Features Added | Primary Use Case |
|---------------|-----------------|-------------------|------------------|
| **BSIM1** | 2μm-0.8μm | Short-channel effects, temperature | Early CMOS processes |
| **BSIM2** | 0.5μm-0.35μm | Enhanced physics, subthreshold | Deep submicron transition |
| **BSIM3v1** | 0.25μm-0.18μm | Basic deep submicron | 180nm technology |
| **BSIM3v2** | 0.18μm-0.13μm | Capacitance, noise models | 130nm technology |
| **BSIM3v3** | 0.13μm-90nm | Industry standard | 90nm technology |
| **BSIM4.0** | 90nm-65nm | Nanometer effects | 65nm technology |
| **BSIM4.1** | 65nm-45nm | Stress effects | 45nm technology |
| **BSIM4.2** | 45nm-32nm | Well proximity effects | 32nm technology |
| **BSIM4.3** | 32nm-22nm | LOD effects | 22nm technology |
| **BSIM4.4** | 22nm-14nm | RF modeling | 14nm FinFET |
| **BSIM4.5** | 14nm-10nm | Gate tunneling | 10nm technology |
| **BSIM4.6** | 10nm-7nm | Advanced RF | 7nm technology |
| **BSIM4.7** | 7nm-5nm | FinFET modeling | 5nm technology |
| **BSIM4.8** | 5nm-3nm | Quantum effects | 3nm and beyond |

## Parameter Extraction

Model parameters are typically extracted using:
- DC I-V measurements
- Capacitance measurements
- Temperature characterization
- Process variation analysis

## Model Card Organization

### MOSFET Model Cards
- **[Level 1 MOSFET Model](#level-1-mosfet-model)** - Basic SPICE MOSFET model (LEVEL=1)
- **[Level 2 MOSFET Model](#level-2-mosfet-model)** - Enhanced SPICE MOSFET model (LEVEL=2)
- **[Level 3 MOSFET Model](#level-3-mosfet-model)** - Semi-empirical MOSFET model (LEVEL=3)
- **[BSIM1 Model](#bsim1-model)** - Berkeley Short-channel IGFET Model v1
- **[BSIM2 Model](#bsim2-model)** - Berkeley Short-channel IGFET Model v2
- **[BSIM3v3 Model](#bsim3v3-model)** - Berkeley Short-channel IGFET Model v3.3
- **[BSIM4 Model](#bsim4-model)** - Berkeley Short-channel IGFET Model v4
- **[Advanced BSIM Parameters](#advanced-bsim-parameters)** - Additional quantum mechanical and layout-dependent parameters
- **[BSIM5 Model](#bsim5-model)** - Berkeley Short-channel IGFET Model v5
- **[Process Variation and Delta Parameters](#process-variation-parameters)** - Process variation and delta parameters
- **[PSP Model](#psp-model)** - Penn State Philips MOSFET Model
- **[HiSIM Model](#hisim-model)** - Hiroshima University STARC IGFET Model
- **[SOI MOSFET Models](#soi-mosfet-models)** - Silicon-on-Insulator MOSFET models
- **[FinFET Models](#finfet-models)** - 3D FinFET transistor models
- **[High Voltage MOSFET Models](#high-voltage-mosfet-models)** - High voltage MOSFET models

### Other Transistor Models
- **[JFET Model](#jfet-model)** - Junction Field Effect Transistor
- **[MESFET Model](#mesfet-model)** - Metal Semiconductor Field Effect Transistor

### Bipolar Transistor Models
- **[Gummel-Poon BJT Model](#gummel-poon-bjt-model)** - Standard SPICE BJT model
- **[VBIC Model](#vbic-model)** - Vertical Bipolar Inter-Company Model
- **[MEXTRAM Model](#mextram-model)** - Most EXcellent TRAnsistor Model

### Diode Models
- **[SPICE Diode Model](#spice-diode-model)** - Standard SPICE diode model
- **[Enhanced Diode Models](#enhanced-diode-models)** - Advanced diode models

### Passive Component Models
- **[Resistor Models](#resistor-models)** - Linear and advanced resistor models
- **[Capacitor Models](#capacitor-models)** - Linear and advanced capacitor models
- **[Inductor Models](#inductor-models)** - Linear and advanced inductor models

## MOSFET Model Parameters

<a id="level-1-mosfet-model"></a>

### Level 1 MOSFET Model (LEVEL=1)

**⚠️ Parameter Name Differences Across Simulators:**

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| **VTO** / **VTH0** | Threshold voltage | `vto` | `vth0` | `vth` |
| **KP** | Transconductance parameter | `kp` | `u0` | `kp` |
| GAMMA | Bulk threshold parameter | gamma | gamma | gamma |
| PHI | Surface potential | phi | phi | phi |
| LAMBDA | Channel length modulation | lambda | lambda | lambda |
| TOX | Oxide thickness | tox | tox | tox |
| LD | Lateral diffusion | ld | ld | ld |
| WD | Width reduction | wd | wd | wd |
| U0 | Surface mobility | u0 | u0 | u0 |
| RSH | Sheet resistance | rsh | rsh | rsh |
| CJ | Zero-bias bulk junction capacitance | cj | cj | cj |
| CJSW | Zero-bias sidewall bulk junction capacitance | cjsw | cjsw | cjsw |
| MJ | Bulk junction grading coefficient | mj | mj | mj |
| MJSW | Sidewall bulk junction grading coefficient | mjsw | mjsw | mjsw |
| PB | Bulk junction potential | pb | pb | pb |
| CGSO | Gate-source overlap capacitance | cgso | cgso | cgso |
| CGDO | Gate-drain overlap capacitance | cgdo | cgdo | cgdo |
| CGBO | Gate-bulk overlap capacitance | cgbo | cgbo | cgbo |
| AT | Flicker noise temperature coefficient | at | at | at |

**Key Differences:**
- **Threshold voltage**: `VTO` (NGSPICE) vs `VTH0` (HSPICE) vs `vth` (SPECTRE)
- **Transconductance**: `KP` (NGSPICE/SPECTRE) vs `U0` (HSPICE)
- **Parameter case**: NGSPICE is case-sensitive, HSPICE/SPECTRE are case-insensitive

<a id="level-2-mosfet-model"></a>

### Level 2 MOSFET Model (LEVEL=2)

*Note: Includes all Level 1 parameters plus additional short-channel effects*

**✅ Consistent Parameter Names Across Simulators:**

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| NFS | Fast surface state density | nfs | nfs | nfs |
| DELTA | Narrow channel effect | delta | delta | delta |
| XJ | Metallurgical junction depth | xj | xj | xj |
| UCRIT | Critical field for mobility degradation | ucrit | ucrit | ucrit |
| UEXP | Mobility field dependence exponent | uexp | uexp | uexp |
| VMAX | Maximum drift velocity | vmax | vmax | vmax |
| NEFF | Effective channel doping | neff | neff | neff |
| UTRA | Mobility transverse field coefficient | utra | utra | utra |

**Note:** Level 2 parameters are consistent across all simulators (unlike Level 1).

<a id="process-variation-parameters"></a>

### Process Variation and Delta Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| DEOT | Delta equivalent oxide thickness | deot | deot | deot |
| DTOXP | Delta oxide thickness for PMOS | dtoxp | dtoxp | dtoxp |
| DPHIG | Delta gate work function | dphig | dphig | dphig |
| DU0 | Delta mobility | du0 | du0 | du0 |
| DVSAT | Delta saturation velocity | dvsat | dvsat | dvsat |
| DCGSO | Delta gate-source overlap capacitance | dcgso | dcgso | dcgso |
| DCGDO | Delta gate-drain overlap capacitance | dcgdo | dcgdo | dcgdo |
| DCGSL | Delta gate-source overlap capacitance for short channel | dcgsl | dcgsl | dcgsl |
| DCGDL | Delta gate-drain overlap capacitance for short channel | dcgdl | dcgdl | dcgdl |
| DCJ | Delta bulk junction capacitance | dcj | dcj | dcj |
| DCJSW | Delta sidewall bulk junction capacitance | dcjsw | dcjsw | dcjsw |
| DCJSWG | Delta gate sidewall bulk junction capacitance | dcjswg | dcjswg | dcjswg |
| RN1 | Random number 1 for process variation | rn1 | rn1 | rn1 |
| RN2 | Random number 2 for process variation | rn2 | rn2 | rn2 |
| RP1 | Random number 1 for PMOS | rp1 | rp1 | rp1 |
| RP2 | Random number 2 for PMOS | rp2 | rp2 | rp2 |

<a id="level-3-mosfet-model"></a>

### Level 3 MOSFET Model (LEVEL=3)

*Note: Semi-empirical model with improved short-channel behavior*

**✅ Consistent Parameter Names Across Simulators:**

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| THETA | Mobility modulation coefficient | theta | theta | theta |
| KAPPA | Saturation field factor | kappa | kappa | kappa |
| ETA | Drain-induced barrier lowering | eta | eta | eta |

**Note:** Level 3 parameters are consistent across all simulators (unlike Level 1).

## Parameter Name Evolution

### Historical Development of SPICE Model Parameters

| Model Level | Parameter Consistency | Example Differences | Reason |
|-------------|----------------------|-------------------|---------|
| **Level 1** | ❌ **Inconsistent** | `VTO` vs `VTH0` vs `vth` | Historical divergence during 1980s commercialization |
| **Level 2** | ✅ **Consistent** | All parameters identical | Standardization effort |
| **Level 3** | ✅ **Consistent** | All parameters identical | Continued standardization |
| **BSIM1+** | ✅ **Consistent** | All parameters identical | Modern industry standard |

**Key Insight:** The parameter naming chaos in Level 1 reflects the "compatibility wars" of early SPICE commercialization. Later models (Level 2+, BSIM) represent standardization efforts to solve this problem.

---

<a id="bsim1-model"></a>

### BSIM1 Model (LEVEL=4, VERSION=1)

*Note: First Berkeley Short-channel IGFET Model - Industry-standard parameter names*

**✅ Consistent Parameter Names Across All Simulators:**

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VFB | Flat-band voltage | vfb | vfb | vfb |
| LVFB | Length dependence of VFB | lvfb | lvfb | lvfb |
| WVFB | Width dependence of VFB | wvfb | wvfb | wvfb |
| PHIB | Bulk Fermi potential | phib | phib | phib |
| K1 | First-order body effect coefficient | k1 | k1 | k1 |
| K2 | Second-order body effect coefficient | k2 | k2 | k2 |
| ETA | Drain-induced barrier lowering | eta | eta | eta |
| MUZ | Zero-field mobility | muz | muz | muz |
| DL | Channel length reduction | dl | dl | dl |
| DW | Channel width reduction | dw | dw | dw |
| U0 | Low-field mobility | u0 | u0 | u0 |
| U1 | Mobility temperature coefficient | u1 | u1 | u1 |
| X2MZ | Mobility temperature exponent | x2mz | x2mz | x2mz |
| MUS | Mobility scattering coefficient | mus | mus | mus |
| X2MS | Mobility scattering exponent | x2ms | x2ms | x2ms |
| X3MS | Mobility scattering exponent | x3ms | x3ms | x3ms |
| TOX | Gate oxide thickness | tox | tox | tox |
| TEMP | Temperature coefficient | temp | temp | temp |
| VDD | Supply voltage | vdd | vdd | vdd |
| ICG | Gate current coefficient | icg | icg | icg |
| ICB | Bulk current coefficient | icb | icb | icb |
| ICS | Source current coefficient | ics | ics | ics |
| ICD | Drain current coefficient | icd | icd | icd |
| CJ | Zero-bias bulk capacitance | cj | cj | cj |
| CJSW | Zero-bias sidewall capacitance | cjsw | cjsw | cjsw |
| MJ | Bulk grading coefficient | mj | mj | mj |
| MJSW | Sidewall grading coefficient | mjsw | mjsw | mjsw |
| PB | Bulk potential | pb | pb | pb |

<a id="bsim2-model"></a>

### BSIM2 Model (LEVEL=5, VERSION=2)

*Note: Enhanced BSIM1 model with improved physics*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTH0 | Zero-bias threshold voltage | vth0 | vth0 | vth0 |
| K1 | First-order body effect coefficient | k1 | k1 | k1 |
| K2 | Second-order body effect coefficient | k2 | k2 | k2 |
| ETA | Drain-induced barrier lowering | eta | eta | eta |
| ETA0 | DIBL coefficient | eta0 | eta0 | eta0 |
| ETAB | Body-bias dependence of eta | etab | etab | etab |
| MU0 | Low-field mobility | mu0 | mu0 | mu0 |
| MUS | Mobility scattering coefficient | mus | mus | mus |
| MU20 | Mobility model parameter | mu20 | mu20 | mu20 |
| MU2G | Gate-bias dependence of mobility | mu2g | mu2g | mu2g |
| MU2B | Body-bias dependence of mobility | mu2b | mu2b | mu2b |
| MU3 | Mobility model parameter | mu3 | mu3 | mu3 |
| MU3G | Gate-bias dependence of mu3 | mu3g | mu3g | mu3g |
| MU3B | Body-bias dependence of mu3 | mu3b | mu3b | mu3b |
| MU4 | Mobility model parameter | mu4 | mu4 | mu4 |
| MU4G | Gate-bias dependence of mu4 | mu4g | mu4g | mu4g |
| MU4B | Body-bias dependence of mu4 | mu4b | mu4b | mu4b |
| U0 | Low-field mobility | u0 | u0 | u0 |
| U1 | Mobility temperature coefficient | u1 | u1 | u1 |
| DL | Channel length reduction | dl | dl | dl |
| DW | Channel width reduction | dw | dw | dw |
| N0 | Zero-bias subthreshold swing | n0 | n0 | n0 |
| NB | Body-bias dependence of n0 | nb | nb | nb |
| ND | Drain-bias dependence of n0 | nd | nd | nd |
| N00 | Subthreshold swing coefficient | n00 | n00 | n00 |
| N0B | Body-bias dependence of n00 | n0b | n0b | n0b |
| N0D | Drain-bias dependence of n00 | n0d | n0d | n0d |
| PHI | Surface potential | phi | phi | phi |
| PHIB | Bulk Fermi potential | phib | phib | phib |
| GAMMA1 | First-order body effect coefficient | gamma1 | gamma1 | gamma1 |
| GAMMA2 | Second-order body effect coefficient | gamma2 | gamma2 | gamma2 |
| VFB | Flat-band voltage | vfb | vfb | vfb |
| LVFB | Length dependence of VFB | lvfb | lvfb | lvfb |
| WVFB | Width dependence of VFB | wvfb | wvfb | wvfb |
| VBM | Maximum body voltage | vbm | vbm | vbm |
| UTE | Mobility temperature exponent | ute | ute | ute |
| UC | Mobility temperature coefficient | uc | uc | uc |
| UD | Mobility temperature coefficient | ud | ud | ud |
| UP | Mobility temperature coefficient | up | up | up |
| US | Mobility temperature coefficient | us | us | us |
| UA | Mobility temperature coefficient | ua | ua | ua |
| UB | Mobility temperature coefficient | ub | ub | ub |
| VSAT | Saturation velocity | vsat | vsat | vsat |
| A0 | Bulk charge coefficient | a0 | a0 | a0 |
| AGS | Gate-bias dependence of a0 | ags | ags | ags |
| B0 | Bulk charge coefficient | b0 | b0 | b0 |
| B1 | Bulk charge coefficient | b1 | b1 | b1 |
| KETA | DIBL temperature coefficient | keta | keta | keta |
| A1 | Bulk charge coefficient | a1 | a1 | a1 |
| A2 | Bulk charge coefficient | a2 | a2 | a2 |
| RDSW | Source/drain resistance per width | rdsw | rdsw | rdsw |
| PRWG | Gate-bias dependence of RDSW | prwg | prwg | prwg |
| PRWB | Body-bias dependence of RDSW | prwb | prwb | prwb |
| WR | Width offset for RDSW | wr | wr | wr |
| DWG | Weff dependence of RDSW | dwg | dwg | dwg |
| DWB | Weff dependence of RDSW | dwb | dwb | dwb |
| PCLM | Channel length modulation parameter | pclm | pclm | pclm |
| PDIBL1 | DIBL effect correction parameter | pdibl1 | pdibl1 | pdibl1 |
| PDIBL2 | DIBL effect correction parameter | pdibl2 | pdibl2 | pdibl2 |
| PDIBLB | Body-bias dependence of DIBL | pdiblb | pdiblb | pdiblb |
| PSCBE1 | Substrate current body effect | pscbe1 | pscbe1 | pscbe1 |
| PSCBE2 | Substrate current body effect | pscbe2 | pscbe2 | pscbe2 |
| PVAG | Early voltage gate dependence | pvag | pvag | pvag |
| DELTA | Vds effect on threshold | delta | delta | delta |
| ALPHA0 | Substrate current parameter | alpha0 | alpha0 | alpha0 |
| BETA0 | Substrate current parameter | beta0 | beta0 | beta0 |
| CGSO | Gate-source overlap capacitance | cgso | cgso | cgso |
| CGDO | Gate-drain overlap capacitance | cgdo | cgdo | cgdo |
| CGBO | Gate-bulk overlap capacitance | cgbo | cgbo | cgbo |
| CGS | Gate-source capacitance | cgs | cgs | cgs |
| CGD | Gate-drain capacitance | cgd | cgd | cgd |
| CGB | Gate-bulk capacitance | cgb | cgb | cgb |
| CJ | Zero-bias bulk capacitance | cj | cj | cj |
| CJSW | Zero-bias sidewall capacitance | cjsw | cjsw | cjsw |
| MJ | Bulk grading coefficient | mj | mj | mj |
| MJSW | Sidewall grading coefficient | mjsw | mjsw | mjsw |
| PB | Bulk potential | pb | pb | pb |
| PBSW | Sidewall bulk potential | pbsw | pbsw | pbsw |
| PBSWG | Gate sidewall bulk potential | pbswg | pbswg | pbswg |
| TT | Transit time | tt | tt | tt |
| FC | Forward bias depletion coefficient | fc | fc | fc |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| TLEV | Temperature equation selector | tlev | tlev | tlev |
| TLEVC | Capacitance temperature selector | tlevc | tlevc | tlevc |
| TOX | Gate oxide thickness | tox | tox | tox |
| XPART | Charge partitioning flag | xpart | xpart | xpart |
| CGSL | Gate-source overlap for short channel | cgsl | cgsl | cgsl |
| CGDL | Gate-drain overlap for short channel | cgdl | cgdl | cgdl |
| CKAPPAS | S/D sidewall capacitance coefficient | ckappas | ckappas | ckappas |
| CKAPPAD | DIBL capacitance coefficient | ckappad | ckappad | ckappad |
| ACM | Area calculation method | acm | acm | acm |
| CALCACM | Area calculation method | calcacm | calcacm | calcacm |
| WLN | Length/width dependence flag | wln | wln | wln |
| WW | Width/width dependence flag | ww | ww | ww |
| WWL | Width/length dependence flag | wwl | wwl | wwl |
| LW | Length/width dependence flag | lw | lw | lw |
| LWL | Length/length dependence flag | lwl | lwl | lwl |
| CAPOP | Capacitance operation point | capop | capop | capop |
| CGSO | Gate-source overlap capacitance | cgso | cgso | cgso |
| CGDO | Gate-drain overlap capacitance | cgdo | cgdo | cgdo |
| CGBO | Gate-bulk overlap capacitance | cgbo | cgbo | cgbo |

<a id="bsim3v1-model"></a>

### BSIM3v1 Model (LEVEL=8, VERSION=3.1)

*Note: First version of BSIM3 - basic deep submicron modeling*

**Key Features:**
- Basic short-channel and narrow-width effects
- Improved subthreshold modeling
- Temperature dependencies

**Core Parameters:**
| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTH0 | Long-channel threshold voltage | vth0 | vth0 | vth |
| TOX | Gate oxide thickness | tox | tox | tox |
| U0 | Low-field mobility | u0 | u0 | u0 |
| VSAT | Saturation velocity | vsat | vsat | vsat |
| K1 | First-order body effect | k1 | k1 | k1 |
| K2 | Second-order body effect | k2 | k2 | k2 |
| DVT0 | Short-channel effect coefficient | dvt0 | dvt0 | dvt0 |
| DVT1 | Short-channel effect coefficient | dvt1 | dvt1 | dvt1 |
| UA | Mobility degradation | ua | ua | ua |
| UB | Mobility degradation | ub | ub | ub |
| PCLM | Channel length modulation | pclm | pclm | pclm |
| PDIBL1 | DIBL correction | pdibl1 | pdibl1 | pdibl1 |
| PSCBE1 | Substrate current | pscbe1 | pscbe1 | pscbe1 |
| DELTA | Vds effect on threshold | delta | delta | delta |
| ETA0 | DIBL coefficient | eta0 | eta0 | eta0 |
| CIT | Interface trap capacitance | cit | cit | cit |
| CDSC | Drain/source coupling | cdsc | cdsc | cdsc |
| NFACTOR | Subthreshold swing factor | nfactor | nfactor | nfactor |
| XJ | Junction depth | xj | xj | xj |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| CGSO | Gate-source overlap capacitance | cgso | cgso | cgso |
| CGDO | Gate-drain overlap capacitance | cgdo | cgdo | cgdo |
| CGBO | Gate-bulk overlap capacitance | cgbo | cgbo | cgbo |

<a id="bsim3v2-model"></a>

### BSIM3v2 Model (LEVEL=8, VERSION=3.2)

*Note: Enhanced BSIM3v1 with improved capacitance and noise modeling*

**Key Features (added to BSIM3v1):**
- Enhanced capacitance modeling (CAPMOD parameter)
- Improved noise modeling
- Better velocity saturation effects
- Non-quasi-static effects

**Additional Parameters (beyond BSIM3v1):**
| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| CAPMOD | Capacitance model selector | capmod | capmod | capmod |
| XPART | Charge partitioning flag | xpart | xpart | xpart |
| CGSL | Gate-source overlap for short channel | cgsl | cgsl | cgsl |
| CGDL | Gate-drain overlap for short channel | cgdl | cgdl | cgdl |
| CKAPPAS | Sidewall capacitance coefficient | ckappas | ckappas | ckappas |
| CKAPPAD | DIBL capacitance coefficient | ckappad | ckappad | ckappad |
| NOIA | Oxide trap density for flicker noise | noia | noia | noia |
| NOIB | Gate-induced flicker noise | noib | noib | noib |
| NOIC | Coupling flicker noise | noic | noic | noic |
| EM | Mobility fluctuation parameter | em | em | em |
| AFM | Flicker noise exponent | afm | afm | afm |
| KF | Flicker noise coefficient | kf | kf | kf |

<a id="bsim3v3-model"></a>

### BSIM3v3 Model (LEVEL=8, VERSION=3.3)

*Note: Industry standard MOSFET model for deep submicron technologies*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTH0 | Long-channel threshold voltage at Vbs=0 | vth0 | vth0 | vth |
| TOX | Gate oxide thickness | tox | tox | tox |
| U0 | Low-field surface mobility | u0 | u0 | u0 |
| VSAT | Saturation velocity | vsat | vsat | vsat |
| K1 | First-order body effect coefficient | k1 | k1 | k1 |
| K2 | Second-order body effect coefficient | k2 | k2 | k2 |
| K3 | Narrow width effect coefficient | k3 | k3 | k3 |
| K3B | Body effect coefficient of K3 | k3b | k3b | k3b |
| W0 | Narrow width parameter | w0 | w0 | w0 |
| NLX | Lateral non-uniform doping parameter | nlx | nlx | nlx |
| DVT0 | First coefficient of short-channel effect on Vth | dvt0 | dvt0 | dvt0 |
| DVT1 | Second coefficient of short-channel effect on Vth | dvt1 | dvt1 | dvt1 |
| DVT2 | Body-bias coefficient of short-channel effect on Vth | dvt2 | dvt2 | dvt2 |
| DVT0W | First coefficient of narrow width effect on Vth | dvt0w | dvt0w | dvt0w |
| DVT1W | Second coefficient of narrow width effect on Vth | dvt1w | dvt1w | dvt1w |
| DVT2W | Body-bias coefficient of narrow width effect on Vth | dvt2w | dvt2w | dvt2w |
| UTE | Mobility temperature exponent | ute | ute | ute |
| UA | First-order mobility degradation coefficient | ua | ua | ua |
| UB | Second-order mobility degradation coefficient | ub | ub | ub |
| UC | Body-effect of mobility degradation coefficient | uc | uc | uc |
| RDSW | Source/drain resistance per width | rdsw | rdsw | rdsw |
| PRWG | Gate-bias dependence of RDSW | prwg | prwg | prwg |
| PRWB | Body-bias dependence of RDSW | prwb | prwb | prwb |
| WR | Width offset from Weff for Rds calculation | wr | wr | wr |
| DWG | Coefficient of Weff dependence of RDSW | dwg | dwg | dwg |
| DWB | Coefficient of Weff dependence of RDSW | dwb | dwb | dwb |
| PCLM | Channel length modulation parameter | pclm | pclm | pclm |
| PCLMG | PCLM gate dependence | pclmg | pclmg | pclmg |
| CHARGEWF | Charge well factor | chargewf | chargewf | chargewf |
| PDIBL1 | First output resistance DIBL effect correction parameter | pdibl1 | pdibl1 | pdibl1 |
| PDIBL2 | Second output resistance DIBL effect correction parameter | pdibl2 | pdibl2 | pdibl2 |
| PDIBLB | Body-bias dependence of DIBL effect | pdiblb | pdiblb | pdiblb |
| PSCBE1 | First substrate current body effect parameter | pscbe1 | pscbe1 | pscbe1 |
| PSCBE2 | Second substrate current body effect parameter | pscbe2 | pscbe2 | pscbe2 |
| PVAG | Gate dependence of Early voltage | pvag | pvag | pvag |
| DELTA | Effect of Vds on threshold voltage | delta | delta | delta |
| ETA0 | Drain-induced barrier lowering coefficient | eta0 | eta0 | eta0 |
| ETAB | Body-bias dependence of eta0 | etab | etab | etab |
| DSUB | DIBL coefficient exponent in subthreshold region | dsub | dsub | dsub |
| CIT | Interface trap capacitance | cit | cit | cit |
| CDSC | Drain/source to channel coupling capacitance | cdsc | cdsc | cdsc |
| CDSCB | Body-bias dependence of CDSC | cdscb | cdscb | cdscb |
| CDSCD | Drain-bias dependence of CDSC | cdscd | cdscd | cdscd |
| NFACTOR | Subthreshold swing factor | nfactor | nfactor | nfactor |
| XJ | Source/drain junction depth | xj | xj | xj |
| VSATCV | Saturation velocity for C-V | vsatcv | vsatcv | vsatcv |
| MINV | Fitting parameter for moderate inversion | minv | minv | minv |
| MINVCV | Fitting parameter for moderate inversion in C-V | minvcv | minvcv | minvcv |
| NOFF | C-V parameter | noff | noff | noff |
| VOFF | C-V parameter | voff | voff | voff |
| VOFFCV | C-V parameter | voffcv | voffcv | voffcv |
| TNOM | Temperature at which parameters are extracted | tnom | tnom | tnom |
| CGSO | Gate-source overlap capacitance per unit width | cgso | cgso | cgso |
| CGDO | Gate-drain overlap capacitance per unit width | cgdo | cgdo | cgdo |
| CGBO | Gate-bulk overlap capacitance per unit length | cgbo | cgbo | cgbo |
| CAPMOD | Capacitance model selector | capmod | capmod | capmod |
| XPART | Charge partitioning flag | xpart | xpart | xpart |
| CGSL | Gate-source overlap capacitance for short channel | cgsl | cgsl | cgsl |
| CGDL | Gate-drain overlap capacitance for short channel | cgdl | cgdl | cgdl |
| CKAPPAS | S/D sidewall capacitance coefficient | ckappas | ckappas | ckappas |
| CKAPPAD | DIBL capacitance coefficient | ckappad | ckappad | ckappad |
| ACM | Area calculation method | acm | acm | acm |
| CALCACM | Area calculation method | calcacm | calcacm | calcacm |
| BULKMOD | Bulk charge effect model | bulkmod | bulkmod | bulkmod |
| VERSION | Model version number | version | version | version |
| RGATEMOD | Gate resistance model selector | rgateMod | rgateMod | rgateMod |
| PERMOD | Perimeter model selector | perMod | perMod | perMod |
| ACNQSMOD | AC NQS model selector | acnqsMod | acnqsMod | acnqsMod |
| TRNQSMOD | Transient NQS model selector | trnqsMod | trnqsMod | trnqsMod |
| DELVTRAND | Random threshold voltage variation | delvtrand | delvtrand | delvtrand |
| DTFIN | Fin thickness variation | dtfin | dtfin | dtfin |
| EFCI_GLOBAL | Effective current for global model | efci_global | efci_global | efci_global |
| DDELVTRAND | Delta threshold voltage variation | ddelvtrand | ddelvtrand | ddelvtrand |
| DELVTRAND_SIGN | Sign of threshold voltage variation | delvtrand_sign | delvtrand_sign | delvtrand_sign |
| DELVTRAND_SSQ | Sum of squares for threshold voltage variation | delvtrand_ssq | delvtrand_ssq | delvtrand_ssq |
| U0MULT | Mobility multiplier | u0mult | u0mult | u0mult |
| LINTNOI | Length integration for noise | lintnoi | lintnoi | lintnoi |
| LINTIGEN | Length integration for gate current | lintigen | lintigen | lintigen |
| DELTAPPSD | Delta for process variation | deltaprsd | deltaprsd | deltaprsd |
| IMIN | Minimum current for current calculations | imin | imin | imin |
| LPHIG | Fin height parameter | lphig | lphig | lphig |
| LRDSW | RDSW length dependence | lrdsw | lrdsw | lrdsw |
| ARDSW | RDSW area dependence | ardsw | ardsw | ardsw |
| BRDSW | RDSW bias dependence | brdsw | brdsw | brdsw |
| PRWG | Gate dependence of RDSW | prwg | prwg | prwg |
| PPDIBLC2 | DIBL parameter | ppdiblc2 | ppdiblc2 | ppdiblc2 |
| PDIBLCB | DIBL body-bias dependence | pdiblcb | pdiblcb | pdiblcb |
| IJTHSFWD | Forward junction thermal current | ijthsfwd | ijthsfwd | ijthsfwd |
| IJTHSREV | Reverse junction thermal current | ijthsrev | ijthsrev | ijthsrev |
| BVS | Substrate current induced body effect | bvs | bvs | bvs |
| XJBVS | BVS temperature coefficient | xjbvs | xjbvs | xjbvs |
| TNJTSSWG | Temperature coefficient for sidewall junction | tnjtsswg | tnjtsswg | tnjtsswg |
| VTSS | Source temperature coefficient | vtss | vtss | vtss |
| VTSD | Drain temperature coefficient | vtsd | vtsd | vtsd |
| VTSSWS | Sidewall temperature coefficient | vtssws | vtssws | vtssws |
| LKVTH0WE | Length dependence of VTH0 width effect | lkvth0we | lkvth0we | lkvth0we |
| WKVTH0WE | Width dependence of VTH0 width effect | wkvth0we | wkvth0we | wkvth0we |
| VFBSDOFF | Flat-band voltage for source/drain | vfbsdoff | vfbsdoff | vfbsdoff |
| EASUB | Electron affinity | easub | easub | easub |
| NI0SUB | Intrinsic carrier concentration | ni0sub | ni0sub | ni0sub |
| BG0SUB | Bandgap narrowing | bg0sub | bg0sub | bg0sub |
| NC0SUB | Conduction band density of states | nc0sub | nc0sub | nc0sub |
| PHIG | Gate work function | phig | phig | phig |
| RDSWMIN | Minimum source/drain resistance per width | rdswmin | rdswmin | rdswmin |
| CDSCD | Drain/source to channel coupling capacitance drain | cdscd | cdscd | cdscd |
| DEVTYPE | Device type (1=NMOS, 0=PMOS) | devtype | devtype | devtype |
| IGCMOD | Gate current model selector | igcmod | igcmod | igcmod |
| IGBMOD | Gate-bulk current model selector | igbmod | igbmod | igbmod |
| GIDLMOD | Gate-induced drain leakage model selector | gidbmod | gidbmod | gidbmod |
| COREMOD | Core model selector | coremod | coremod | coremod |
| GEOMOD | Geometry model selector | geomod | geomod | geomod |
| CGEOMOD | C-V geometry model selector | cgeomod | cgeomod | cgeomod |
| IIMOD | Impact ionization model selector | iimod | iimod | iimod |
| RDSMOD | Source/drain resistance model selector | rdsmod | rdsmod | rdsmod |
| XL | Length oxide encroachment | xl | xl | xl |
| LINT | Length integration | lint | lint | lint |
| LL | Layout length parameter | ll | ll | ll |
| LLC | Layout length capacitance | llc | llc | llc |
| LLN | Layout length noise | lln | lln | lln |
| DLC | Delta length for capacitance | dlc | dlc | dlc |
| EOT | Equivalent oxide thickness | eot | eot | eot |
| TFIN | Fin thickness | tfin | tfin | tfin |
| HFIN | Fin height | hfin | hfin | hfin |
| FPITCH | Fin pitch | fpitch | fpitch | fpitch |
| FECH | Fin etch parameter | fech | fech | fech |
| NF | Number of fins | nf | nf | nf |
| NFIN | Number of fins | nfin | nfin | nfin |
| NBODY | Body doping concentration | nbody | nbody | nbody |
| NSD | Source/drain doping concentration | nsd | nsd | nsd |
| NGATE | Gate doping concentration | ngate | ngate | ngate |
| IMIN | Minimum current for current calculations | imin | imin | imin |
| LPHIG | Fin height parameter | lphig | lphig | lphig |
| LRDSW | RDSW length dependence | lrdsw | lrdsw | lrdsw |
| ARDSW | RDSW area dependence | ardsw | ardsw | ardsw |
| BRDSW | RDSW bias dependence | brdsw | brdsw | brdsw |
| PRWG | Gate dependence of RDSW | prwg | prwg | prwg |
| CIT | Interface trap capacitance | cit | cit | cit |
| CDSC | Drain/source to channel coupling capacitance | cdsc | cdsc | cdsc |
| LCDSC | CDSC length dependence | lcdsc | lcdsc | lcdsc |
| LCDSCD | CDSCD length dependence | lcdscd | lcdscd | lcdscd |
| DVT0 | Short-channel effect coefficient | dvt0 | dvt0 | dvt0 |
| LDVT0 | DVT0 length dependence | ldvt0 | ldvt0 | ldvt0 |
| DVT1 | Short-channel effect coefficient | dvt1 | dvt1 | dvt1 |
| LDVT1 | DVT1 length dependence | ldvt1 | ldvt1 | ldvt1 |
| PHIN | PHI length dependence | phin | phin | phin |
| LPHIN | PHIN length dependence | lphin | lphin | lphin |
| ETA0 | DIBL coefficient | eta0 | eta0 | eta0 |
| LETA0 | ETA0 length dependence | leta0 | leta0 | leta0 |
| DSUB | DIBL coefficient exponent | dsub | dsub | dsub |
| LDSUB | DSUB length dependence | ldsub | ldsub | ldsub |
| K1RSCE | RSCE first coefficient | k1rsce | k1rsce | k1rsce |
| LK1RSCE | K1RSCE length dependence | lk1rsce | lk1rsce | lk1rsce |
| LPE0 | Zero-bias lateral field parameter | lpe0 | lpe0 | lpe0 |
| LLPE0 | LPE0 length dependence | llpe0 | llpe0 | llpe0 |
| QMFACTOR | Quantum mechanical factor | qmfactor | qmfactor | qmfactor |
| QM0 | Quantum mechanical parameter | qm0 | qm0 | qm0 |
| LVSAT | VSAT length dependence | lvsat | lvsat | lvsat |
| AVSAT | VSAT area dependence | avsat | avsat | avsat |
| BVSAT | VSAT bias dependence | bvsat | bvsat | bvsat |
| AVSAT1 | VSAT1 area dependence | avsat1 | avsat1 | avsat1 |
| BVSAT1 | VSAT1 bias dependence | bvsat1 | bvsat1 | bvsat1 |
| KSATIV | Saturation velocity coefficient | ksativ | ksativ | ksativ |
| LKSATIV | KSATIV length dependence | lksativ | lksativ | lksativ |
| DELTAVSAT | VSAT delta | deltavsat | deltavsat | deltavsat |
| MEXP | Mobility exponent | mexp | mexp | mexp |
| LMEXP | MEXP length dependence | lmexp | lmexp | lmexp |
| AMEXP | MEXP area dependence | amexp | amexp | amexp |
| BMEXP | MEXP bias dependence | bmexp | bmexp | bmexp |
| PTWG | PTWG parameter | ptwg | ptwg | ptwg |
| LPTWG | PTWG length dependence | lptwg | lptwg | lptwg |
| APTWG | PTWG area dependence | aptwg | aptwg | aptwg |
| BPTWG | PTWG bias dependence | bptwg | bptwg | bptwg |
| LU0 | U0 length dependence | lu0 | lu0 | lu0 |
| ETAMOB | Mobility parameter | etamob | etamob | etamob |
| UP | Mobility temperature coefficient | up | up | up |
| LUP | UP length dependence | lup | lup | lup |
| LPA | LPA parameter | lpa | lpa | lpa |
| LUA | UA length dependence | lua | lua | lua |
| AUA | UA area dependence | aua | aua | aua |
| BUA | UA bias dependence | bua | bua | bua |
| EU | Mobility parameter | eu | eu | eu |
| LEU | EU length dependence | leu | leu | leu |
| UD | Mobility parameter | ud | ud | ud |
| LUD | UD length dependence | lud | lud | lud |
| AUD | UD area dependence | aud | aud | aud |
| BUD | UD bias dependence | bud | bud | bud |
| UCS | Mobility parameter | ucs | ucs | ucs |
| LUCS | UCS length dependence | lucs | lucs | lucs |
| LPDIBL1 | PDIBL1 length dependence | lpdibl1 | lpdibl1 | lpdibl1 |
| LPDIBL2 | PDIBL2 length dependence | lpdibl2 | lpdibl2 | lpdibl2 |
| DROUT | DROUT parameter | drout | drout | drout |
| LDROUT | DROUT length dependence | ldrout | ldrout | ldrout |
| LPVAG | PVAG length dependence | lpvag | lpvag | lpvag |
| LPCLM | PCLM length dependence | lpclm | lpclm | lpclm |
| CFS | Capacitance parameter | cfs | cfs | cfs |
| DVTP0 | DVTP0 parameter | dvtp0 | dvtp0 | dvtp0 |
| DVTP1 | DVTP1 parameter | dvtp1 | dvtp1 | dvtp1 |
| DELTAWCV | Delta width for C-V | deltawcv | deltawcv | deltawcv |
| QMTCENCV | Quantum mechanical C-V parameter | qmtcencv | qmtcencv | qmtcencv |
| CGSL | Gate-source overlap for short channel | cgsl | cgsl | cgsl |
| CGDL | Gate-drain overlap for short channel | cgdl | cgdl | cgdl |

<a id="advanced-bsim-parameters"></a>

### Advanced BSIM Parameters

*Note: Additional quantum mechanical and layout-dependent parameters for advanced MOSFET modeling*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| QMFACTOR | Quantum mechanical factor | qmfactor | qmfactor | qmfactor |
| QM0 | Quantum mechanical parameter | qm0 | qm0 | qm0 |
| QMTCENCV | Quantum mechanical C-V parameter | qmtcencv | qmtcencv | qmtcencv |
| DELTAWCV | Delta width for C-V | deltawcv | deltawcv | deltawcv |
| LCDSC | CDSC length dependence | lcdsc | lcdsc | lcdsc |
| LCDSCD | CDSCD length dependence | lcdscd | lcdscd | lcdscd |
| LDVT0 | DVT0 length dependence | ldvt0 | ldvt0 | ldvt0 |
| LDVT1 | DVT1 length dependence | ldvt1 | ldvt1 | ldvt1 |
| LETA0 | ETA0 length dependence | leta0 | leta0 | leta0 |
| LDSUB | DSUB length dependence | ldsub | ldsub | ldsub |
| LPE0 | LPE0 length dependence | lpe0 | lpe0 | lpe0 |
| LLPE0 | LPE0 length dependence | llpe0 | llpe0 | llpe0 |
| LMEXP | MEXP length dependence | lmexp | lmexp | lmexp |
| AMEXP | MEXP area dependence | amexp | amexp | amexp |
| BMEXP | MEXP bias dependence | bmexp | bmexp | bmexp |
| LPTWG | PTWG length dependence | lptwg | lptwg | lptwg |
| APTWG | PTWG area dependence | aptwg | aptwg | aptwg |
| BPTWG | PTWG bias dependence | bptwg | bptwg | bptwg |
| LU0 | U0 length dependence | lu0 | lu0 | lu0 |
| ETAMOB | Mobility parameter | etamob | etamob | etamob |
| LUP | UP length dependence | lup | lup | lup |
| LPA | LPA parameter | lpa | lpa | lpa |
| LUA | UA length dependence | lua | lua | lua |
| AUA | UA area dependence | aua | aua | aua |
| BUA | UA bias dependence | bua | bua | bua |
| LEU | EU length dependence | leu | leu | leu |
| UD | Mobility parameter | ud | ud | ud |
| LUD | UD length dependence | lud | lud | lud |
| AUD | UD area dependence | aud | aud | aud |
| BUD | UD bias dependence | bud | bud | bud |
| UCS | Mobility parameter | ucs | ucs | ucs |
| LUCS | UCS length dependence | lucs | lucs | lucs |
| LPDIBL1 | PDIBL1 length dependence | lpdibl1 | lpdibl1 | lpdibl1 |
| LPDIBL2 | PDIBL2 length dependence | lpdibl2 | lpdibl2 | lpdibl2 |
| LDROUT | DROUT length dependence | ldrout | ldrout | ldrout |
| LPVAG | PVAG length dependence | lpvag | lpvag | lpvag |
| LPCLM | PCLM length dependence | lpclm | lpclm | lpclm |
| CFS | Capacitance parameter | cfs | cfs | cfs |
| DVTP0 | DVTP0 parameter | dvtp0 | dvtp0 | dvtp0 |
| DVTP1 | DVTP1 parameter | dvtp1 | dvtp1 | dvtp1 |
| CGSL | Gate-source overlap for short channel | cgsl | cgsl | cgsl |
| CGDL | Gate-drain overlap for short channel | cgdl | cgdl | cgdl |

<a id="bsim4-model"></a>

### BSIM4 Model (LEVEL=14, VERSION=4)

*Note: Advanced MOSFET model for nanometer technologies*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTH0 | Long-channel threshold voltage | vth0 | vth0 | vth |
| TOX | Gate oxide thickness | tox | tox | tox |
| U0 | Low-field mobility | u0 | u0 | u0 |
| VSAT | Saturation velocity | vsat | vsat | vsat |
| K1 | First-order body effect coefficient | k1 | k1 | k1 |
| K2 | Second-order body effect coefficient | k2 | k2 | k2 |
| K3 | Narrow width effect coefficient | k3 | k3 | k3 |
| W0 | Narrow width parameter | w0 | w0 | w0 |
| LPE0 | Zero-bias lateral field parameter | lpe0 | lpe0 | lpe0 |
| LPEB | Body-bias dependence of LPE0 | lpeb | lpeb | lpeb |
| DVT0 | Short-channel effect coefficient | dvt0 | dvt0 | dvt0 |
| DVT1 | Short-channel effect coefficient | dvt1 | dvt1 | dvt1 |
| DVT2 | Short-channel effect coefficient | dvt2 | dvt2 | dvt2 |
| DVT0W | Narrow width effect coefficient | dvt0w | dvt0w | dvt0w |
| DVT1W | Narrow width effect coefficient | dvt1w | dvt1w | dvt1w |
| DVT2W | Narrow width effect coefficient | dvt2w | dvt2w | dvt2w |
| UTE | Mobility temperature exponent | ute | ute | ute |
| UA | Mobility degradation coefficient | ua | ua | ua |
| UB | Mobility degradation coefficient | ub | ub | ub |
| UC | Mobility degradation coefficient | uc | uc | uc |
| RDSW | Source/drain resistance per width | rdsw | rdsw | rdsw |
| PRWG | Gate dependence of RDSW | prwg | prwg | prwg |
| PRWB | Body dependence of RDSW | prwb | prwb | prwb |
| WR | Width offset for RDSW | wr | wr | wr |
| PCLM | Channel length modulation parameter | pclm | pclm | pclm |
| PDIBL1 | DIBL effect correction parameter | pdibl1 | pdibl1 | pdibl1 |
| PDIBL2 | DIBL effect correction parameter | pdibl2 | pdibl2 | pdibl2 |
| PDIBLB | Body-bias dependence of DIBL | pdiblb | pdiblb | pdiblb |
| PSCBE1 | Substrate current body effect | pscbe1 | pscbe1 | pscbe1 |
| PSCBE2 | Substrate current body effect | pscbe2 | pscbe2 | pscbe2 |
| PVAG | Early voltage gate dependence | pvag | pvag | pvag |
| DELTA | Vds effect on threshold | delta | delta | delta |
| ETA0 | DIBL coefficient | eta0 | eta0 | eta0 |
| ETAB | Body-bias dependence of eta0 | etab | etab | etab |
| DSUB | DIBL coefficient exponent | dsub | dsub | dsub |
| CIT | Interface trap capacitance | cit | cit | cit |
| CDSC | Drain/source coupling capacitance | cdsc | cdsc | cdsc |
| CDSCB | Body-bias dependence of CDSC | cdscb | cdscb | cdscb |
| CDSCD | Drain-bias dependence of CDSC | cdscd | cdscd | cdscd |
| NFACTOR | Subthreshold swing factor | nfactor | nfactor | nfactor |
| XJ | Source/drain junction depth | xj | xj | xj |
| VSATCV | Saturation velocity for C-V | vsatcv | vsatcv | vsatcv |
| MINV | Moderate inversion parameter | minv | minv | minv |
| MINVCV | Moderate inversion for C-V | minvcv | minvcv | minvcv |
| NOFF | C-V parameter | noff | noff | noff |
| VOFF | C-V parameter | voff | voff | voff |
| VOFFCV | C-V parameter | voffcv | voffcv | voffcv |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| CGSO | Gate-source overlap capacitance | cgso | cgso | cgso |
| CGDO | Gate-drain overlap capacitance | cgdo | cgdo | cgdo |
| CGBO | Gate-bulk overlap capacitance | cgbo | cgbo | cgbo |
| CAPMOD | Capacitance model selector | capmod | capmod | capmod |
| XPART | Charge partitioning flag | xpart | xpart | xpart |
| CGSL | Gate-source overlap for short channel | cgsl | cgsl | cgsl |
| CGDL | Gate-drain overlap for short channel | cgdl | cgdl | cgdl |
| CKAPPAS | S/D sidewall capacitance coefficient | ckappas | ckappas | ckappas |
| CKAPPAD | DIBL capacitance coefficient | ckappad | ckappad | ckappad |
| ACM | Area calculation method | acm | acm | acm |
| BULKMOD | Bulk charge effect model | bulkmod | bulkmod | bulkmod |
| VERSION | Model version number | version | version | version |
| KT1 | Temperature coefficient for threshold voltage | kt1 | kt1 | kt1 |
| KT2 | Body-bias coefficient of threshold voltage temperature effect | kt2 | kt2 | kt2 |
| UA1 | Temperature coefficient for UA | ua1 | ua1 | ua1 |
| UB1 | Temperature coefficient for UB | ub1 | ub1 | ub1 |
| UC1 | Temperature coefficient for UC | uc1 | uc1 | uc1 |
| RDSW | Source/drain resistance per width | rdsw | rdsw | rdsw |
| PRWG | Gate-bias dependence of RDSW | prwg | prwg | prwg |
| PRWB | Body-bias dependence of RDSW | prwb | prwb | prwb |
| WR | Width offset from Weff for Rds calculation | wr | wr | wr |
| DWG | Coefficient of Weff dependence of RDSW | dwg | dwg | dwg |
| DWB | Coefficient of Weff dependence of RDSW | dwb | dwb | dwb |
| TOXG | Gate oxide thickness for gate current | toxg | toxg | toxg |
| XL | Length oxide encroachment | xl | xl | xl |
| AVSAT | Saturation velocity area dependence | avsat | avsat | avsat |
| VSAT1R | Saturation velocity 1 reverse | vsat1r | vsat1r | vsat1r |
| AVSATCV | Saturation velocity for C-V area dependence | avsatcv | avsatcv | avsatcv |
| VSATCV | Saturation velocity for C-V | vsatcv | vsatcv | vsatcv |
| UTL | Mobility temperature coefficient for low field | utl | utl | utl |
| LSP | Source/drain diffusion length | lsp | lsp | lsp |
| AIGC | Gate current coefficient | aigc | aigc | aigc |
| AIGS | Gate-source current coefficient | aigs | aigs | aigs |
| AIGD | Gate-drain current coefficient | aigd | aigd | aigd |
| CGS | Gate-source capacitance | cgs | cgs | cgs |
| CGD | Gate-drain capacitance | cgd | cgd | cgd |
| CGB | Gate-bulk capacitance | cgb | cgb | cgb |
| CJSWGS | Sidewall gate-source capacitance | cjswgs | cjswgs | cjswgs |
| CJSWGD | Sidewall gate-drain capacitance | cjswgd | cjswgd | cjswgd |
| DELVTRAND | Random threshold voltage variation | delvtrand | delvtrand | delvtrand |
| U0MULT | Mobility multiplier | u0mult | u0mult | u0mult |
| EFCI_GLOBAL | Effective current for global model | efci_global | efci_global | efci_global |
| DDELVTRAND | Delta threshold voltage variation | ddelvtrand | ddelvtrand | ddelvtrand |
| DTFIN | Fin thickness variation | dtfin | dtfin | dtfin |
| DELVTRAND_SIGN | Sign of threshold voltage variation | delvtrand_sign | delvtrand_sign | delvtrand_sign |
| DELVTRAND_SSQ | Sum of squares for threshold voltage variation | delvtrand_ssq | delvtrand_ssq | delvtrand_ssq |

<a id="soi-mosfet-models"></a>

### SOI MOSFET Models

*Note: Silicon-on-Insulator MOSFET models (various levels)*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTHSOI | SOI threshold voltage | vthsoi | vthsoi | vthsoi |
| TOXSOI | SOI oxide thickness | toxsoi | toxsoi | toxsoi |
| TBOX | Buried oxide thickness | tbox | tbox | tbox |
| ALPHA | Body factor | alpha | alpha | alpha |
| BETA | Body effect coefficient | beta | beta | beta |
| GAMMA | Body effect coefficient | gamma | gamma | gamma |
| K1 | First-order body effect coefficient | k1 | k1 | k1 |
| K2 | Second-order body effect coefficient | k2 | k2 | k2 |
| ETA | DIBL coefficient | eta | eta | eta |
| DELTA | Vds effect on threshold | delta | delta | delta |
| VSAT | Saturation velocity | vsat | vsat | vsat |
| U0 | Low-field mobility | u0 | u0 | u0 |

<a id="finfet-models"></a>

### FinFET Models

*Note: 3D FinFET transistor models (various implementations)*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| TFIN | Fin thickness | tfin | tfin | tfin |
| HFIN | Fin height | hfin | hfin | hfin |
| NF | Number of fins | nf | nf | nf |
| NFIN | Number of fins | nfin | nfin | nfin |
| FPITCH | Fin pitch | fpitch | fpitch | fpitch |
| FECH | Fin etch parameter | fech | fech | fech |
| NBODY | Body doping | nbody | nbody | nbody |
| NSD | Source/drain doping | nsd | nsd | nsd |
| NGATE | Gate doping | ngate | ngate | ngate |
| LPHIG | Fin height parameter | lphig | lphig | lphig |
| LRDSW | RDSW length dependence | lrdsw | lrdsw | lrdsw |
| ARDSW | RDSW area dependence | ardsw | ardsw | ardsw |
| BRDSW | RDSW bias dependence | brdsw | brdsw | brdsw |
| PRWG | Gate dependence of RDSW | prwg | prwg | prwg |
| CIT | Interface trap capacitance | cit | cit | cit |
| LCDSC | CDSC length dependence | lcdsc | lcdsc | lcdsc |
| LCDSCD | CDSCD length dependence | lcdscd | lcdscd | lcdscd |
| DVT0 | Short-channel effect coefficient | dvt0 | dvt0 | dvt0 |
| LDVT0 | DVT0 length dependence | ldvt0 | ldvt0 | ldvt0 |
| DVT1 | Short-channel effect coefficient | dvt1 | dvt1 | dvt1 |
| LDVT1 | DVT1 length dependence | ldvt1 | ldvt1 | ldvt1 |
| PHIN | PHI length dependence | phin | phin | phin |
| LPHIN | PHIN length dependence | lphin | lphin | lphin |
| LETA0 | eta0 length dependence | leta0 | leta0 | leta0 |
| DSUB | DIBL coefficient exponent | dsub | dsub | dsub |
| LDSUB | DSUB length dependence | ldsub | ldsub | ldsub |
| K1RSCE | RSCE first coefficient | k1rsce | k1rsce | k1rsce |
| LK1RSCE | K1RSCE length dependence | lk1rsce | lk1rsce | lk1rsce |
| LPE0 | Zero-bias lateral field parameter | lpe0 | lpe0 | lpe0 |
| LLPE0 | LPE0 length dependence | llpe0 | llpe0 | llpe0 |
| QMFACTOR | Quantum mechanical factor | qmfactor | qmfactor | qmfactor |
| QM0 | Quantum mechanical parameter | qm0 | qm0 | qm0 |
| LVSAT | VSAT length dependence | lvsat | lvsat | lvsat |
| AVSAT | VSAT area dependence | avsat | avsat | avsat |
| BVSAT | VSAT bias dependence | bvsat | bvsat | bvsat |
| AVSAT1 | VSAT1 area dependence | avsat1 | avsat1 | avsat1 |
| BVSAT1 | VSAT1 bias dependence | bvsat1 | bvsat1 | bvsat1 |
| KSATIV | Saturation velocity coefficient | ksativ | ksativ | ksativ |
| LKSATIV | KSATIV length dependence | lksativ | lksativ | lksativ |
| DELTAVSAT | VSAT delta | deltavsat | deltavsat | deltavsat |
| MEXP | Mobility exponent | mexp | mexp | mexp |
| LMEXP | MEXP length dependence | lmexp | lmexp | lmexp |
| AMEXP | MEXP area dependence | amexp | amexp | amexp |
| BMEXP | MEXP bias dependence | bmexp | bmexp | bmexp |
| PTWG | PTWG parameter | ptwg | ptwg | ptwg |
| LPTWG | PTWG length dependence | lptwg | lptwg | lptwg |
| APTWG | PTWG area dependence | aptwg | aptwg | aptwg |
| BPTWG | PTWG bias dependence | bptwg | bptwg | bptwg |
| LU0 | U0 length dependence | lu0 | lu0 | lu0 |
| ETAMOB | Mobility parameter | etamob | etamob | etamob |
| LUP | UP length dependence | lup | lup | lup |
| LPA | LPA parameter | lpa | lpa | lpa |
| LUA | UA length dependence | lua | lua | lua |
| AUA | UA area dependence | aua | aua | aua |
| BUA | UA bias dependence | bua | bua | bua |
| EU | Mobility parameter | eu | eu | eu |
| LEU | EU length dependence | leu | leu | leu |
| UD | Mobility parameter | ud | ud | ud |
| LUD | UD length dependence | lud | lud | lud |
| AUD | UD area dependence | aud | aud | aud |
| BUD | UD bias dependence | bud | bud | bud |
| UCS | Mobility parameter | ucs | ucs | ucs |
| LUCS | UCS length dependence | lucs | lucs | lucs |
| LPDIBL1 | PDIBL1 length dependence | lpdibl1 | lpdibl1 | lpdibl1 |
| LPDIBL2 | PDIBL2 length dependence | lpdibl2 | lpdibl2 | lpdibl2 |
| DROUT | DROUT parameter | drout | drout | drout |
| LDROUT | DROUT length dependence | ldrout | ldrout | ldrout |
| LPVAG | PVAG length dependence | lpvag | lpvag | lpvag |
| LPCLM | PCLM length dependence | lpclm | lpclm | lpclm |
| CFS | Capacitance parameter | cfs | cfs | cfs |
| DVTP0 | DVTP0 parameter | dvtp0 | dvtp0 | dvtp0 |
| DVTP1 | DVTP1 parameter | dvtp1 | dvtp1 | dvtp1 |
| CKAPPAS | S/D sidewall capacitance coefficient | ckappas | ckappas | ckappas |
| CKAPPAD | DIBL capacitance coefficient | ckappad | ckappad | ckappad |
| QMTCECV | Quantum mechanical C-V parameter | qmtcencv | qmtcencv | qmtcencv |
| EASUB | Electron affinity | easub | easub | easub |
| NI0SUB | Intrinsic carrier concentration | ni0sub | ni0sub | ni0sub |
| BG0SUB | Bandgap narrowing | bg0sub | bg0sub | bg0sub |
| NC0SUB | Conduction band density of states | nc0sub | nc0sub | nc0sub |
| PHIG | Gate work function | phig | phig | phig |
| CDSCD | Drain/source to channel coupling capacitance | cdscd | cdscd | cdscd |
| UA | First-order mobility degradation coefficient | ua | ua | ua |
| PDIBL2 | Second output resistance DIBL effect correction | pdibl2 | pdibl2 | pdibl2 |
| PCLM | Channel length modulation parameter | pclm | pclm | pclm |
| PCLMG | PCLM gate dependence | pclmg | pclmg | pclmg |
| DEVTYPE | Device type (1=NMOS, 0=PMOS) | devtype | devtype | devtype |
| IGCMOD | Gate current model selector | igcmod | igcmod | igcmod |
| IGBMOD | Gate-bulk current model selector | igbmod | igbmod | igbmod |
| GIDLMOD | Gate-induced drain leakage model selector | gidbmod | gidbmod | gidbmod |
| COREMOD | Core model selector | coremod | coremod | coremod |
| GEOMOD | Geometry model selector | geomod | geomod | geomod |
| CGEOMOD | C-V geometry model selector | cgeomod | cgeomod | cgeomod |
| IIMOD | Impact ionization model selector | iimod | iimod | iimod |
| RDSMOD | Source/drain resistance model selector | rdsmod | rdsmod | rdsmod |
| LINT | Length integration | lint | lint | lint |
| LL | Layout length parameter | ll | ll | ll |
| LLC | Layout length capacitance | llc | llc | llc |
| LLN | Layout length noise | lln | lln | lln |
| DLC | Delta length for capacitance | dlc | dlc | dlc |
| EOT | Equivalent oxide thickness | eot | eot | eot |
| TFIN | Fin thickness | tfin | tfin | tfin |
| HFIN | Fin height | hfin | hfin | hfin |
| FPITCH | Fin pitch | fpitch | fpitch | fpitch |
| FECH | Fin etch parameter | fech | fech | fech |
| NF | Number of fins | nf | nf | nf |
| NFIN | Number of fins | nfin | nfin | nfin |
| NBODY | Body doping concentration | nbody | nbody | nbody |
| NSD | Source/drain doping concentration | nsd | nsd | nsd |
| NGATE | Gate doping concentration | ngate | ngate | ngate |
| IMIN | Minimum current for current calculations | imin | imin | imin |
| LPHIG | Fin height parameter | lphig | lphig | lphig |
| LRDSW | RDSW length dependence | lrdsw | lrdsw | lrdsw |
| ARDSW | RDSW area dependence | ardsw | ardsw | ardsw |
| BRDSW | RDSW bias dependence | brdsw | brdsw | brdsw |
| PRWG | Gate dependence of RDSW | prwg | prwg | prwg |

<a id="bsim41-model"></a>

### BSIM4.1 Model (LEVEL=14, VERSION=4.1)

*Note: BSIM4.1 adds mechanical stress effects and improved layout dependencies*

**Key Features (added to BSIM4.0):**
- Mechanical stress modeling (STI stress effects)
- Enhanced layout-dependent parameter extraction
- Improved statistical modeling capabilities

**Additional Parameters:**
| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| SA | Stress effect amplitude | sa | sa | sa |
| SB | Stress effect bias dependence | sb | sb | sb |
| SD | Stress effect drain bias | sd | sd | sd |
| SCA | Stress concentration at active region | sca | sca | sca |
| SCB | Stress concentration at body | scb | scb | scb |
| SCC | Stress concentration at channel | scc | scc | scc |
| SC | Stress coefficient | sc | sc | sc |

<a id="bsim42-model"></a>

### BSIM4.2 Model (LEVEL=14, VERSION=4.2)

*Note: BSIM4.2 enhances stress modeling and adds well proximity effects*

**Key Features (added to BSIM4.1):**
- Advanced mechanical stress modeling
- Well proximity effect (WPE) modeling
- Improved halo/pocket implant effects

**Additional Parameters:**
| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| WEB | Well edge bias | web | web | web |
| WEC | Well edge current | wec | wec | wec |
| KVTH0WE | Well edge threshold voltage | kvth0we | kvth0we | kvth0we |
| K2WE | Well edge body effect | k2we | k2we | k2we |
| KU0WE | Well edge mobility | ku0we | ku0we | ku0we |
| PVAGWE | Well edge early voltage | pvagwe | pvagwe | pvagwe |
| SAREF | Reference distance for stress | saref | saref | saref |
| SBREF | Reference bias for stress | sbref | sbref | sbref |
| WLOD | Width dependence of length offset | wlod | wlod | wlod |
| KU0LD | Length dependence of KU0WE | ku0ld | ku0ld | ku0ld |

<a id="bsim43-model"></a>

### BSIM4.3 Model (LEVEL=14, VERSION=4.3)

*Note: BSIM4.3 adds comprehensive layout-dependent effects (LOD)*

**Key Features (added to BSIM4.2):**
- Length of diffusion (LOD) effects
- Advanced parasitic resistance modeling
- Improved source/drain engineering effects

**Additional Parameters:**
| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| LLODKU0 | LOD effect on KU0 | llodku0 | llodku0 | llodku0 |
| WLODKU0 | Width LOD effect on KU0 | wlodku0 | wlodku0 | wlodku0 |
| LLODVTH | LOD effect on VTH | llodvth | llodvth | llodvth |
| WLODVTH | Width LOD effect on VTH | wlodvth | wlodvth | wlodvth |
| LLOD | Length of diffusion | llod | llod | llod |
| WLOD | Width of diffusion | wlod | wlod | wlod |
| LODK2 | LOD effect on K2 | lodk2 | lodk2 | lodk2 |
| LODETA0 | LOD effect on ETA0 | lodeta0 | lodeta0 | lodeta0 |
| LODSUB | LOD effect on DSUB | lodsub | lodsub | lodsub |

<a id="bsim44-model"></a>

### BSIM4.4 Model (LEVEL=14, VERSION=4.4)

*Note: BSIM4.4 enhances noise and RF modeling capabilities*

**Key Features (added to BSIM4.3):**
- Advanced flicker noise modeling
- Improved thermal noise calculation
- RF substrate coupling effects
- Enhanced gate resistance modeling

**Additional Parameters:**
| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| NTNOI | Thermal noise parameter | ntnoi | ntnoi | ntnoi |
| RNOIA | Noise parameter A | rnoia | rnoia | rnoia |
| RNOIB | Noise parameter B | rnoib | rnoib | rnoib |
| RNOIC | Noise parameter C | rnoic | rnoic | rnoic |
| FNOIMOD | Flicker noise model selector | fnoimod | fnoimod | fnoimod |
| TNOIMOD | Thermal noise model selector | tnoimod | tnoimod | tnoimod |
| GIDLSTRONG | Strong GIDL parameter | gidlstrong | gidlstrong | gidlstrong |
| IGIDLMOD | GIDL model selector | igidlmod | igidlmod | igidlmod |

<a id="bsim45-model"></a>

### BSIM4.5 Model (LEVEL=14, VERSION=4.5)

*Note: BSIM4.5 adds comprehensive gate current and tunneling models*

**Key Features (added to BSIM4.4):**
- Advanced gate leakage modeling
- Quantum mechanical tunneling effects
- Improved poly-Si gate depletion effects
- Enhanced high-frequency modeling

**Additional Parameters:**
| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| AIGBINV | Gate current parameter | aigbinv | aigbinv | aigbinv |
| BIGBINV | Gate current parameter | bigbinv | bigbinv | bigbinv |
| CIGBINV | Gate current parameter | cigbinv | cigbinv | cigbinv |
| NIGBINV | Gate current parameter | nigbinv | nigbinv | nigbinv |
| AIGBACC | Gate current parameter | aigbacc | aigbacc | aigbacc |
| BIGBACC | Gate current parameter | bigbacc | bigbacc | bigbacc |
| CIGBACC | Gate current parameter | cigbacc | cigbacc | cigbacc |
| NIGBACC | Gate current parameter | nigbacc | nigbacc | nigbacc |
| AIGC | Gate current parameter | aigc | aigc | aigc |
| BIGC | Gate current parameter | bigc | bigc | bigc |
| CIGC | Gate current parameter | cigc | cigc | cigc |

<a id="bsim46-model"></a>

### BSIM4.6 Model (LEVEL=14, VERSION=4.6)

*Note: BSIM4.6 adds RF and microwave modeling capabilities*

**Key Features (added to BSIM4.5):**
- Enhanced RF substrate modeling
- Improved S-parameter extraction
- Advanced harmonic balance analysis support
- Better high-frequency noise modeling

**Additional Parameters:**
| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| RGATEMOD | Gate resistance model selector | rgatemod | rgatemod | rgatemod |
| RBodymod | Body resistance model selector | rbodymod | rbodymod | rbodymod |
| RGEOMOD | Gate geometry model selector | rgeomod | rgeomod | rgeomod |
| XGW | Gate width parameter | xgw | xgw | xgw |
| XGL | Gate length parameter | xgl | xgl | xgl |
| NSEG | Number of segments | nseg | nseg | nseg |
| RDSMOD | Source/drain resistance model | rdsmod | rdsmod | rdsmod |
| HDIF | Diffusion height | hdif | hdif | hdif |
| LDIF | Diffusion length | ldif | ldif | ldif |

<a id="bsim47-model"></a>

### BSIM4.7 Model (LEVEL=14, VERSION=4.7)

*Note: BSIM4.7 adds multi-gate transistor support (FinFET, Tri-gate)*

**Key Features (added to BSIM4.6):**
- FinFET transistor modeling
- 3D geometry effects
- Quantum confinement effects
- Multi-fin device characteristics

**Additional Parameters:**
| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| TFIN | Fin thickness | tfin | tfin | tfin |
| HFIN | Fin height | hfin | hfin | hfin |
| NF | Number of fins | nf | nf | nf |
| NFIN | Number of fins | nfin | nfin | nfin |
| FPITCH | Fin pitch | fpitch | fpitch | fpitch |
| FECH | Fin etch parameter | fech | fech | fech |
| LFIN | Fin length | lfin | lfin | lfin |
| NBODY | Body doping concentration | nbody | nbody | nbody |
| NSD | Source/drain doping | nsd | nsd | nsd |
| NGATE | Gate doping | ngate | ngate | ngate |

<a id="bsim48-model"></a>

### BSIM4.8 Model (LEVEL=14, VERSION=4.8)

*Note: BSIM4.8 - Most advanced BSIM4 with comprehensive modeling capabilities*

**Key Features (added to BSIM4.7):**
- Complete quantum mechanical modeling
- Advanced statistical analysis
- Full 3D stress modeling
- Comprehensive noise and reliability modeling
- Process variation aware modeling

**Additional Parameters:**
| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| QMFACTOR | Quantum mechanical factor | qmfactor | qmfactor | qmfactor |
| QM0 | Quantum mechanical parameter | qm0 | qm0 | qm0 |
| QMTCENCV | QM C-V parameter | qmtcencv | qmtcencv | qmtcencv |
| DELTAWCV | Delta width for C-V | deltawcv | deltawcv | deltawcv |
| SIGMA | Statistical parameter | sigma | sigma | sigma |
| SIGMA0 | Statistical parameter | sigma0 | sigma0 | sigma0 |
| DELVTRAND | Threshold variation | delvtrand | delvtrand | delvtrand |
| DELVTRAND_SSQ | Sum of squares variation | delvtrand_ssq | delvtrand_ssq | delvtrand_ssq |
| EFCI_GLOBAL | Effective current | efci_global | efci_global | efci_global |
| DDELVTRAND | Delta threshold variation | ddelvtrand | ddelvtrand | ddelvtrand |
| MISMATCH | Mismatch model selector | mismatch | mismatch | mismatch |
| MISMATCH0 | Mismatch parameter | mismatch0 | mismatch0 | mismatch0 |

<a id="high-voltage-mosfet-models"></a>

### High Voltage MOSFET Models

*Note: High voltage MOSFET models (various levels)*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTHHV | High voltage threshold | vthhv | vthhv | vthhv |
| TOXHV | High voltage oxide thickness | toxhv | toxhv | toxhv |
| U0HV | High voltage mobility | u0hv | u0hv | u0hv |
| VSATHV | High voltage saturation velocity | vsathv | vsathv | vsathv |
| K1HV | High voltage body effect | k1hv | k1hv | k1hv |
| K2HV | High voltage body effect | k2hv | k2hv | k2hv |
| K3HV | High voltage narrow width | k3hv | k3hv | k3hv |
| W0HV | High voltage width offset | w0hv | w0hv | w0hv |
| NLXHV | High voltage doping | nlxhv | nlxhv | nlxhv |
| DVT0HV | High voltage short channel | dvt0hv | dvt0hv | dvt0hv |
| DVT1HV | High voltage short channel | dvt1hv | dvt1hv | dvt1hv |
| DVT2HV | High voltage body bias | dvt2hv | dvt2hv | dvt2hv |

## Other Transistor Models

<a id="jfet-model"></a>

### JFET Model

*Note: Junction Field Effect Transistor model*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTO | Pinch-off voltage | vto | vto | vto |
| BETA | Transconductance parameter | beta | beta | beta |
| LAMBDA | Channel length modulation | lambda | lambda | lambda |
| RD | Drain resistance | rd | rd | rd |
| RS | Source resistance | rs | rs | rs |
| CGS | Gate-source capacitance | cgs | cgs | cgs |
| CGD | Gate-drain capacitance | cgd | cgd | cgd |
| PB | Gate junction potential | pb | pb | pb |
| IS | Gate saturation current | is | is | is |
| FC | Forward bias depletion coefficient | fc | fc | fc |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |

<a id="mesfet-model"></a>

### MESFET Model

*Note: Metal Semiconductor Field Effect Transistor model*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTO | Pinch-off voltage | vto | vto | vto |
| BETA | Transconductance parameter | beta | beta | beta |
| B | Doping profile parameter | b | b | b |
| ALPHA | Saturation voltage parameter | alpha | alpha | alpha |
| LAMBDA | Channel length modulation | lambda | lambda | lambda |
| RD | Drain resistance | rd | rd | rd |
| RS | Source resistance | rs | rs | rs |
| RG | Gate resistance | rg | rg | rg |
| CGS | Gate-source capacitance | cgs | cgs | cgs |
| CGD | Gate-drain capacitance | cgd | cgd | cgd |
| PB | Gate junction potential | pb | pb | pb |
| IS | Gate saturation current | is | is | is |
| FC | Forward bias depletion coefficient | fc | fc | fc |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |

## Bipolar Transistor Models

<a id="gummel-poon-bjt-model"></a>

### Gummel-Poon BJT Model

*Note: Standard SPICE bipolar transistor model*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| IS | Transport saturation current | is | is | is |
| BF | Ideal maximum forward beta | bf | bf | bf |
| BR | Ideal maximum reverse beta | br | br | br |
| NF | Forward current emission coefficient | nf | nf | nf |
| NR | Reverse current emission coefficient | nr | nr | nr |
| VAF | Forward Early voltage | vaf | vaf | vaf |
| VAR | Reverse Early voltage | var | var | var |
| IKF | Forward knee current | ikf | ikf | ikf |
| IKR | Reverse knee current | ikr | ikr | ikr |
| ISE | Base-emitter leakage saturation current | ise | ise | ise |
| ISC | Base-collector leakage saturation current | isc | isc | isc |
| NE | Base-emitter leakage emission coefficient | ne | ne | ne |
| NC | Base-collector leakage emission coefficient | nc | nc | nc |
| CJC | Base-collector zero-bias depletion capacitance | cjc | cjc | cjc |
| CJE | Base-emitter zero-bias depletion capacitance | cje | cje | cje |
| VJC | Base-collector built-in potential | vjc | vjc | vjc |
| VJE | Base-emitter built-in potential | vje | vje | vje |
| MJC | Base-collector grading coefficient | mjc | mjc | mjc |
| MJE | Base-emitter grading coefficient | mje | mje | mje |
| TF | Ideal forward transit time | tf | tf | tf |
| TR | Ideal reverse transit time | tr | tr | tr |
| RB | Zero-bias base resistance | rb | rb | rb |
| RBM | Minimum base resistance | rbm | rbm | rbm |
| IRB | Current where base resistance falls to 50% | irb | irb | irb |
| RC | Collector resistance | rc | rc | rc |
| RE | Emitter resistance | re | re | re |
| ITF | High-current parameter for effect on TF | itf | itf | itf |
| VTF | Voltage describing dependence of TF | vtf | vtf | vtf |
| XTF | Coefficient for bias dependence of TF | xtf | xtf | xtf |
| VJS | Substrate junction built-in potential | vjs | vjs | vjs |
| CJS | Zero-bias collector-substrate capacitance | cjs | cjs | cjs |
| MJS | Substrate junction grading coefficient | mjs | mjs | mjs |
| FC | Forward bias depletion capacitance coefficient | fc | fc | fc |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| EG | Energy gap | eg | eg | eg |
| XTI | Temperature exponent for IS | xti | xti | xti |
| XTB | Forward and reverse beta temperature coefficient | xtb | xtb | xtb |
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |

## Diode Models

<a id="spice-diode-model"></a>

### SPICE Diode Model

*Note: Standard SPICE diode model*

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| IS | Saturation current | is | is | is |
| RS | Series resistance | rs | rs | rs |
| N | Emission coefficient | n | n | n |
| BV | Reverse breakdown voltage | bv | bv | bv |
| IBV | Reverse breakdown current | ibv | ibv | ibv |
| CJ0 | Zero-bias junction capacitance | cj0 | cj0 | cj0 |
| VJ | Junction potential | vj | vj | vj |
| M | Grading coefficient | m | m | m |
| CJSW | Zero-bias sidewall capacitance | cjsw | cjsw | cjsw |
| VJSW | Sidewall junction potential | vjsw | vjsw | vjsw |
| MSW | Sidewall grading coefficient | msw | msw | msw |
| FC | Forward bias depletion capacitance coefficient | fc | fc | fc |
| TT | Transit time | tt | tt | tt |
| IKF | High injection knee current | ikf | ikf | ikf |
| ISR | Recombination current | isr | isr | isr |
| NR | Emission coefficient for ISR | nr | nr | nr |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| EG | Energy gap | eg | eg | eg |
| XTI | Saturation current temperature exponent | xti | xti | xti |
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |

<a id="advanced-diode-parameters"></a>

### Advanced Diode Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| LEVEL | Model level | level | level | level |
| TLEV | Temperature equation selector | tlev | tlev | tlev |
| TLEVC | Temperature equation selector for capacitance | tlevc | tlevc | tlevc |
| CTA | Capacitance temperature coefficient | cta | cta | cta |
| CTP | Capacitance temperature coefficient | ctp | ctp | ctp |
| PTA | Potential temperature coefficient | pta | pta | pta |
| PTP | Potential temperature coefficient | ptp | ptp | ptp |
| TBV1 | Breakdown voltage temperature coefficient | tbv1 | tbv1 | tbv1 |
| TBV2 | Breakdown voltage temperature coefficient | tbv2 | tbv2 | tbv2 |
| TRS1 | Series resistance temperature coefficient | trs1 | trs1 | trs1 |
| TRS2 | Series resistance temperature coefficient | trs2 | trs2 | trs2 |
| MINR | Minimum resistance | minr | minr | minr |
| IK | High injection knee current | ik | ik | ik |
| ISR | Recombination current | isr | isr | isr |
| NR | Emission coefficient for ISR | nr | nr | nr |

## Passive Component Models

<a id="resistor-models"></a>

### Resistor Models

<a id="linear-resistor-parameters"></a>

#### Linear Resistor Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| R | Resistance value | r | r | r |
| TC1 | Linear temperature coefficient | tc1 | tc1 | tc1 |
| TC2 | Quadratic temperature coefficient | tc2 | tc2 | tc2 |
| TCE | Exponential temperature coefficient | tce | tce | tce |
| RSH | Sheet resistance | rsh | rsh | rsh |
| L | Length | l | l | l |
| W | Width | w | w | w |
| NR | Number of squares | nr | nr | nr |
| DEFW | Default width | defw | defw | defw |
| DEFL | Default length | defl | defl | defl |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |

#### Advanced Resistor Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| LEVEL | Model level | level | level | level |
| WF | Width narrowing factor | wf | wf | wf |
| LF | Length narrowing factor | lf | lf | lf |
| RSW | Source/drain sheet resistance | rsw | rsw | rsw |
| DW | Width difference | dw | dw | dw |
| DL | Length difference | dl | dl | dl |
| MULT | Multiplier | mult | mult | mult |
| PVC1 | Linear voltage coefficient | pvc1 | pvc1 | pvc1 |
| PVC2 | Quadratic voltage coefficient | pvc2 | pvc2 | pvc2 |
| PTC1 | Linear temperature coefficient | ptc1 | ptc1 | ptc1 |
| PTC2 | Quadratic temperature coefficient | ptc2 | ptc2 | ptc2 |
| TFAC | Temperature factor | tfac | tfac | tfac |

<a id="capacitor-models"></a>

### Capacitor Models

<a id="linear-capacitor-parameters"></a>

#### Linear Capacitor Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| C | Capacitance value | c | c | c |
| IC | Initial voltage | ic | ic | ic |
| L | Inductance (for LCR models) | l | l | l |
| R | Resistance (for LCR models) | r | r | r |
| TC1 | Linear temperature coefficient | tc1 | tc1 | tc1 |
| TC2 | Quadratic temperature coefficient | tc2 | tc2 | tc2 |
| VC1 | Linear voltage coefficient | vc1 | vc1 | vc1 |
| VC2 | Quadratic voltage coefficient | vc2 | vc2 | vc2 |
| BV | Breakdown voltage | bv | bv | bv |

## Additional Model Parameters

- [Resistor Model Parameters](#resistor-model-parameters)
- [Capacitor Model Parameters](#capacitor-model-parameters)

| TOX | Oxide thickness | tox | tox | tox |
| LD | Lateral diffusion | ld | ld | ld |
| WD | Width reduction | wd | wd | wd |
| U0 | Surface mobility | u0 | u0 | u0 |
| RSH | Sheet resistance | rsh | rsh | rsh |
| CJ | Zero-bias bulk junction capacitance | cj | cj | cj |
| CJSW | Zero-bias sidewall bulk junction capacitance | cjsw | cjsw | cjsw |
| MJ | Bulk junction grading coefficient | mj | mj | mj |
| MJSW | Sidewall bulk junction grading coefficient | mjsw | mjsw | mjsw |
| PB | Bulk junction potential | pb | pb | pb |
| CGSO | Gate-source overlap capacitance | cgso | cgso | cgso |
| CGDO | Gate-drain overlap capacitance | cgdo | cgdo | cgdo |
| CGBO | Gate-bulk overlap capacitance | cgbo | cgbo | cgbo |

<a id="bsim3v3--bsim4-parameters"></a>

### BSIM3v3 / BSIM4 Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTH0 | Long-channel threshold voltage at Vbs=0 | vth0 | vth0 | vth |
| TOX | Gate oxide thickness | tox | tox | tox |
| U0 | Low-field surface mobility | u0 | u0 | u0 |
| VSAT | Saturation velocity | vsat | vsat | vsat |
| K1 | First-order body effect coefficient | k1 | k1 | k1 |
| K2 | Second-order body effect coefficient | k2 | k2 | k2 |
| K3 | Narrow width effect coefficient | k3 | k3 | k3 |
| K3B | Body effect coefficient of K3 | k3b | k3b | k3b |
| W0 | Narrow width parameter | w0 | w0 | w0 |
| NLX | Lateral non-uniform doping parameter | nlx | nlx | nlx |
| DVT0 | First coefficient of short-channel effect on Vth | dvt0 | dvt0 | dvt0 |
| DVT1 | Second coefficient of short-channel effect on Vth | dvt1 | dvt1 | dvt1 |
| DVT2 | Body-bias coefficient of short-channel effect on Vth | dvt2 | dvt2 | dvt2 |
| DVT0W | First coefficient of narrow width effect on Vth | dvt0w | dvt0w | dvt0w |
| DVT1W | Second coefficient of narrow width effect on Vth | dvt1w | dvt1w | dvt1w |
| DVT2W | Body-bias coefficient of narrow width effect on Vth | dvt2w | dvt2w | dvt2w |
| UTE | Mobility temperature exponent | ute | ute | ute |
| UA | First-order mobility degradation coefficient | ua | ua | ua |
| UB | Second-order mobility degradation coefficient | ub | ub | ub |
| UC | Body-effect of mobility degradation coefficient | uc | uc | uc |
| RDSW | Source/drain resistance per width | rdsw | rdsw | rdsw |
| PRWG | Gate-bias dependence of RDSW | prwg | prwg | prwg |
| PRWB | Body-bias dependence of RDSW | prwb | prwb | prwb |
| WR | Width offset from Weff for Rds calculation | wr | wr | wr |
| DWG | Coefficient of Weff dependence of RDSW | dwg | dwg | dwg |
| DWB | Coefficient of Weff dependence of RDSW | dwb | dwb | dwb |
| PCLM | Channel length modulation parameter | pclm | pclm | pclm |
| PCLMG | PCLM gate dependence | pclmg | pclmg | pclmg |
| CHARGEWF | Charge well factor | chargewf | chargewf | chargewf |
| PDIBL1 | First output resistance DIBL effect correction parameter | pdibl1 | pdibl1 | pdibl1 |
| PDIBL2 | Second output resistance DIBL effect correction parameter | pdibl2 | pdibl2 | pdibl2 |
| PDIBLB | Body-bias dependence of DIBL effect | pdiblb | pdiblb | pdiblb |
| PSCBE1 | First substrate current body effect parameter | pscbe1 | pscbe1 | pscbe1 |
| PSCBE2 | Second substrate current body effect parameter | pscbe2 | pscbe2 | pscbe2 |
| PVAG | Gate dependence of Early voltage | pvag | pvag | pvag |
| DELTA | Effect of Vds on threshold voltage | delta | delta | delta |
| ETA0 | Drain-induced barrier lowering coefficient | eta0 | eta0 | eta0 |
| ETAB | Body-bias dependence of eta0 | etab | etab | etab |
| DSUB | DIBL coefficient exponent in subthreshold region | dsub | dsub | dsub |
| CIT | Interface trap capacitance | cit | cit | cit |
| CDSC | Drain/source to channel coupling capacitance | cdsc | cdsc | cdsc |
| CDSCB | Body-bias dependence of CDSC | cdscb | cdscb | cdscb |
| CDSCD | Drain-bias dependence of CDSC | cdscd | cdscd | cdscd |
| NFACTOR | Subthreshold swing factor | nfactor | nfactor | nfactor |
| XJ | Source/drain junction depth | xj | xj | xj |
| VSATCV | Saturation velocity for C-V | vsatcv | vsatcv | vsatcv |
| MINV | Fitting parameter for moderate inversion | minv | minv | minv |
| MINVCV | Fitting parameter for moderate inversion in C-V | minvcv | minvcv | minvcv |
| NOFF | C-V parameter | noff | noff | noff |
| VOFF | C-V parameter | voff | voff | voff |
| VOFFCV | C-V parameter | voffcv | voffcv | voffcv |
| TNOM | Temperature at which parameters are extracted | tnom | tnom | tnom |
| CGSO | Gate-source overlap capacitance per unit width | cgso | cgso | cgso |
| CGDO | Gate-drain overlap capacitance per unit width | cgdo | cgdo | cgdo |
| CGBO | Gate-bulk overlap capacitance per unit length | cgbo | cgbo | cgbo |
| CAPMOD | Capacitance model selector | capmod | capmod | capmod |
| XPART | Charge partitioning flag | xpart | xpart | xpart |
| CGSL | Gate-source overlap capacitance for short channel | cgsl | cgsl | cgsl |
| CGDL | Gate-drain overlap capacitance for short channel | cgdl | cgdl | cgdl |
| CKAPPAS | S/D sidewall capacitance coefficient | ckappas | ckappas | ckappas |
| CKAPPAD | DIBL capacitance coefficient | ckappad | ckappad | ckappad |
| ACM | Area calculation method | acm | acm | acm |
| CALCACM | Area calculation method | calcacm | calcacm | calcacm |
| BULKMOD | Bulk charge effect model | bulkmod | bulkmod | bulkmod |
| VERSION | Model version number | version | version | version |

<a id="soi-mosfet-parameters"></a>

### SOI MOSFET Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTHSOI | SOI threshold voltage | vthsoi | vthsoi | vthsoi |
| TOXSOI | SOI oxide thickness | toxsoi | toxsoi | toxsoi |
| TBOX | Buried oxide thickness | tbox | tbox | tbox |
| ALPHA | Body factor | alpha | alpha | alpha |
| BETA | Body effect coefficient | beta | beta | beta |
| GAMMA | Body effect coefficient | gamma | gamma | gamma |
| K1 | First-order body effect coefficient | k1 | k1 | k1 |
| K2 | Second-order body effect coefficient | k2 | k2 | k2 |
| ETA | DIBL coefficient | eta | eta | eta |
| DELTA | Vds effect on threshold | delta | delta | delta |
| VSAT | Saturation velocity | vsat | vsat | vsat |
| U0 | Low-field mobility | u0 | u0 | u0 |

<a id="finfet-parameters"></a>

### FinFET Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| TFIN | Fin thickness | tfin | tfin | tfin |
| HFIN | Fin height | hfin | hfin | hfin |
| NF | Number of fins | nf | nf | nf |
| NFIN | Number of fins | nfin | nfin | nfin |
| FPITCH | Fin pitch | fpitch | fpitch | fpitch |
| FECH | Fin etch parameter | fech | fech | fech |
| NBODY | Body doping | nbody | nbody | nbody |
| NSD | Source/drain doping | nsd | nsd | nsd |
| NGATE | Gate doping | ngate | ngate | ngate |
| LPHIG | Fin height parameter | lphig | lphig | lphig |
| LRDSW | RDSW length dependence | lrdsw | lrdsw | lrdsw |
| ARDSW | RDSW area dependence | ardsw | ardsw | ardsw |
| BRDSW | RDSW bias dependence | brdsw | brdsw | brdsw |
| PRWG | Gate dependence of RDSW | prwg | prwg | prwg |

<a id="high-voltage-mosfet-parameters"></a>

### High Voltage MOSFET Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| VTHHV | High voltage threshold | vthhv | vthhv | vthhv |
| TOXHV | High voltage oxide thickness | toxhv | toxhv | toxhv |
| U0HV | High voltage mobility | u0hv | u0hv | u0hv |
| VSATHV | High voltage saturation velocity | vsathv | vsathv | vsathv |
| K1HV | High voltage body effect | k1hv | k1hv | k1hv |
| K2HV | High voltage body effect | k2hv | k2hv | k2hv |
| K3HV | High voltage narrow width | k3hv | k3hv | k3hv |
| W0HV | High voltage width offset | w0hv | w0hv | w0hv |
| NLXHV | High voltage doping | nlxhv | nlxhv | nlxhv |
| DVT0HV | High voltage short channel | dvt0hv | dvt0hv | dvt0hv |
| DVT1HV | High voltage short channel | dvt1hv | dvt1hv | dvt1hv |
| DVT2HV | High voltage body bias | dvt2hv | dvt2hv | dvt2hv |

<a id="capacitance-parameters"></a>

### Capacitance Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| CJ | Zero-bias bulk junction capacitance per unit area | cj | cj | cj |
| CJSW | Zero-bias sidewall bulk junction capacitance per unit length | cjsw | cjsw | cjsw |
| CJSWG | Zero-bias gate sidewall bulk junction capacitance per unit length | cjswg | cjswg | cjswg |
| MJ | Bulk junction grading coefficient | mj | mj | mj |
| MJSW | Sidewall bulk junction grading coefficient | mjsw | mjsw | mjsw |
| MJSWG | Gate sidewall bulk junction grading coefficient | mjswg | mjswg | mjswg |
| PB | Bulk junction potential | pb | pb | pb |
| PBSW | Sidewall bulk junction potential | pbsw | pbsw | pbsw |
| PBSWG | Gate sidewall bulk junction potential | pbswg | pbswg | pbswg |
| TT | Transit time | tt | tt | tt |

<a id="process-and-geometry-parameters"></a>

### Process and Geometry Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| LMIN | Minimum channel length | lmin | lmin | lmin |
| LMAX | Maximum channel length | lmax | lmax | lmax |
| WMIN | Minimum channel width | wmin | wmin | wmin |
| WMAX | Maximum channel width | wmax | wmax | wmax |
| XL | Length oxide encroachment | xl | xl | xl |
| XW | Width oxide encroachment | xw | xw | xw |
| BINUNIT | Bin unit parameter | binunit | binunit | binunit |
| TOXM | Gate oxide thickness multiplier | toxm | toxm | toxm |
| NCH | Channel doping concentration | nch | nch | nch |
| LLN | Length/length dependence flag | lln | lln | lln |
| LWN | Length/width dependence flag | lwn | lwn | lwn |
| WWN | Width/width dependence flag | wwn | wwn | wwn |
| WINT | Width integration | wint | wint | wint |
| MOBMOD | Mobility model selector | mobmod | mobmod | mobmod |
| LDIF | Length of diffusion | ldif | ldif | ldif |
| HDIF | Height of diffusion | hdif | hdif | hdif |
| RSC | Source contact resistance | rsc | rsc | rsc |
| RDC | Drain contact resistance | rdc | rdc | rdc |
| NRD | Number of drain regions | nrd | nrd | nrd |
| NRS | Number of source regions | nrs | nrs | nrs |
| LINTNOI | Length integration for noise | lintnoi | lintnoi | lintnoi |

<a id="temperature-parameters"></a>

### Temperature Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| TNOM | Nominal temperature | tnom | tnom | tnom |
| KT1 | Temperature coefficient for threshold voltage | kt1 | kt1 | kt1 |
| KT2 | Body-bias coefficient of threshold voltage temperature effect | kt2 | kt2 | kt2 |
| KT1L | Channel length dependence of KT1 | kt1l | kt1l | kt1l |
| K2T | Flicker noise coefficient | k2t | k2t | k2t |
| UTE | Mobility temperature exponent | ute | ute | ute |
| UA1 | Temperature coefficient for UA | ua1 | ua1 | ua1 |
| UB1 | Temperature coefficient for UB | ub1 | ub1 | ub1 |
| UC1 | Temperature coefficient for UC | uc1 | uc1 | uc1 |

<a id="resistor-model-parameters"></a>

## Resistor Model Parameters

<a id="linear-resistor-parameters"></a>

### Linear Resistor Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| R | Resistance value | r | r | r |
| TC1 | Linear temperature coefficient | tc1 | tc1 | tc1 |
| TC2 | Quadratic temperature coefficient | tc2 | tc2 | tc2 |
| TCE | Exponential temperature coefficient | tce | tce | tce |
| RSH | Sheet resistance | rsh | rsh | rsh |
| L | Length | l | l | l |
| W | Width | w | w | w |
| NR | Number of squares | nr | nr | nr |
| DEFW | Default width | defw | defw | defw |
| DEFL | Default length | defl | defl | defl |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |

<a id="advanced-resistor-parameters"></a>

### Advanced Resistor Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| LEVEL | Model level | level | level | level |
| WF | Width narrowing factor | wf | wf | wf |
| LF | Length narrowing factor | lf | lf | lf |
| RSW | Source/drain sheet resistance | rsw | rsw | rsw |
| DW | Width difference | dw | dw | dw |
| DL | Length difference | dl | dl | dl |
| MULT | Multiplier | mult | mult | mult |

<a id="capacitor-model-parameters"></a>

## Capacitor Model Parameters

<a id="linear-capacitor-parameters"></a>

### Linear Capacitor Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| C | Capacitance value | c | c | c |
| IC | Initial voltage | ic | ic | ic |
| L | Inductance (for LCR models) | l | l | l |
| R | Resistance (for LCR models) | r | r | r |
| TC1 | Linear temperature coefficient | tc1 | tc1 | tc1 |
| TC2 | Quadratic temperature coefficient | tc2 | tc2 | tc2 |
| VC1 | Linear voltage coefficient | vc1 | vc1 | vc1 |
| VC2 | Quadratic voltage coefficient | vc2 | vc2 | vc2 |
| BV | Breakdown voltage | bv | bv | bv |

<a id="mim-capacitor-parameters"></a>

### MIM Capacitor Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| CJ | Zero-bias capacitance per unit area | cj | cj | cj |
| CJSW | Zero-bias sidewall capacitance per unit length | cjsw | cjsw | cjsw |
| CJSWG | Zero-bias gate sidewall capacitance per unit length | cjswg | cjswg | cjswg |
| MJ | Area grading coefficient | mj | mj | mj |
| MJSW | Sidewall grading coefficient | mjsw | mjsw | mjsw |
| MJSWG | Gate sidewall grading coefficient | mjswg | mjswg | mjswg |
| PB | Built-in potential | pb | pb | pb |
| PBSW | Sidewall built-in potential | pbsw | pbsw | pbsw |
| PBSWG | Gate sidewall built-in potential | pbswg | pbswg | pbswg |
| TC1 | Linear temperature coefficient | tc1 | tc1 | tc1 |
| TC2 | Quadratic temperature coefficient | tc2 | tc2 | tc2 |

<a id="bjt-model-parameters"></a>

## BJT Model Parameters

<a id="vbic-model-parameters"></a>

### VBIC Model Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| IS | Transport saturation current | is | is | is |
| BF | Ideal maximum forward beta | bf | bf | bf |
| BR | Ideal maximum reverse beta | br | br | br |
| NF | Forward current emission coefficient | nf | nf | nf |
| NR | Reverse current emission coefficient | nr | nr | nr |
| VAF | Forward Early voltage | vaf | vaf | vaf |
| VAR | Reverse Early voltage | var | var | var |
| IKF | Forward knee current | ikf | ikf | ikf |
| IKR | Reverse knee current | ikr | ikr | ikr |
| ISE | Base-emitter leakage saturation current | ise | ise | ise |
| ISC | Base-collector leakage saturation current | isc | isc | isc |
| NE | Base-emitter leakage emission coefficient | ne | ne | ne |
| NC | Base-collector leakage emission coefficient | nc | nc | nc |
| CJC | Base-collector zero-bias depletion capacitance | cjc | cjc | cjc |
| CJE | Base-emitter zero-bias depletion capacitance | cje | cje | cje |
| VJC | Base-collector built-in potential | vjc | vjc | vjc |
| VJE | Base-emitter built-in potential | vje | vje | vje |
| MJC | Base-collector grading coefficient | mjc | mjc | mjc |
| MJE | Base-emitter grading coefficient | mje | mje | mje |
| TF | Ideal forward transit time | tf | tf | tf |
| TR | Ideal reverse transit time | tr | tr | tr |
| RB | Zero-bias base resistance | rb | rb | rb |
| RBM | Minimum base resistance | rbm | rbm | rbm |
| IRB | Current where base resistance falls to 50% | irb | irb | irb |
| RC | Collector resistance | rc | rc | rc |
| RE | Emitter resistance | re | re | re |
| ITF | High-current parameter for effect on TF | itf | itf | itf |
| VTF | Voltage describing dependence of TF | vtf | vtf | vtf |
| XTF | Coefficient for bias dependence of TF | xtf | xtf | xtf |
| VJS | Substrate junction built-in potential | vjs | vjs | vjs |
| CJS | Zero-bias collector-substrate capacitance | cjs | cjs | cjs |
| MJS | Substrate junction grading coefficient | mjs | mjs | mjs |
| FC | Forward bias depletion capacitance coefficient | fc | fc | fc |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| EG | Energy gap | eg | eg | eg |
| XTI | Temperature exponent for IS | xti | xti | xti |
| XTB | Forward and reverse beta temperature coefficient | xtb | xtb | xtb |
| XTB | Forward and reverse beta temperature coefficient | xtb | xtb | xtb |

<a id="diode-model-parameters"></a>

## Diode Model Parameters

<a id="standard-diode-parameters"></a>

### Standard Diode Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| IS | Saturation current | is | is | is |
| RS | Series resistance | rs | rs | rs |
| N | Emission coefficient | n | n | n |
| BV | Reverse breakdown voltage | bv | bv | bv |
| IBV | Reverse breakdown current | ibv | ibv | ibv |
| CJ0 | Zero-bias junction capacitance | cj0 | cj0 | cj0 |
| VJ | Junction potential | vj | vj | vj |
| M | Grading coefficient | m | m | m |
| CJSW | Zero-bias sidewall capacitance | cjsw | cjsw | cjsw |
| VJSW | Sidewall junction potential | vjsw | vjsw | vjsw |
| MSW | Sidewall grading coefficient | msw | msw | msw |
| FC | Forward bias depletion capacitance coefficient | fc | fc | fc |
| TT | Transit time | tt | tt | tt |
| IKF | High injection knee current | ikf | ikf | ikf |
| ISR | Recombination current | isr | isr | isr |
| NR | Emission coefficient for ISR | nr | nr | nr |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| EG | Energy gap | eg | eg | eg |
| XTI | Saturation current temperature exponent | xti | xti | xti |
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |

<a id="advanced-diode-parameters"></a>

### Advanced Diode Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| LEVEL | Model level | level | level | level |
| TLEV | Temperature equation selector | tlev | tlev | tlev |
| TLEVC | Temperature equation selector for capacitance | tlevc | tlevc | tlevc |
| CTA | Capacitance temperature coefficient | cta | cta | cta |
| CTP | Capacitance temperature coefficient | ctp | ctp | ctp |
| PTA | Potential temperature coefficient | pta | pta | pta |
| PTP | Potential temperature coefficient | ptp | ptp | ptp |
| TBV1 | Breakdown voltage temperature coefficient | tbv1 | tbv1 | tbv1 |
| TBV2 | Breakdown voltage temperature coefficient | tbv2 | tbv2 | tbv2 |
| TRS1 | Series resistance temperature coefficient | trs1 | trs1 | trs1 |
| TRS2 | Series resistance temperature coefficient | trs2 | trs2 | trs2 |

<a id="inductor-model-parameters"></a>

## Inductor Model Parameters

<a id="linear-inductor-parameters"></a>

### Linear Inductor Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| L | Inductance value | l | l | l |
| R | Series resistance | r | r | r |
| C | Parallel capacitance | c | c | c |
| IC | Initial current | ic | ic | ic |
| TC1 | Linear temperature coefficient | tc1 | tc1 | tc1 |
| TC2 | Quadratic temperature coefficient | tc2 | tc2 | tc2 |
| TNOM | Nominal temperature | tnom | tnom | tnom |
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |

<a id="mutual-inductor-parameters"></a>

### Mutual Inductor Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| K | Coupling coefficient | k | k | k |
| LM | Mutual inductance | lm | lm | lm |

<a id="advanced-inductor-parameters"></a>

### Advanced Inductor Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| LEVEL | Model level | level | level | level |
| FREQ | Frequency for distributed effects | freq | freq | freq |
| D | Diameter | d | d | d |
| NT | Number of turns | nt | nt | nt |
| SPACING | Turn spacing | spacing | spacing | spacing |
| LENGTH | Length | length | length | length |

<a id="transmission-line-parameters"></a>
## Transmission Line Parameters

<a id="lossless-transmission-line-t-element"></a>

### Lossless Transmission Line (T Element)

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| Z0 | Characteristic impedance | z0 | z0 | z0 |
| TD | Transmission delay | td | td | td |
| F | Frequency for loss calculation | f | f | f |
| NL | Normalized length | nl | nl | nl |

<a id="lossy-transmission-line-w-element"></a>

### Lossy Transmission Line (W Element)

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| R | Resistance per unit length | r | r | r |
| L | Inductance per unit length | l | l | l |
| G | Conductance per unit length | g | g | g |
| C | Capacitance per unit length | c | c | c |
| LEN | Length of transmission line | len | len | len |
| LUMPS | Number of lumped segments | lumps | lumps | lumps |

<a id="u-element-lumped-transmission-line"></a>

### U Element (Lumped Transmission Line)

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| LEN | Length of transmission line | len | len | len |
| N | Number of sections | n | n | n |
| R1 | Resistance of series element | r1 | r1 | r1 |
| L1 | Inductance of series element | l1 | l1 | l1 |
| C1 | Capacitance of shunt element | c1 | c1 | c1 |
| R2 | Resistance of shunt element | r2 | r2 | r2 |

<a id="references"></a>

<a id="mosfet-noise-parameters"></a>

## MOSFET Noise Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |
| EF | Flicker noise frequency exponent | ef | ef | ef |
| NOIA | Oxide trap density for flicker noise | noia | noia | noia |
| NOIB | Gate-induced flicker noise coefficient | noib | noib | noib |
| NOIC | Coupling flicker noise coefficient | noic | noic | noic |
| EM | Mobility fluctuation parameter | em | em | em |
| AF | Flicker noise exponent | af | af | af |
| EF | Flicker noise frequency exponent | ef | ef | ef |
| NOIMOD | Noise model selector | noimod | noimod | noimod |
| TNOIMOD | Thermal noise model selector | tnoimod | tnoimod | tnoimod |

<a id="resistor-noise-parameters"></a>

## Resistor Noise Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |
| EF | Flicker noise frequency exponent | ef | ef | ef |
| LF | Flicker noise frequency exponent | lf | lf | lf |
| WF | Flicker noise frequency exponent | wf | wf | wf |
| NOISE | Noise model selector | noise | noise | noise |
| RNOISE | Resistor noise model | rnoise | rnoise | rnoise |

<a id="bjt-noise-parameters"></a>

## BJT Noise Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| KF | Flicker noise coefficient | kf | kf | kf |
| AF | Flicker noise exponent | af | af | af |
| FB | Flicker noise coefficient for base | fb | fb | fb |
| FC | Flicker noise coefficient for collector | fc | fc | fc |
| FN | Flicker noise coefficient for emitter | fn | fn | fn |
| NLEV | Noise model level | nlev | nlev | nlev |
| GSP | Shot noise generation parameter | gsp | gsp | gsp |

<a id="process-variation-parameters-1"></a>

## Process Variation Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| LVAR | Length variation | lvar | lvar | lvar |
| WVAR | Width variation | wvar | wvar | wvar |
| VTHVAR | Threshold voltage variation | vthvar | vthvar | vthvar |
| TOXVAR | Oxide thickness variation | toxvar | toxvar | toxvar |
| U0VAR | Mobility variation | u0var | u0var | u0var |
| RSHVAR | Sheet resistance variation | rshvar | rshvar | rshvar |
| PVAR | Process variation parameter | pvar | pvar | pvar |
| LVARC | Length variation coefficient | lvarc | lvarc | lvarc |
| WVARC | Width variation coefficient | wvarc | wvarc | wvarc |
| GLOBALFLAG | Global variation flag | globalflag | globalflag | globalflag |
| MISMATCHFLAG | Mismatch variation flag | mismatchflag | mismatchflag | mismatchflag |
| SIGMA_FACTOR | Sigma factor for variation | sigma_factor | sigma_factor | sigma_factor |
| SIGMA_FACTOR_FLICKER | Flicker noise sigma factor | sigma_factor_flicker | sigma_factor_flicker | sigma_factor_flicker |
| LOD_FLAG | Length of diffusion flag | lod_flag | lod_flag | lod_flag |
| OSE_FLAG | Overlap source extension flag | ose_flag | ose_flag | ose_flag |
| PSE_FLAG | Polysilicon source extension flag | pse_flag | pse_flag | pse_flag |
| RG_FLAG | Gate resistance flag | rg_flag | rg_flag | rg_flag |
| OVERLAP_MULT | Overlap capacitance multiplier | overlap_mult | overlap_mult | overlap_mult |
| AJUNCTION_MULT | Area junction capacitance multiplier | ajunction_mult | ajunction_mult | ajunction_mult |
| PJUNCTION_MULT | Perimeter junction capacitance multiplier | pjunction_mult | pjunction_mult | pjunction_mult |
| LINT_DIFF | Length difference for diffusion | lint_diff | lint_diff | lint_diff |
| WINT_DIFF | Width difference for diffusion | wint_diff | wint_diff | wint_diff |
| DLC_DIFF | Delta length for capacitance | dlc_diff | dlc_diff | dlc_diff |
| DWC_DIFF | Delta width for capacitance | dwc_diff | dwc_diff | dwc_diff |
| RDSW_DIFF_0 | RDSW variation parameter | rdsw_diff_0 | rdsw_diff_0 | rdsw_diff_0 |
| VTH0_DIFF_0 | VTH0 variation parameter | vth0_diff_0 | vth0_diff_0 | vth0_diff_0 |
| U0_DIFF_0 | U0 variation parameter | u0_diff_0 | u0_diff_0 | u0_diff_0 |
| VSAT_DIFF_0 | VSAT variation parameter | vsat_diff_0 | vsat_diff_0 | vsat_diff_0 |
| VOFF_DIFF_0 | VOFF variation parameter | voff_diff_0 | voff_diff_0 | voff_diff_0 |
| K2_DIFF_0 | K2 variation parameter | k2_diff_0 | k2_diff_0 | k2_diff_0 |
| NFACTOR_DIFF_0 | NFACTOR variation parameter | nfactor_diff_0 | nfactor_diff_0 | nfactor_diff_0 |
| KT1_DIFF_0 | KT1 variation parameter | kt1_diff_0 | kt1_diff_0 | kt1_diff_0 |
| DEVTYPE | Device type | devtype | devtype | devtype |
| EASUB | Electron affinity | easub | easub | easub |
| NI0SUB | Intrinsic carrier concentration | ni0sub | ni0sub | ni0sub |
| BG0SUB | Bandgap narrowing | bg0sub | bg0sub | bg0sub |
| NC0SUB | Conduction band density | nc0sub | nc0sub | nc0sub |
| PHIG | Gate work function | phig | phig | phig |
| DEOT | Delta equivalent oxide thickness | deot | deot | deot |
| DTOXP | Delta oxide thickness for PMOS | dtoxp | dtoxp | dtoxp |
| DPHIG | Delta gate work function | dphig | dphig | dphig |
| DCGDO | Delta gate-drain overlap capacitance | dcgdo | dcgdo | dcgdo |
| DCGSL | Delta gate-source overlap capacitance for short channel | dcgsl | dcgsl | dcgsl |
| DCGDL | Delta gate-drain overlap capacitance for short channel | dcgdl | dcgdl | dcgdl |
| DCJ | Delta bulk junction capacitance | dcj | dcj | dcj |
| DCJSW | Delta sidewall bulk junction capacitance | dcjsw | dcjsw | dcjsw |
| DCJSWG | Delta gate sidewall bulk junction capacitance | dcjswg | dcjswg | dcjswg |
| DCGSO | Delta gate-source overlap capacitance | dcgso | dcgso | dcgso |
| DU0 | Delta mobility | du0 | du0 | du0 |
| DVSAT | Delta saturation velocity | dvsat | dvsat | dvsat |

<a id="mismatch-parameters"></a>

## Mismatch Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| A0 | Area mismatch coefficient | a0 | a0 | a0 |
| A1 | Area mismatch coefficient | a1 | a1 | a1 |
| A2 | Area mismatch coefficient | a2 | a2 | a2 |
| B0 | Perimeter mismatch coefficient | b0 | b0 | b0 |
| B1 | Perimeter mismatch coefficient | b1 | b1 | b1 |
| B2 | Perimeter mismatch coefficient | b2 | b2 | b2 |
| C0 | Distance mismatch coefficient | c0 | c0 | c0 |
| C1 | Distance mismatch coefficient | c1 | c1 | c1 |
| C2 | Distance mismatch coefficient | c2 | c2 | c2 |
| MISMATCH | Mismatch model selector | mismatch | mismatch | mismatch |

<a id="monte-carlo-parameters"></a>

## Monte Carlo Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| MCRUNS | Number of Monte Carlo runs | mcruns | mcruns | mcruns |
| MCSEED | Random seed for Monte Carlo | mcseed | mcseed | mcseed |
| MCTYPE | Monte Carlo type | mctype | mctype | mctype |
| MCDEV | Device for Monte Carlo | mcdev | mcdev | mcdev |
| MCLOT | Lot number for Monte Carlo | mclot | mclot | mclot |
| MCWAFR | Wafer number for Monte Carlo | mcwafr | mcwafr | mcwafr |
| MCCHIP | Chip number for Monte Carlo | mcchip | mcchip | mcchip |
| MCDEVX | X coordinate for Monte Carlo | mcdevx | mcdevx | mcdevx |
| MCDEVY | Y coordinate for Monte Carlo | mcdevy | mcdevy | mcdevy |

<a id="s-parameter-model-parameters"></a>

## S-Parameter Model Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| S11 | S11 parameter | s11 | s11 | s11 |
| S12 | S12 parameter | s12 | s12 | s12 |
| S21 | S21 parameter | s21 | s21 | s21 |
| S22 | S22 parameter | s22 | s22 | s22 |
| FREQ | Frequency for S-parameters | freq | freq | freq |
| Z0 | Reference impedance | z0 | z0 | z0 |
| ANGLE | Phase angle | angle | angle | angle |
| MAG | Magnitude | mag | mag | mag |
| DB | Magnitude in dB | db | db | db |
| SPAR | S-parameter model selector | spar | spar | spar |

<a id="varactor-model-parameters"></a>

## Varactor Model Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| CJ0 | Zero-bias capacitance | cj0 | cj0 | cj0 |
| VJ | Built-in potential | vj | vj | vj |
| M | Grading coefficient | m | m | m |
| CJSW | Sidewall capacitance | cjsw | cjsw | cjsw |
| VJSW | Sidewall built-in potential | vjsw | vjsw | vjsw |
| MSW | Sidewall grading coefficient | msw | msw | msw |
| FC | Forward bias depletion coefficient | fc | fc | fc |
| BV | Breakdown voltage | bv | bv | bv |
| IBV | Breakdown current | ibv | ibv | ibv |
| TT | Transit time | tt | tt | tt |

<a id="rf-mosfet-parameters"></a>

## RF MOSFET Parameters

| Parameter | Description | NGSPICE | HSPICE | SPECTRE |
|-----------|-------------|---------|--------|---------|
| RGATE | Gate resistance | rgate | rgate | rgate |
| RS | Source resistance | rs | rs | rs |
| RD | Drain resistance | rd | rd | rd |
| RSB | Source-body resistance | rsb | rsb | rsb |
| RDB | Drain-body resistance | rdb | rdb | rdb |
| CGSO | Gate-source overlap capacitance | cgso | cgso | cgso |
| CGDO | Gate-drain overlap capacitance | cgdo | cgdo | cgdo |
| CGBO | Gate-bulk overlap capacitance | cgbo | cgbo | cgbo |
| CGS | Gate-source capacitance | cgs | cgs | cgs |
| CGD | Gate-drain capacitance | cgd | cgd | cgd |
| CGB | Gate-bulk capacitance | cgb | cgb | cgb |
| FMIN | Minimum noise figure | fmin | fmin | fmin |
| GAMMA_OPT | Optimal reflection coefficient magnitude | gamma_opt | gamma_opt | gamma_opt |
| RN | Equivalent noise resistance | rn | rn | rn |

## References

- [NGSPICE Manual](docs/ngspice/)
- [HSPICE Documentation](docs/hspice/)
- [SPECTRE Documentation](docs/spectre/)

This reference table is based on common industry practices and may vary slightly depending on specific model versions and simulator implementations.
