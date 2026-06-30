These are tests using the Matlab / Simulink interface to Xyce.

It should be possible to automate running these tests in our nightly
regression testing.  The Matlab Unit Test classes or Simulink Test are 
probably the way to go with this.  But until that is figured out, here 
is how to run this test manually.

1. Install Matlab & Simulink 

2. Build Xyce with Cmake on a computer with Matlab installed.  Cmake should
detect Matlab's required "mex" compiler and the needed header directories
and support libraries.  You don't need to install Xyce at this point.  
You can use the Xyce-Simulink interface from the build directory

2. Start Matlab.  

3. In the upper icon pane in the Matlab window click on the Set Path icon
This will open a dialog box where you'll need to add the directories holding
the Xyce-Simulink definition file (XyceBlocks.slx) and the compiled interface 
library (xyce_sfunction.mexmaci64 on a 64bit Mac OS)

The directories you need to add are something like these:

/your/path/to/XyceSourceDir/Xyce/utils/SimulinkInterface/
/your/path/to/XyceBuildDir/utils/SimulinkInterface/

4. Next use Matlab's directory navagitor to move to the directory where 
this file and the examples are located. Matlab needs to be in this diretory
to find the simulation netlists.

5. Now click on the Simulink icon to start simulink.

6. Once Simulink stats, click on the Open button on the Simulink window.  
Open the file Driver1_1DAC_2ADC.slx

7. You can now click on the Run icon to run the Simulink - Xyce simulation.  
In this case Simulink is providing the input to the circuit and Xyce is 
calculating the output.  Click on the the scope icons to view the transient
output.  Click on the "View diagnostics" text on the bottom of the Simulink
window to see Xyce's simulation output.



