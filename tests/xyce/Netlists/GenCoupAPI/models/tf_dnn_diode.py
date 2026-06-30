import numpy as np
from BaseDevice import BaseDevice
from TFModel import TFModel
import DeviceSupport
import os

DEBUG=False

class Device(BaseDevice):

    def processPythonParams(self, b_params, d_params, i_params, s_params):
        # minimum implementation is to return an empty dictionary
        p_params = {}
        p_params['TFModelFileName'] = os.path.dirname(os.path.abspath(__file__)) + "/" + "./DNN/diode_double_network_1N4148.h5"
        self.pythonParamsMerge(b_params, d_params, i_params, s_params, p_params)

    def processNumInternalVars(self, b_params, d_params, i_params, s_params):
        i_params["numInternalVars"]=0
        return i_params["numInternalVars"]
    
    def processNumStateVars(self, b_params, d_params, i_params, s_params):
        i_params["numStateVars"]=1
        return i_params["numStateVars"]
    
    def processNumStoreVars(self, b_params, d_params, i_params, s_params):
        i_params["numStoreVars"]=1
        return i_params["numStoreVars"] 
    
    def getArraySizes(self, b_params, d_params, i_params, s_params):
        sizes_dict = super().getArraySizes(b_params, d_params, i_params, s_params)
        sizes_dict['B']=[0,]
        return sizes_dict
    
    def getJacStampSize(self, b_params, d_params, i_params, s_params):
        # minimum implementation is to return a size 0 numpy array of ints
        # There are four variables, the two external and two internal:
        row_sizes = np.zeros(shape=(0,),dtype='i4')
        return row_sizes

    def setJacStamp(self, jacStamp, b_params, d_params, i_params, s_params):
        return 1
    
    def computeXyceVectors(self, fSV, solV, stoV, staV, deviceOptions, solverState,
            origFlag, F, Q, B, dFdX, dQdX, dFdXdVp, dQdXdVp, 
            b_params, d_params, i_params, s_params):

        # get nextSolutionVariables which is index 0 of solV
        solV=solV[0]
        # solV, F, Q, and B are memory views
        # cast them to numpy arrays without copying data
        np_solV = np.array(solV, dtype=np.float64, copy=False)
        np_stoV = np.array(stoV, dtype=np.float64, copy=False)
        np_F  = np.array( F, dtype=np.float64, copy=False)
        np_Q  = np.array( Q, dtype=np.float64, copy=False)
        np_B  = np.array( B, dtype=np.float64, copy=False)
    
        np_dFdX = [np.array(item, dtype=np.float64, copy=False) for item in dFdX]
        np_dQdX = [np.array(item, dtype=np.float64, copy=False) for item in dQdX]
        np_stoV = [np.array(item, dtype=np.float64, copy=False) for item in stoV]
    

        numVars=2 # would be np_solV.shape[0], but Vd uniquely determines both Vpp and Vn
        indepVars2D = np.zeros(shape=(1,numVars,),dtype=np.float64)
        Fcontribs = np.zeros(shape=(numVars,),dtype=np.float64)
        indepVars2D[0,0]=np_solV[0];

        Vd = indepVars2D[0,0]

        DEBUG and print("Vpp: ", Vd)
        DEBUG and print("Vn: ", 0)
        DEBUG and print("Vp: ", Vd)

        ##
        ## LIMITING
        ## 
        Vd_orig = Vd
        origFlag[0] = True
        
        #tVcrit = d_params["tVcrit"]
        ## Setup initial junction conditions if UIC enabled
        ##------------------------------------------------
        #if newtonIter == 0:
        #    if initJctFlag and voltageLimiterFlag:
        #        if b_params["InitCondGiven"]:
        #            Vd = d_params["InitCond"]
        #            origFlag = False
        #        elif b_params["off"]:
        #            Vd = 0.0
        #            origFlag[0] = False
        #        else:
        #            if inputOPFlag:
        #                if (fSV[0]==0 or fSV[1]==0):
        #                    Vd=tVcrit
        #                    origFlag[0] = False
        #            else:
        #                Vd=tVcrit
        #                origFlag[0] = False
        #    # assume there is no history -- then check if the
        #    # state vector can overwrite this
        #    DEBUG and voltageLimiterFlag and print("Vd in limiting: ", Vd)
        #    Vd_old = Vd

        #    if (not dcopFlag or (locaEnabledFlag and dcopFlag)):
        #        Vd_old = np_stoV[1][0] # [currStoVectorRawPtr,li_storevd]
        #else:  # just do this whenever it isn't the first iteration
        #    Vd_old = np_stoV[0][0] # [nextStoVectorRawPtr,li_storevd]
        #DEBUG and voltageLimiterFlag and print("Vd_old in limiting: ", Vd_old)

        ### 
        ### LIMITING
        ### 
        #CONSTQ = 1.6021918e-19
        #CONSTboltz = 1.3806226e-23
        #CONSTKoverQ=CONSTboltz/CONSTQ
        #Temp = d_params["Temp"]
        #N = d_params["N"]
        #Vt = CONSTKoverQ * Temp
        #Vte = N * Vt
        #if voltageLimiterFlag:
        #    if (newtonIter >= 0):
        #        # removed breakdown check
        #        (Vd, ichk) = DeviceSupport.pnjlim(Vd, Vd_old, Vte, tVcrit)
        #        if ichk==1: origFlag[0] = False
        #indepVars2D[0,0] = Vd
    
        ## only called for comparison
        #(Fcontribs2D, dFdXcontribs3D) = self.generateData(indepVars2D[:,0], b_params, d_params, i_params, s_params)

        v_diff = indepVars2D[0,0]
        np_v_diff = np.reshape(np.array([v_diff,], dtype=np.float64), newshape=(1,1))
        F0 = self.tf_model.predict(np_v_diff) 

        Fcontribs[0] = F0
        Fcontribs[1] = -F0
        for i in range(numVars):
            F[i]=Fcontribs[i]
        #    F[i]=Fcontribs2D[0,i]
        #    print("gmlsF(%d):"%i,Fcontribs2D[0,i]," vs ","compF(%d):"%i,Fcontribs[i])

    
        dFdXcontribs = np.zeros(shape=(numVars,numVars),dtype=np.float64)
        dF00 = self.tf_model.gradient(np_v_diff)
        dFdXcontribs[0][0] = dF00
        dFdXcontribs[1][0] = -dF00

        for i in range(numVars):
            for j in range(numVars):
                #np_dQdX[i][j]=dQdXcontribs[i][j]
                np_dFdX[i][j]=dFdXcontribs[i][j]
                #np_dFdX[i][j]=dFdXcontribs3D[0][i][j]
        #        DEBUG and print("compdFdx(%d,%d):"%(i,j),dFdXcontribs3D[0,i,j]," vs ","gmlsdFdX(%d,%d):"%(i,j),dFdXcontribs[i,j])

        
        ### LIMITING
        ##
        ### load the voltage limiter vector.
        ##Cd = 0
        ##Gd = dFdX[0][0]
        ##if(b_params["voltageLimiterFlag"]):
        ##    np_dFdXdVp  = np.array( dFdXdVp, dtype=np.float64, copy=False)
        ##    np_dQdXdVp  = np.array( dQdXdVp, dtype=np.float64, copy=False)
        ##    dFdXdVpcontribs = np.zeros(shape=(numVars,),dtype=np.float64)
        ##    dQdXdVpcontribs = np.zeros(shape=(numVars,),dtype=np.float64)
        ##    Vd_diff = Vd - Vd_orig;
        ##    Cd_Jdxp = -( Cd ) * Vd_diff;
        ##    Gd_Jdxp = -( Gd ) * Vd_diff;
        ##    DEBUG and print("Cd_Jdxp: ", Cd_Jdxp);
        ##    DEBUG and print("Gd_Jdxp: ", Gd_Jdxp);
    
        ##    # Load the dFdxdVp vector
        ##    dFdXdVpcontribs[1] += Gd_Jdxp;
        ##    dFdXdVpcontribs[0] -= Gd_Jdxp;
        ##    # dQdxdVp vector
        ##    dQdXdVpcontribs[1] += Cd_Jdxp;
        ##    dQdXdVpcontribs[0] -= Cd_Jdxp;

        ##    for i in range(2):
        ##        np_dFdXdVp[i] = dFdXdVpcontribs[i]
        ##        np_dQdXdVp[i] = dQdXdVpcontribs[i]

        ##if b_params["voltageLimiterFlag"]:
        ##    np_stoV[0][0] = Vd # [nextStoVectorRawPtr, li_stovd]

        return 1

    def initialize(self, deviceOptions, solverState, b_params, d_params, i_params, s_params):
        # setup Tensorflow DNN
        self.tf_model = TFModel(s_params["TFModelFileName"])
