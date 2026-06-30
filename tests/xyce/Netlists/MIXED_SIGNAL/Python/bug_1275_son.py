# Certification test for SON Bug 1275

# file will be used to compare information also output to
# stdout against a gold standard
f= open("bug_1275_son.cir.OutputData","w+")

import sys
# import the Xyce libraries
from xyce_interface import xyce_interface

# this calls the xyce_interface.open() method to
# make a xyce object
if (len(sys.argv) > 1):
  libDirectory = sys.argv[1]
  xyce_obj = xyce_interface(libdir=libDirectory)
else:
  xyce_obj = xyce_interface()
print( xyce_obj )

xyce_obj.initialize(['bug_1275_son.cir'])

# define timing
clock_frequency = 1e5
clock_period = 1/clock_frequency
stop_time = 0.001
current_time = 0.0

# simulate the first clock cycle
current_time += clock_period
(result, actual_time) = xyce_obj.simulateUntil( current_time )

# loop through the range of values
while current_time <= stop_time:

  print('-'*100)

  # read the ADC values from the circuit
  (adc_result, adc_names, adc_num_names, adc_num_points, adc_time_array, adc_voltage_array) = xyce_obj.getTimeVoltagePairsADC()

  # output to stdout (for human readability)
  print('Time = %.3f ms' % (current_time*1000))
  print("ADC-VDD = %f, ADC-VOUT = %f" % (adc_voltage_array[0][0], adc_voltage_array[1][0]))
  print("ADC-VDD = %f, ADC-VOUT = %f" % (adc_voltage_array[0][1], adc_voltage_array[1][1]))

  # output to file for verification
  f.write('Time = %.3f ms\n' % (current_time*1000))
  f.write("ADC-VDD = %f, ADC-VOUT = %f\n" % (adc_voltage_array[0][0], adc_voltage_array[1][0]))
  f.write("ADC-VDD = %f, ADC-VOUT = %f\n" % (adc_voltage_array[0][1], adc_voltage_array[1][1]))
  
  # progress another clock cycle
  current_time += clock_period
  (result, actual_time) = xyce_obj.simulateUntil( current_time )
  print("Updated to time = %.3f ms with with result = %d" % (actual_time*1000, result))
  f.write("Updated to time = %.3f ms with with result = %d\n" % (actual_time*1000, result))

# close the Xyce object
xyce_obj.close()

# close file also
f.close()
