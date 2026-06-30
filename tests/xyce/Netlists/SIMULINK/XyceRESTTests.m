%
% These are units tests written in Matlab script
% designed to test the Xyce REST API as exposed by
% the python code in XyceRest.py
%

% This will need to be modified for a given test configuration for now

%
% functions tested
% . xyce_open
% . xyce_close
% . xyce_initialize
% . xyce_run 
% . xyce_getsimtime
% . xyce_getfinaltime 
% . xyce_simulateuntil
% . xyce_getdacnames
% . xyce_getadcmap
% . xyce_checkcircuitparamexists
% . xyce_getcircuitvalue
% . xyce_setcircuitparameter
% . xyce_gettimevoltagepairs
% xyce_updatetimevoltagepairs
% 
%


classdef XyceRESTTests < matlab.unittest.TestCase
  methods(Test)
  
    %
    % Test responding
    % 
    function testServiceResponding(testCase)
      status = webreadnoproxy("http://localhost:5000/test" );
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      testCase.verifyEqual(status.Body.Data, "Active and responding.");
    end
    
    %
    % Test status
    % 
    function testServerStatus(testCase)
      status = webreadnoproxy("http://localhost:5000/status" );
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      testCase.verifyEqual(status.Body.Data.numInstance, 0);
    end

    %
    % Test xyce_open and xyce_close 
    %
    function testServerXyceOpenClose(testCase)
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    %
    % Test xyce_initialize
    %
    function testServerXyceInitialize(testCase)
      %wo=weboptions('MediaType', 'application/json');
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist1.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % close this xyce object
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    %
    % Test xyce_run
    %
    function testServerXyceRun(testCase)
      %wo=weboptions('MediaType', 'application/json');
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist1.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % now try running a circuit.
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_run", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    
    %
    % Test xyce_getstimtime 
    %
    function testServerXyceGetSimTime(testCase)
      %wo=weboptions('MediaType', 'application/json');
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist1.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % now try running a circuit.
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_run", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % call getsimtime 
      % now try running a circuit.
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getsimtime", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      testCase.verifyEqual(status.Body.Data.time, 1.0);
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    %
    % Test xyce_finaltime 
    %
    function testServerXyceFinalSimTime(testCase)
      %wo=weboptions('MediaType', 'application/json');
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist1.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % call getfinaltime 
      % now try running a circuit.
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getfinaltime", jsarg3);
      testCase.verifyEqual(status.Body.Data.time, 1.0);
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    
    %
    % Test xyce_simulateuntil 
    %
    function testServerXyceSimulateUntil(testCase)
      %wo=weboptions('MediaType', 'application/json');
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist2.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % call getfinaltime 
      % now try running a circuit.
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getfinaltime", jsarg3);
      finalTime = status.Body.Data.time;
      testCase.verifyEqual(finalTime, 1.0);
      
      numSteps = 20;
      deltaTime = finalTime / numSteps;
      for i = 1:numSteps
        % now try running a circuit.
        s3.uuid=xyceID;
        s3.simtime = [i * deltaTime];
        jsarg3=jsonencode(s3);
        status = webwritenoproxy("http://localhost:5000/xyce_simulateuntil", jsarg3);
        %print(retcode)
        %testCase.verifyEqual(retcode.simulatedTime, (i * deltaTime), "AbsTol", 1e-6);
      end
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    
    %
    % Test xyce_getdacnames 
    %
    function testServerXyceGetDACNames(testCase)
      %wo=weboptions('MediaType', 'application/json');
      s.stats=0;
      jsarg=jsonencode(s);
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist3.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      %testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % call xyce_getdacnames 
      % now try running a circuit.
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getdacnames", jsarg3);
      testCase.verifyEqual(status.Body.Data.status, 1.0);
      testCase.verifyEqual(size(status.Body.Data.dacNames,1), 1);
      testCase.verifyEqual(status.Body.Data.dacNames{1}, 'YDAC!DAC_DRIVER1');
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    %
    % Test xyce_getadcmap 
    %
    function testServerXyceGetADCMap(testCase)
      %wo=weboptions('MediaType', 'application/json');
      s.stats=0;
      jsarg=jsonencode(s);
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist3.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % call xyce_getadcmap 
      % now try running a circuit.
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getadcmap", jsarg3);
      testCase.verifyEqual(status.Body.Data.status, 1.0);
      testCase.verifyEqual(size(status.Body.Data.ADCnames,1), 2);
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    
    %
    % Test xyce_checkcircuitparamexists 
    %
    function testServerXyceCheckCircuitParameterExists(testCase)
      %wo=weboptions('MediaType', 'application/json');
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist2.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      s3.uuid=xyceID;
      s3.paramname ='TOMP';
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_checkcircuitparamexists", jsarg3);
      testCase.verifyEqual( status.Body.Data.result, 0)
      s3.uuid=xyceID;
      s3.paramname ='TEMP';
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_checkcircuitparamexists", jsarg3);
      testCase.verifyEqual( status.Body.Data.result, 1)
      s3.uuid=xyceID;
      s3.paramname ='R1:R';
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_checkcircuitparamexists", jsarg3);
      testCase.verifyEqual( status.Body.Data.result, 1)
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    %
    % Test xyce_getcircuitvalue 
    %
    function testServerXyceGetCirciutValue(testCase)
      %wo=weboptions('MediaType', 'application/json');
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist2.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      s3.uuid=xyceID;
      s3.paramname ='R1:R';
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getcircuitvalue", jsarg3);
      testCase.verifyEqual( status.Body.Data.value, 1)
      s3.uuid=xyceID;
      s3.paramname ='TEMP';
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getcircuitvalue", jsarg3);
      testCase.verifyEqual( status.Body.Data.value, 27)
      s3.uuid=xyceID;
      s3.paramname ='V(1)';
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getcircuitvalue", jsarg3);
      testCase.verifyEqual( status.Body.Data.value, 0)
      
      
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    %
    % Test xyce_getcircuitvalue and xyce_setcircuitparameter
    %
    function testServerXyceGetAndSetCirciutValue(testCase)
      %wo=weboptions('MediaType', 'application/json');
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist2.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      %testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      s3.uuid=xyceID;
      s3.paramname ='R1:R';
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getcircuitvalue", jsarg3);
      %testCase.verifyEqual( status.Body.Data.value, 1)
      s3.paramname ='TEMP';
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getcircuitvalue", jsarg3);
      %testCase.verifyEqual( status.Body.Data.value, 27)
      s3.paramname ='V(1)';
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getcircuitvalue", jsarg3);
      %testCase.verifyEqual( status.Body.Data.value, 0)
      
      % call getfinaltime 
      % now try running a circuit.
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getfinaltime", jsarg3);
      %testCase.verifyEqual(status.Body.Data.time, 1.0);
      
      numSteps = 20;
      deltaTime = status.Body.Data.time / numSteps;
      for i = 1:numSteps
        % now try running a circuit.
        s3.uuid=xyceID;
        s3.simtime = [i * deltaTime];
        jsarg3=jsonencode(s3);
        status = webwritenoproxy("http://localhost:5000/xyce_simulateuntil", jsarg3);
        %print(retcode)
        %testCase.verifyEqual(status.Body.Data.simulatedTime, (i * deltaTime), "AbsTol", 1e-6);
        if( i==(numSteps/2))
          % in the middle of the run change the resistance on resistor R1
          s4.uuid=xyceID;
          s4.paramname ='R1:R';
          s4.paramval =[1000.0];
          jsarg4=jsonencode(s4);
          status = webwritenoproxy("http://localhost:5000/xyce_setcircuitparameter", jsarg4);
          testCase.verifyEqual( status.Body.Data.status, 1)
          s4.paramname ='TEMP';
          s4.paramval =[-50.0];
          jsarg4=jsonencode(s4);
          status = webwritenoproxy("http://localhost:5000/xyce_setcircuitparameter", jsarg4);
          testCase.verifyEqual( status.Body.Data.status, 1)
        end
        if( i==((numSteps/2)+1))
          % check that param is still set. 
          s3.uuid=xyceID;
          s3.paramname ='R1:R';
          jsarg3=jsonencode(s3);
          status = webwritenoproxy("http://localhost:5000/xyce_getcircuitvalue", jsarg3);
          testCase.verifyEqual( status.Body.Data.value, 1000)
          s3.paramname ='TEMP';
          jsarg3=jsonencode(s3);
          status = webwritenoproxy("http://localhost:5000/xyce_getcircuitvalue", jsarg3);
          testCase.verifyEqual( status.Body.Data.value, -50)
        end 
      end
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    %
    % Test xyce_gettimevoltagepairs
    %
    function testServerXyceGetTimeVoltagePairs(testCase)
      %wo=weboptions('MediaType', 'application/json');
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist3.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      %testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % call getfinaltime 
      % now try running a circuit.
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getfinaltime", jsarg3);
      %testCase.verifyEqual(status.Body.Data.time, 1.0);
      
      numSteps = 20;
      deltaTime = status.Body.Data.time / numSteps;
      for i = 1:numSteps
        % now try running a circuit.
        s3.uuid=xyceID;
        s3.simtime = [i * deltaTime];
        jsarg3=jsonencode(s3);
        status = webwritenoproxy("http://localhost:5000/xyce_simulateuntil", jsarg3);
        testCase.verifyEqual(status.Body.Data.simulatedTime, (i * deltaTime), "AbsTol", 1e-6);
        
        status = webwritenoproxy("http://localhost:5000/xyce_gettimevoltagepairs", jsarg3);
        testCase.verifyEqual(status.Body.Data.status, 1)
        testCase.verifyEqual(size(status.Body.Data.ADCnames,1), 2)
        % Xyce can return 1 or 2 points in the array
        testCase.verifyEqual(status.Body.Data.numPointsInArray, [1; 1], "AbsTol", 1)
        testCase.verifyEqual(size(status.Body.Data.timeArray,1),status.Body.Data.numADCnames*status.Body.Data.numPointsInArray(2)) 
        testCase.verifyEqual(size(status.Body.Data.voltageArray,1),status.Body.Data.numADCnames*status.Body.Data.numPointsInArray(2))
      end
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
    
    %
    % Test xyce_updatetimevoltagepairs
    %
    function testServerXyceUpdateTimeVoltagePairs(testCase)
      %wo=weboptions('MediaType', 'application/json');
      jsarg=jsonencode(' ');
      status = webwritenoproxy("http://localhost:5000/xyce_open", jsarg);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      xyceID = status.Body.Data.uuid;
      testCase.verifyNotEqual(xyceID, '');
      
      % call initialize 
      s2.uuid=xyceID;
      s2.simfile='TestNetlist3.cir';
      jsarg2=jsonencode(s2);
      status = webwritenoproxy("http://localhost:5000/xyce_initialize", jsarg2);
      %testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % call getfinaltime 
      % now try running a circuit.
      s3.uuid=xyceID;
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_getfinaltime", jsarg3);
      %testCase.verifyEqual(status.Body.Data.time, 1.0);
      
      numSteps = 2;
      deltaTime = status.Body.Data.time / numSteps;
      
      % now try running the circuit half way through transient circuit.
      s3.uuid=xyceID;
      s3.simtime = [ deltaTime];
      jsarg3=jsonencode(s3);
      status = webwritenoproxy("http://localhost:5000/xyce_simulateuntil", jsarg3);
      testCase.verifyEqual(status.Body.Data.simulatedTime, deltaTime, "AbsTol", 1e-6);
      simulatedTime = status.Body.Data.simulatedTime;
      
      % set the DAC 
      status = webwritenoproxy("http://localhost:5000/xyce_getdacnames", jsarg3);
      testCase.verifyEqual(status.Body.Data.status, 1.0);
      testCase.verifyEqual(size(status.Body.Data.dacNames,1), 1);
      dacName = status.Body.Data.dacNames{1};
      
      timeArray = [simulatedTime + 1e-7; simulatedTime + 1e-3];
      dacVData = [0.5; 0.5];
      s4.uuid=xyceID;
      s4.devname = dacName;
      s4.timearray = timeArray;
      s4.voltarray = dacVData;
      jsarg4=jsonencode(s4);
      status = webwritenoproxy("http://localhost:5000/xyce_updatetimevoltagepairs", jsarg4);
    
      % run to the end of the transient simulation
      status = webwritenoproxy("http://localhost:5000/xyce_run", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
      
      % verify the DAC change propagated in the circuit 
      s5.uuid=xyceID;
      s5.paramname ='V(N1)';
      jsarg5=jsonencode(s5);
      status = webwritenoproxy("http://localhost:5000/xyce_getcircuitvalue", jsarg5);
      testCase.verifyEqual( status.Body.Data.value, 0.5)
      
      % close this xyce object
      status = webwritenoproxy("http://localhost:5000/xyce_close", jsarg3);
      testCase.verifyEqual(status.StatusCode, matlab.net.http.StatusCode(200));
    end
    
  end
end 