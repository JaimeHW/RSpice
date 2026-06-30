import numpy as np
from KokkosDevice import KokkosDevice
from GMLS import GMLS
import DeviceSupport

DEBUG=False

class Device(KokkosDevice):

    def __del__(self):
        # destroys all Kokkos objects
        del self.gmls
        # potentially finalizes Kokkos
        super().__del__()

    def updateTemperature(self, b_params, d_params, i_params, s_params):
        # model updates 
        # limit grading coeff
        if( d_params["M"] > 0.9 ): d_params["M"] = 0.9

        # limit activation energy
        if( d_params["EG"] < 0.1 ): d_params["EG"] = 0.1

        # limit depl cap coeff
        if( d_params["FC"] > 0.95 ): d_params["FC"] = 0.95

        if( d_params["RS"]==0.0 ):
            d_params["COND"] = 0.0
        else:
            d_params["COND"] = 1.0/d_params["RS"];

        xfc = np.log(1.0-d_params["FC"]);
        F2 = np.exp((1.0+d_params["M"])*xfc);
        F3 = 1.0-d_params["FC"]*(1.0+d_params["M"]);

        #instance updates
        #if (!given("LAMBERTW"))
        #  lambertWFlag = getDeviceOptions().lambertWFlag;
        #if (!given("TEMP"))
        #  Temp = getDeviceOptions().temp.getImmutableValue<double>();
        if ( (d_params["RS"] == 0.0) and (i_params["lambertWFlag"] == 1) ):
          d_params["RS"] =1.0e-12;

        #update temp
        CONSTQ = 1.6021918e-19
        CONSTboltz = 1.3806226e-23
        CONSTKoverQ=CONSTboltz/CONSTQ

        CONSTEg300=1.1150877
        CONSTEg0=1.16
        CONSTbetaEg=1108.0
        CONSTREFTEMP=300.15
        CONSTalphaEg=7.02e-4
        CONSTroot2 = np.sqrt(2)

        Temp = CONSTREFTEMP

        vtnom = CONSTKoverQ * d_params["TNOM"]

        xfc = np.log( 1.0 - d_params["FC"] );

        #if( temp != -999.0 ) Temp = temp;
        TNOM = d_params["TNOM"]

        vt = CONSTKoverQ * Temp;
        fact2 = Temp / CONSTREFTEMP;
        egfet = CONSTEg0 - (CONSTalphaEg*Temp*Temp)/(Temp+CONSTbetaEg);
        arg = -egfet/(2.0*CONSTboltz*Temp) + \
                     CONSTEg300/(CONSTboltz*(CONSTREFTEMP+CONSTREFTEMP));
        pbfact = -2.0*vt*(1.5*np.log(fact2)+CONSTQ*arg);
        egfet1 = CONSTEg0 - (CONSTalphaEg*d_params["TNOM"]*d_params["TNOM"])/ \
                        (d_params["TNOM"]+CONSTbetaEg);
        arg1 = -egfet1/(2.0*CONSTboltz*d_params["TNOM"]) + \
              CONSTEg300/(2.0*CONSTboltz*CONSTREFTEMP);
        fact1 = d_params["TNOM"]/CONSTREFTEMP;
        pbfact1 = -2.0*vtnom*(1.5*np.log(fact1)+CONSTQ*arg1);

        pbo = (d_params["VJ"]-pbfact1)/fact1;
        gmaold = (d_params["VJ"]-pbo)/pbo;

        tJctCap = d_params["CJO"]/ \
                  (1.0+d_params["M"]*(4.0e-4*(d_params["TNOM"]-CONSTREFTEMP) -gmaold));

        tJctPot = pbfact+fact2*pbo;
        gmanew = (tJctPot-pbo)/pbo;

        tJctCap *= 1.0 + d_params["M"]*(4.0e-4*(Temp-CONSTREFTEMP)-gmanew);

        tSatCur = d_params["IS"]*np.exp(((Temp/d_params["TNOM"])-1.0)* \
                                d_params["EG"]/(d_params["N"]*vt)+ \
                                d_params["XTI"]/d_params["N"]*np.log(Temp/d_params["TNOM"]));

        tF1 = tJctPot*(1.0-np.exp((1.0-d_params["M"])*xfc))/(1.0-d_params["M"]);

        tDepCap = d_params["FC"]*tJctPot;

        vte = d_params["N"]*vt;
        tVcrit = vte*np.log(vte/(CONSTroot2*tSatCur));

        tRS   = d_params["RS"];
        tCOND = d_params["COND"];
        #if (model_.IRFGiven)
        #{
        #  tIRF  = model_.IRF*pow(fact2,1.6);
        #}
        #else
        #{
        tIRF = d_params["IRF"];
        #}
        #int level = model_.getLevel();
        #if(level == 2)   // this section is PSPICE compatible
        #{
        #  tSatCurR = model_.ISR*exp((Temp/TNOM - 1.0)*
        #                            model_.EG/(model_.NR*vt)+
        #                            model_.XTI/model_.NR*log(Temp/TNOM));

        #  tIKF = model_.IKF*(1 + model_.TIKF*(Temp-TNOM));

        #  tempBV = model_.BV*(1 + (Temp-TNOM)*
        #                      ( model_.TBV1 + model_.TBV2*(Temp-TNOM) ));

        #  tRS = model_.RS*(1 + (Temp-TNOM)*
        #                   ( model_.TRS1 + model_.TRS2*(Temp-TNOM) ));

        #  tCOND = 0.0;
        #  if(tRS != 0.0) tCOND = 1.0/tRS;

        #  tJctPot = (model_.VJ - egfet1)*fact2 - 3*vt*log(fact2) + egfet;

        #  tJctCap = model_.CJO/(1.0 +
        #                        model_.M*(4.0e-4*(Temp-TNOM) + (1-tJctPot/model_.VJ)));
        #}
        #else
        #{
        tempBV=d_params["BV"];
        #}

        #if( model_.BVGiven)
        #{
        #    double IRFactor=tIRF;
        #    double cbv = model_.IBV;
        #    double xbv;
        #    double cthreshlow; //lower threshold for IBV
        #    double cthreshhigh; //high threshold for IBV
        #    int iter_count;
        #    const int ITER_COUNT_MAX=8;
        #    double arg2;

        #    double arg1=3.0*vte/(CONSTe*tempBV);
        #    arg1=arg1*arg1*arg1;
        #    cthreshlow=tSatCur*IRFactor*(1-arg1);
        #    cthreshhigh=tSatCur*IRFactor*(1-1.0/(CONSTe*CONSTe*CONSTe)) *
        #                exp(-1.0*(3.0*vte-tempBV)/vte);

        #    if(cbv >= cthreshhigh)
        #    {                     //if IBV is too high, tBrkdwnV will go below 3NVt.
        #      tBrkdwnV=3.0*vte;   //Clip tBrkdwnV to 3*N*Vt in this case (and hence
        #    }                     //clip IBV to cthreshhigh).


        #    else if(cbv <= cthreshlow)
        #    {                          //if IBV is too low, tBrkdwnV will go above
        #      tBrkdwnV=tempBV;  //BV.  Clip tBrkdwnV to BV in this case (and
        #    }                          //hence clip IBV to cthreshlow).
        #    //If IBV is in an acceptable range, perform a Picard iteration to find
        #    //tBrkdwnV, starting with an initial guess of tBrkdwnV=tempBV, and
        #    //running through the iteration ITER_COUNT_MAX times.

        #    else
        #    {
        #      xbv=tempBV;
        #      for(iter_count=0; iter_count < ITER_COUNT_MAX; iter_count++)
        #      {
        #        arg2=3.0*vte/(CONSTe*xbv);
        #        arg2=arg2*arg2*arg2;
        #        xbv=tempBV-vte*log(cbv/(tSatCur*IRFactor))+vte *
        #            log(1-arg2);
        #      }
        #      tBrkdwnV=xbv;
        #    }
        #}
        DEBUG and print(" IS = ", d_params["IS"])
        DEBUG and print(" vtnom   = ", vtnom)
        DEBUG and print(" xfc     = ", xfc)
        DEBUG and print(" TNOM    = ", TNOM)
        DEBUG and print(" vt      = ", vt)
        DEBUG and print(" fact2   = ", fact2)
        DEBUG and print(" egfet   = ", egfet)
        DEBUG and print(" arg     = ", arg)
        DEBUG and print(" pbfact  = ", pbfact)
        DEBUG and print(" egfet1  = ", egfet1)
        DEBUG and print(" arg1    = ", arg1)
        DEBUG and print(" fact1   = ", fact1)
        DEBUG and print(" pbfact1 = ", pbfact1)
        DEBUG and print(" pbo     = ", pbo)
        DEBUG and print(" gmaold  = ", gmaold)
        DEBUG and print(" gmanew  = ", gmanew)
        DEBUG and print(" tJctCap = ", tJctCap)
        DEBUG and print(" tJctPot = ", tJctPot)
        DEBUG and print(" tSatCur = ", tSatCur)
        DEBUG and print(" tF1     = ", tF1)
        DEBUG and print(" tDepCap = ", tDepCap)
        DEBUG and print(" vte     = ", vte)
        DEBUG and print(" tempBV  = ", tempBV)
        DEBUG and print(" tVcrit  = ", tVcrit)
        DEBUG and print(" tRS     = ", tRS)
        DEBUG and print(" tCOND   = ", tCOND)
        DEBUG and print(" tIRF    = ", tIRF)
        DEBUG and print(" tBrkdwnV= ", d_params["tBrkdwnV"])
        d_params["Temp"] =  Temp
        d_params["tJctCap"]= tJctCap
        d_params["tJctPot"]= tJctPot
        d_params["tDepCap"]= tDepCap
        d_params["tF1"]=     tF1
        #d_params["tSatCur"]= tSatCur
        d_params["tVcrit"]=  tVcrit
        d_params["tRS"]=     tRS
        d_params["tCOND"]=   tCOND
        d_params["tIRF"]=   tIRF

    def processPythonParams(self, b_params, d_params, i_params, s_params):
        # minimum implementation is to return an empty dictionary
        p_params = {}
        # instance params
        p_params["off"]=False
        p_params["Area"]=1.0
        p_params["InitCond"]=0.0
        p_params["Temp"]=300.15
        p_params["lambertWFlag"]=0
        p_params["InitCondGiven"]=False
        p_params["tJctPot"]=0.0
        p_params["tJctCap"]=0.0
        p_params["tDepCap"]=0.0
        #p_params["tSatCur"]=0.0
        p_params["tVcrit"]=0.0
        p_params["tF1"]=0.0
        p_params["tBrkdwnV"]=0.0
        p_params["tRS"]=0.0
        p_params["tIRF"]=1.0
        #p_params["Vd_old"]=0.0
        #p_params["Vd_orig"]=0.0
        #p_params["li_storevd"]=-1
        #p_params["li_branch_data"]=-1

        ## are these the same in each circuit?
        #p_params["li_Pos"]=2
        #p_params["li_Neg"]=5
        #p_params["li_Pri"]=2

        #p_params["APosEquPosNodeOffset"]=0
        #p_params["APosEquPriNodeOffset"]=-1
        #p_params["ANegEquNegNodeOffset"]=-1
        #p_params["ANegEquPriNodeOffset"]=-1
        #p_params["APriEquPosNodeOffset"]=0
        #p_params["APriEquNegNodeOffset"]=-1
        #p_params["APriEquPriNodeOffset"]=0

        #p_params["numIntVars"]=1
        #p_params["numExtVars"]=2
        #p_params["numStateVars"]=0
        #p_params["setNumStoreVars"]=1
        #p_params["setNumBranchDataVars"]=0
        #p_params["numBranchDataVarsIfAllocated"]=1
    
    
        # model params
        p_params["IS"]=1.0e-14
        p_params["RS"]=0.0
        p_params["COND"]=0.0
        p_params["N"]=1.0
        p_params["ISR"]=0.0
        p_params["NR"]=2.0
        p_params["IKF"]=0.0
        p_params["TT"]=0.0
        p_params["CJO"]=0.0
        p_params["VJ"]=1.0
        p_params["M"]=0.5
        p_params["EG"]=1.11
        p_params["XTI"]=3.0
        p_params["TIKF"]=0.0
        p_params["TBV1"]=0.0
        p_params["TBV2"]=0.0
        p_params["TRS1"]=0.0
        p_params["TRS2"]=0.0
        p_params["FC"]=0.5
        p_params["BV"]=1e99
        p_params["IBV"]=1.0e-10
        p_params["IRF"]=1.0
        p_params["NBV"]=1.0
        p_params["IBVL"]=0.0
        p_params["NBVL"]=1.0
        p_params["F2"]=0.0
        p_params["F3"]=0.0
        p_params["TNOM"]=300.15
        p_params["KF"]=0.0
        p_params["AF"]=1.0
        p_params["BVGiven"]=False
        p_params["IRFGiven"]=False
    
        # from DeviceOptions
        p_params["gmin"]=1.0e-12
    
        # moves parameters from p_params -> (b_params, d_params, i_params, s_params)
        # if they are not already defined there
        self.pythonParamsMerge(b_params, d_params, i_params, s_params, p_params)
        self.updateTemperature(b_params, d_params, i_params, s_params)

    def processNumInternalVars(self, b_params, d_params, i_params, s_params):
        if d_params["RS"]==0.0 or i_params["lambertWFlag"]==1:
            i_params["numInternalVars"]=0
        else:
            i_params["numInternalVars"]=1
        return i_params["numInternalVars"]
    
    def processNumStateVars(self, b_params, d_params, i_params, s_params):
        if d_params["CJO"]==0.0:
            i_params["numStateVars"]=1
        else:
            i_params["numStateVars"]=0
        return i_params["numStateVars"]
    
    def processNumStoreVars(self, b_params, d_params, i_params, s_params):
        i_params["numStoreVars"]=1
        return i_params["numStoreVars"] 

    #def processNumBranchDataVarsIfAllocated(self, b_params, d_params, i_params, s_params):
    #    # we just return number of external variables
    
    def getArraySizes(self, b_params, d_params, i_params, s_params):
        sizes_dict = super().getArraySizes(b_params, d_params, i_params, s_params)
        sizes_dict['B']=[0,]
        return sizes_dict
    
    def getJacStampSize(self, b_params, d_params, i_params, s_params):
        # minimum implementation is to return a size 0 numpy array of ints
        # There are four variables, the two external and two internal:
        row_sizes = np.zeros(shape=(0,),dtype='i4')
        #return row_sizes
        #return 1
        #row_sizes = np.zeros(shape=(3,),dtype='i4')
        #row_sizes[0] = 2
        #row_sizes[1] = 2
        #row_sizes[2] = 3
        return row_sizes

    def setJacStamp(self, jacStamp, b_params, d_params, i_params, s_params):
        #jacStamp[0][0]=0
        #jacStamp[0][1]=2
        #jacStamp[1][0]=1
        #jacStamp[1][1]=2
        #jacStamp[2][0]=0
        #jacStamp[2][1]=1
        #jacStamp[2][2]=2
    
        #jacMap_RS.clear();
        #jacStampMap(jacStamp_RS, jacMap_RS, jacMap2_RS,
        #            jacStamp,    jacMap, jacMap2, 2, 0, 3);
        return 1
    
    def computeXyceVectors(self, fSV, solV, stoV, staV, deviceOptions, solverState,
            origFlag, F, Q, B, dFdX, dQdX, dFdXdVp, dQdXdVp, 
            b_params, d_params, i_params, s_params):

        # get nextSolutionVariables which is index 0 of solV
        solV = solV[0]
        # solV, F, Q, and B are memory views
        # cast them to numpy arrays without copying data
        np_solV = np.array(solV, dtype=np.float64, copy=False)
        np_F  = np.array( F, dtype=np.float64, copy=False)
        np_Q  = np.array( Q, dtype=np.float64, copy=False)
        np_B  = np.array( B, dtype=np.float64, copy=False)
    
        np_dFdX = [np.array(item, dtype=np.float64, copy=False) for item in dFdX]
        np_dQdX = [np.array(item, dtype=np.float64, copy=False) for item in dQdX]
        np_stoV = [np.array(item, dtype=np.float64, copy=False) for item in stoV]
    

        numVars=2 # would be np_solV.shape[0], but Vd uniquely determines both Vpp and Vn
        indepVars2D = np.zeros(shape=(1,numVars,),dtype=np.float64)
        Fcontribs = np.zeros(shape=(numVars,),dtype=np.float64)
        indepVars2D[0,0] = np_solV[0] - np_solV[1]

        Vd = indepVars2D[0,0]

        DEBUG and print("Vpp: ", Vd)
        DEBUG and print("Vn: ", 0)
        DEBUG and print("Vp: ", Vd)

        ##
        ## LIMITING
        ## 
        Vd_orig = Vd
        origFlag[0] = True
        
        tVcrit = d_params["tVcrit"]
        # Setup initial junction conditions if UIC enabled
        #------------------------------------------------
        if solverState.newtonIter == 0:
            if solverState.initJctFlag_ and deviceOptions.voltageLimiterFlag:
                if b_params["InitCondGiven"]:
                    Vd = d_params["InitCond"]
                    origFlag = False
                elif b_params["off"]:
                    Vd = 0.0
                    origFlag[0] = False
                else:
                    if solverState.inputOPFlag:
                        if (fSV[0]==0 or fSV[1]==0):
                            Vd=tVcrit
                            origFlag[0] = False
                    else:
                        Vd=tVcrit
                        origFlag[0] = False
            # assume there is no history -- then check if the
            # state vector can overwrite this
            DEBUG and deviceOptions.voltageLimiterFlag and print("Vd in limiting: ", Vd)
            Vd_old = Vd

            if (not solverState.dcopFlag or (solverState.locaEnabledFlag and solverState.dcopFlag)):
                Vd_old = np_stoV[1][0] # [currStoVectorRawPtr,li_storevd]
        else:  # just do this whenever it isn't the first iteration
            Vd_old = np_stoV[0][0] # [nextStoVectorRawPtr,li_storevd]
        DEBUG and deviceOptions.voltageLimiterFlag and print("Vd_old in limiting: ", Vd_old)

        ## 
        ## LIMITING
        ## 
        CONSTQ = 1.6021918e-19
        CONSTboltz = 1.3806226e-23
        CONSTKoverQ=CONSTboltz/CONSTQ
        Temp = d_params["Temp"]
        N = d_params["N"]
        Vt = CONSTKoverQ * Temp
        Vte = N * Vt
        if deviceOptions.voltageLimiterFlag:
            if (solverState.newtonIter >= 0):
                # removed breakdown check
                (Vd, ichk) = DeviceSupport.pnjlim(Vd, Vd_old, Vte, tVcrit)
                if ichk==1: origFlag[0] = False
        indepVars2D[0,0] = Vd
    
        # only called for comparison
        (Fcontribs2D, dFdXcontribs3D) = self.generateData(indepVars2D[:,0], b_params, d_params, i_params, s_params)

        Fcontribs[0] = self.gmls.predict(np.reshape(np.array([indepVars2D[0,0]], dtype=np.float64), newshape=(1,1)), self.data_F[:,0])
        Fcontribs[1] = self.gmls.predict(np.reshape(np.array([indepVars2D[0,0]], dtype=np.float64), newshape=(1,1)), self.data_F[:,1])
        for i in range(numVars):
            F[i]=Fcontribs[i]
        #    F[i]=Fcontribs2D[0,i]
        #    print("gmlsF(%d):"%i,Fcontribs2D[0,i]," vs ","compF(%d):"%i,Fcontribs[i])

    
        dFdXcontribs = np.zeros(shape=(numVars,numVars),dtype=np.float64)
        #dFdXcontribs[0][0] = self.gmls.gradient(np.reshape(np.array([indepVars[0], indepVars[1]], dtype=np.float64), newshape=(1,2)), self.data_F[:,0], 0)
        #dFdXcontribs[0][1] = self.gmls.gradient(np.reshape(np.array([indepVars[0], indepVars[1]], dtype=np.float64), newshape=(1,2)), self.data_F[:,0], 1)
        #dFdXcontribs[1][0] = self.gmls.gradient(np.reshape(np.array([indepVars[0], indepVars[1]], dtype=np.float64), newshape=(1,2)), self.data_F[:,1], 0)
        #dFdXcontribs[1][1] = self.gmls.gradient(np.reshape(np.array([indepVars[0], indepVars[1]], dtype=np.float64), newshape=(1,2)), self.data_F[:,1], 1)
        dFdXcontribs[0][0] = self.gmls.gradient(np.reshape(np.array([indepVars2D[0,0],], dtype=np.float64), newshape=(1,1)), self.data_F[:,0], 0)
        dFdXcontribs[0][1] = self.gmls.gradient(np.reshape(np.array([indepVars2D[0,0],], dtype=np.float64), newshape=(1,1)), self.data_F[:,0], 1)
        dFdXcontribs[1][0] = self.gmls.gradient(np.reshape(np.array([indepVars2D[0,0],], dtype=np.float64), newshape=(1,1)), self.data_F[:,1], 0)
        dFdXcontribs[1][1] = self.gmls.gradient(np.reshape(np.array([indepVars2D[0,0],], dtype=np.float64), newshape=(1,1)), self.data_F[:,1], 1)



        for i in range(numVars):
            for j in range(numVars):
                #np_dQdX[i][j]=dQdXcontribs[i][j]
                np_dFdX[i][j]=dFdXcontribs[i][j]
                #np_dFdX[i][j]=dFdXcontribs3D[0][i][j]
                DEBUG and print("compdFdx(%d,%d):"%(i,j),dFdXcontribs3D[0,i,j]," vs ","gmlsdFdX(%d,%d):"%(i,j),dFdXcontribs[i,j])

        
        # LIMITING
        
        # load the voltage limiter vector.
        Cd = 0
        Gd = dFdX[0][0]
        if deviceOptions.voltageLimiterFlag:
            np_dFdXdVp  = np.array( dFdXdVp, dtype=np.float64, copy=False)
            np_dQdXdVp  = np.array( dQdXdVp, dtype=np.float64, copy=False)
            dFdXdVpcontribs = np.zeros(shape=(numVars,),dtype=np.float64)
            dQdXdVpcontribs = np.zeros(shape=(numVars,),dtype=np.float64)
            Vd_diff = Vd - Vd_orig;
            Cd_Jdxp = -( Cd ) * Vd_diff;
            Gd_Jdxp = -( Gd ) * Vd_diff;
            DEBUG and print("Cd_Jdxp: ", Cd_Jdxp);
            DEBUG and print("Gd_Jdxp: ", Gd_Jdxp);
    
            # Load the dFdxdVp vector
            dFdXdVpcontribs[1] += Gd_Jdxp;
            dFdXdVpcontribs[0] -= Gd_Jdxp;
            # dQdxdVp vector
            dQdXdVpcontribs[1] += Cd_Jdxp;
            dQdXdVpcontribs[0] -= Cd_Jdxp;

            for i in range(2):
                np_dFdXdVp[i] = dFdXdVpcontribs[i]
                np_dQdXdVp[i] = dQdXdVpcontribs[i]

        if deviceOptions.voltageLimiterFlag:
            np_stoV[0][0] = Vd # [nextStoVectorRawPtr, li_stovd]

        return 1

    def initialize(self, deviceOptions, solverState, b_params, d_params, i_params, s_params):
        # control node voltages sent in
        # index 0->2
        # index 1->3
        xv = np.linspace(-5,5,5000)
        #xv, yv = np.meshgrid(np.linspace(-5,5,500), np.linspace(-5,5,500))
        #all_sV = np.vstack((xv.flatten(), yv.flatten()))
        #all_sV = all_sV.T

        # this voltage is just the voltage drop vpp-vn
        all_sV = xv
        (self.data_F, self.data_dFdX) = self.generateData(all_sV, b_params, d_params, i_params, s_params)
        #self.data = self.generateData(all_sV, b_params, d_params, i_params, s_params)

        # setup GMLS problem on inputs
        DEBUG and print(np.atleast_2d(all_sV).T.shape)
        self.gmls = GMLS(source_sites=np.atleast_2d(all_sV).T, polynomial_order=5, weighting_power=2, epsilon_multiplier=1.7)
    
    def generateData(self, all_sV, b_params, d_params, i_params, s_params):
    
        #global DEBUG

        numVoltages=all_sV.shape[0]
        #if (numVoltages>1): DEBUG=False
        numVars=2 # would be all_sV.shape[1], but Vd uniquely determine Vpp and Vn
        indepVars = all_sV

        # Fortran ordering of data to ensure continuous columns when performing GMLS remap
        Fcontribs = np.zeros(shape=(numVoltages,numVars),dtype=np.float64,order='F')
        dFdXcontribs = np.zeros(shape=(numVoltages,numVars,numVars),dtype=np.float64)

        #def normal_exponential(Vd,tIRF,Isat,Vte,CONSTe,gmin,tBrkdwnV):
        gmin = d_params["gmin"]
        CONSTMAX_EXP_ARG=100.0
        CONSTQ = 1.6021918e-19
        CONSTboltz = 1.3806226e-23
        CONSTKoverQ=CONSTboltz/CONSTQ
        CONSTe = np.exp(1.0)

        Temp = d_params["Temp"]
        N = d_params["N"]
    
        # Adjustment for linear portion of reverse current
        #Vd = d_params["Vd"]

        # rather than use indepVars[:,0] ("Vpp") and indepVars[:,1] ("Vn"), we store ("Vd") in indepVars[:,0] and
        # know that Vn is always 0
        Vpp = indepVars[:]
        Vn  = 0*indepVars[:]#indepVars[:,1]
        Vp  = indepVars[:]
        DEBUG and print("Vpp: ", Vpp[0])
        DEBUG and print("Vn: ", Vn[0])
        DEBUG and print("Vp: ", Vp[0])
        Vd = Vpp - Vn


        Gspr = d_params["tCOND"] * d_params["Area"];

        Vt = CONSTKoverQ * Temp
        DEBUG and print("Vt: ", Vt)
        Vte = N * Vt
        tIRF = d_params["tIRF"]
        tBrkdwnV = d_params["tBrkdwnV"]

    
        #IRfactor = 1 if (Vd >= -3.0 * Vte) else tIRF
        IRfactor = np.where(Vd >= -3.0 * Vte, 1.0, tIRF)
        tSatCur = d_params["IS"]*np.exp(((Temp/d_params["TNOM"])-1.0)*
                                d_params["EG"]/(d_params["N"]*Vt)+
                                d_params["XTI"]/d_params["N"]*np.log(Temp/d_params["TNOM"]));
    
        Isat  = tSatCur * d_params["Area"]

        Isat *= IRfactor;
        DEBUG and print("Isat: ", Isat[0])

        ### if (Vd >= -3.0 * Vte):
        arg1 = np.divide(Vd,Vte);
        arg1 = np.where(CONSTMAX_EXP_ARG<arg1, CONSTMAX_EXP_ARG, arg1);
        evd = np.exp(arg1);
        case1 = np.where(Vd >= -3.0 * Vte, True, False)
        case2 = np.where(np.logical_or(np.where(tBrkdwnV==0,1,0),np.where((Vd >= -tBrkdwnV),1,0)), True, False)
    
        case1Id = Isat * (evd - 1.0) + gmin * Vd;
        case1Gd = Isat * evd / Vte + gmin;
    
        ### elif ((not tBrkdwnV) or (Vd >= -tBrkdwnV)):
        #arg = 3.0 * Vte / (Vd * CONSTe);
        arg = np.divide(3.0 * Vte, Vd * CONSTe, out=np.zeros_like(Vd), where=Vd*CONSTe!=0)
        arg = arg * arg * arg;
        case2Id = -Isat * (1.0 + arg) + gmin * Vd;
        #case2Gd = Isat * 3.0 * arg / Vd + gmin;
        case2Gd = np.divide(Isat * 3.0 * arg, Vd, out=np.zeros_like(Vd), where=Vd!=0) + gmin
        DEBUG and print("case1:", case1[0], "case1Gd:", case1Gd[0])
        DEBUG and print("case2:", case2[0], "case2Gd:", case2Gd[0])

    
        #### else:
        #arg1 = -(tBrkdwnV + Vd) / Vte;
        #arg1 = np.where(CONSTMAX_EXP_ARG<arg1, CONSTMAX_EXP_ARG, arg1);
        #evrev = np.exp(arg1);
    
        #arg2=3.0*Vte/(CONSTe*tBrkdwnV);
        #arg2=arg2*arg2*arg2;
        #Isat_tBrkdwnV=Isat*(1-arg2);
        #case3Id = -Isat_tBrkdwnV * evrev + gmin * Vd;
        #case3Gd = Isat_tBrkdwnV * evrev / Vte + gmin;

        # npwhere on case1 is logical not
        Id = np.copy(case1*case1Id + np.where(case1, False, True)*(case2)*case2Id)# + (not case1)*(not case2)*case3Id
        #DEBUG and print("Id:", Id[0])
        Gd = np.copy(case1*case1Gd + np.where(case1, False, True)*(case2)*case2Gd)# + (not case1)*(not case2)*case3Gd
    
        DEBUG and print("Gd:", Gd[0])
        DEBUG and print("Id:", Id[0])
        DEBUG and print("Vd:", Vd[0])
        DEBUG and print("Vte:", Vte)
        #assert False, "end here"
        ### DEBUG and print("Vte:", Vte)
    
        ### # load F
        Ir = Gspr * (Vp - Vpp)
        #DEBUG and print("Id:", Id[0])
    
        ### # load in the KCL for the positive node:
        coef = -np.copy(Ir)
        Fcontribs[:,0] = np.copy(coef)
        #DEBUG and print("Id:", Id[0])

        coef = np.copy(Id)
        Fcontribs[:,1] = -np.copy(coef)
        #DEBUG and print("Id:", Id[0])
    
        # load in the KCL for the positive prime node:
        coef *= -1;
        #DEBUG and print("Id:", Id[0])
        coef += Ir;
        #DEBUG and print("Id:", Id[0])
        #Fcontribs[li_Pri] -= coef;
        Fcontribs[:,0] -= np.copy(coef)
        #DEBUG and print("Id:", Id[0])
        DEBUG and print("Normal exponential regime.")
        DEBUG and print("Vd  = ",Vd[0])
        DEBUG and print("Vte = ",Vte)
        DEBUG and print("Id  = ",Id[0])
        DEBUG and print("Gd  = ",Gd[0])
        DEBUG and print("Cd  = ",0)
        DEBUG and print("F[0]:",Fcontribs[0,0])
        DEBUG and print("F[1]:",Fcontribs[0,1])
        DEBUG and print("F[2]:",Fcontribs[0,0])

        #TODO remove all after this
        dFdXcontribs = np.zeros(shape=(numVoltages,numVars,numVars),dtype=np.float64)
        #DEBUG and print("dFdx[0][0]=",dFdXcontribs[0][0])
        dFdXcontribs[:,0,0] += Gspr;
        #DEBUG and print("dFdx[0][0]=",dFdXcontribs[0][0])
        dFdXcontribs[:,0,0] -= Gspr;
        #DEBUG and print("dFdx[0][2]=",dFdXcontribs[0][0])

        dFdXcontribs[:,1,1] += Gd;
        #DEBUG and print("dFdx[1][1]=",dFdXcontribs[1][1])
        dFdXcontribs[:,1,0] -= Gd;
        #DEBUG and print("dFdx[1][2]=",dFdXcontribs[1][0])

        dFdXcontribs[:,0,0] -= Gspr;
        #DEBUG and print("dFdx[2][0]=",dFdXcontribs[0][0])
        dFdXcontribs[:,0,1] -= Gd;
        #DEBUG and print("dFdx[2][1]=",dFdXcontribs[0][1])
        dFdXcontribs[:,0,0] += Gspr + Gd;
        #DEBUG and print("dFdx[2][2]=",dFdXcontribs[0][0])
        for i in range(numVars):
            for j in range(numVars):
                DEBUG and print("dFdX[%d][%d]"%(i,j),dFdXcontribs[0,i,j])

        #if (numVoltages>1): DEBUG=True

        return (Fcontribs, dFdXcontribs)
        #return Fcontribs#(Fcontribs, dFdXcontribs)

