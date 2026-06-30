#include <vpi_user.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <N_CIR_XyceCInterface.h>

static int ADCtest_compiletf(char*user_data)
{
      return 0;
}

static int ADCtest_calltf(char*user_data)
{
      printf("In calltf for ADCtest\n");

      // Used as a pointer to a pointer to an N_CIR_Xyce object.
      // This somewhat convoluted syntax is needed to stop p from
      // pointing at the same address as the VPI system task.
      void** p = (void **) malloc( sizeof(void* [1]) );

      // Turn the desired Xyce command line invocation into an int and
      // char** pointer that can used to initialize an N_CIR_ Xyce object.
      // This is hard-coded for initial testing purposes.
      char *argList[] = {
	  (char*)("Xyce"),
          (char*)("-quiet"),
          (char*)("ADCtest.cir")
      };
      int argc = sizeof(argList)/sizeof(argList[0]);
      char** argv = argList;

      xyce_open(p);
      xyce_initialize(p,argc,argv);

      // Get the number of YADC in the netlist, and the maximum name length
      // This allows for better sizing of the ADCnames char arrays.
      int numADCnames, maxADCnameLength;
      int* numADCnamesPtr = &numADCnames;
      int* maxADCnameLengthPtr = &maxADCnameLength;
      xyce_getNumDevices(p, (char *)"YADC", numADCnamesPtr, maxADCnameLengthPtr);
      printf("Num ADCs and Max ADC Name Length: %d %d\n",numADCnames,maxADCnameLength);

      // Initialize arrays of char array
      char ** ADCnames;

      int i, j, k; // loop counters
      ADCnames = (char **) malloc( numADCnames * sizeof(char*));
      for (i = 0; i < numADCnames; i++)
      {
        ADCnames[i] = (char *) malloc( (maxADCnameLength+1)*sizeof(char) );
      }

      // This next set of calls is mostly to allow valgrind coverage of the
      // get/set functions for ADCs.

      // Get all of the ADC parameters
      int widths[2];
      double resistances[2], upperVLimits[2], lowerVLimits[2], settlingTimes[2];
      int* widthsPtr = widths;
      double* Rptr = resistances;
      double* UVLptr = upperVLimits;
      double* LVLptr = lowerVLimits;
      double* STptr = settlingTimes;
      xyce_getADCMap(p, numADCnamesPtr, ADCnames, widthsPtr, Rptr, UVLptr, LVLptr, STptr);

      for (i =0; i < numADCnames; i++)
      {
        printf("ADC %s has width=%d, R=%.2e, UVL=%.2e, LVL=%.2e, ST=%.2e\n",
	       ADCnames[i], widths[i], resistances[i], upperVLimits[i], lowerVLimits[i], settlingTimes[i]);
      }

      // Set ADC widths.  This is hard-coded for two ADCs.  The ordering in
      // ADCnames is "YADC!ADC1" and "YADC!ADC2".
      widths[0]=2;
      widths[1]=3;
      xyce_setADCWidths(p, numADCnames, ADCnames, widthsPtr);

      // Re-get ADC widths as just set, via a different function
      xyce_getADCWidths(p, numADCnames, ADCnames, widthsPtr);
      printf("Widths are: ");
      for (i = 0; i < numADCnames; i++)
      {
	printf("%d ", widths[i]);
      }
      printf("\n");

      int status, numPoints;
      int* numPointsPtr = &numPoints;
      double stepSize = 1e-5;
      double requested_time, actual_time;

      double* actual_time_ptr = &actual_time;
      double** timeArray;
      double** voltageArray;
      int maxNumberOfTimeVals = 25;
      timeArray = (double **) malloc(numADCnames * sizeof(double*));
      voltageArray = (double **) malloc(numADCnames * sizeof(double*));
      for (i=0; i<2; i++)
      {
        timeArray[i] = (double *) malloc( maxNumberOfTimeVals*sizeof(double));
        voltageArray[i] = (double *) malloc( maxNumberOfTimeVals*sizeof(double));
      }

      FILE *fptr;
      fptr = fopen("ADCtest.cir.TVarrayData","w" );

      for (i=0; i<10; i++)
      {
        requested_time = 0.0 + (i+1) * stepSize;
        printf( "Calling simulateUntil for requested time %f\n", requested_time );
        status = xyce_simulateUntil(p, requested_time, actual_time_ptr );
        printf( "Return status from simulateUntil = %d and actual_time = %f\n",status, actual_time);

        status = xyce_getTimeVoltagePairsADC(p, numADCnamesPtr, ADCnames, numPointsPtr, timeArray, voltageArray);
        int adcNum;
        for ( adcNum=0; adcNum<numADCnames; adcNum++ )
        {
          int prev=-1;
          int curr=0;
          for( j=0; j<numPoints; j++ )
          {
            if( fabs( timeArray[adcNum][j] - actual_time ) < 1.0e-11 )
            {
              // found end time of this ADC's time array
              curr = j;
              prev = j-1;
              break; 
            }
          } 
          // output for inspection 
          if( prev >= 0 )
          {
            printf( "ADC %d: Time and voltage array %d values are %.3e %.3e\n", (adcNum+1), prev, timeArray[adcNum][prev], voltageArray[adcNum][prev] );
            fprintf( fptr, "ADC %d: Time and voltage array %d values are %.3e %.3e\n", (adcNum+1), prev, timeArray[adcNum][prev], voltageArray[adcNum][prev] );
          }
          printf( "ADC %d: Time and voltage array %d values are %.3e %.3e\n", (adcNum+1), curr, timeArray[adcNum][curr], voltageArray[adcNum][curr] );
          fprintf( fptr, "ADC %d: Time and voltage array %d values are %.3e %.3e\n", (adcNum+1), curr, timeArray[adcNum][curr], voltageArray[adcNum][curr] );
        }

        // zero out the arrays, before getting their values for the next iteration.
        for (j=0; j<numADCnames; j++)
        {
          for (k=0; k<maxNumberOfTimeVals; k++)
	  {
            timeArray[j][k]=0.0;
            voltageArray[j][k]=0;
	  }
        }
      }

      xyce_close(p);

      // pointer clean-up
      free(p);
      fclose(fptr);

      for (i = 0; i < numADCnames; i++)
      {
        free( ADCnames[i] );
        free( voltageArray[i] );
        free( timeArray[i] );
      }
      free( ADCnames );
      free( voltageArray );
      free( timeArray );

      vpi_printf("Exiting calltf for ADCtest\n");
      return 0;
}

void ADCtest_register()
{
      s_vpi_systf_data tf_data;

      tf_data.type      = vpiSysTask;
      tf_data.tfname    = "$ADCtest";
      tf_data.calltf    = ADCtest_calltf;
      tf_data.compiletf = ADCtest_compiletf;
      tf_data.sizetf    = 0;
      tf_data.user_data = 0;
      vpi_register_systf(&tf_data);
}

void (*vlog_startup_routines[])() = {
    ADCtest_register,
    0 /* final entry must be zero */
};
