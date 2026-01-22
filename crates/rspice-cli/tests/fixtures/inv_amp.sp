* Inverting Amplifier for Sensitivity Analysis
* Gain = -R2/R1 = -10
* Sensitivity test: dVout/dR1

* Input voltage
Vin inp 0 DC 0.1

* Op-amp supply
VCC vcc 0 DC 5
VEE vee 0 DC -5

* Inverting amplifier config
R1 inp vm 1k
R2 vm out 10k

* Ideal op-amp (simplified)
E1 out 0 0 vm 100k

* Check: Vout should be ~-1V
* Sensitivity of Vout to R1 expected to be positive
* (increasing R1 makes gain less negative = Vout less negative)

.END
