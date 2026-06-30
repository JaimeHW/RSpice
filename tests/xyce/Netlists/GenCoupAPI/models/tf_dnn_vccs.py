import numpy as np
from BaseDevice import BaseDevice
from TFModel import TFModel
import os


class Device(BaseDevice):
    
    def processPythonParams(self, b_params, d_params, i_params, s_params):
        p_params = {}
        p_params['nodePos']   = 0
        p_params['nodeNeg']   = 1
        p_params['controlNodePos'] = 2
        p_params['controlNodeNeg']  = 3
        p_params['TFModelFileName'] = os.path.dirname(os.path.abspath(__file__)) + "/" + "./DNN/vccs_network_double.h5"
        self.pythonParamsMerge(b_params, d_params, i_params, s_params, p_params)
    
    def getArraySizes(self, b_params, d_params, i_params, s_params):
        sizes_dict = super().getArraySizes(b_params, d_params, i_params, s_params)
        sizes_dict['B']=[0,]
        sizes_dict['Q']=[0,]
        sizes_dict['dQdX']=[0,]
        return sizes_dict
    
    def getJacStampSize(self, b_params, d_params, i_params, s_params):
        # minimum implementation is to return a size 0 numpy array of ints
    
        nodePos   = i_params['nodePos']
        nodeNeg   = i_params['nodeNeg']
    
        # There are four variables, the two external and two internal:
        row_sizes = np.zeros(shape=(4,),dtype='i4')
        row_sizes[nodePos]   = 2
        row_sizes[nodeNeg]   = 2
    
        return row_sizes
    
    
    def setJacStamp(self, jacStamp, b_params, d_params, i_params, s_params):
    
        nodePos   = i_params['nodePos']
        nodeNeg   = i_params['nodeNeg']
        controlNodePos = i_params['controlNodePos']
        controlNodeNeg = i_params['controlNodeNeg']
    
        jacStamp[nodePos][0] = controlNodePos;
        jacStamp[nodePos][1] = controlNodeNeg;
        jacStamp[nodeNeg][0] = controlNodePos;
        jacStamp[nodeNeg][1] = controlNodeNeg;
    
        return 1
    
    def computeXyceVectors(self, fSV, solV, stoV, staV, deviceOptions, solverState,
            origFlag, F, Q, B, dFdX, dQdX, dFdXdVp, dQdXdVp, 
            b_params, d_params, i_params, s_params):
    
        # get nextSolutionVariables which is index 0 of solV
        solV = solV[0]
        nodePos   = i_params['nodePos']
        nodeNeg   = i_params['nodeNeg']
        controlNodePos = i_params['controlNodePos']
        controlNodeNeg = i_params['controlNodeNeg']
    
        # solV, F, Q, and B are memory views
        # cast them to numpy arrays without copying data
        np_solV = np.array(solV, dtype=np.float64, copy=False)
        np_F  = np.array( F, dtype=np.float64, copy=False)
        np_Q  = np.array( Q, dtype=np.float64, copy=False)
        np_B  = np.array( B, dtype=np.float64, copy=False)
    
        np_dFdX = [np.array(item, dtype=np.float64, copy=False) for item in dFdX]
        np_dQdX = [np.array(item, dtype=np.float64, copy=False) for item in dQdX]
    
        transConductance_ = d_params['TRANSCONDUCTANCE']
    
        numVars=np_solV.shape[0]
        indepVars = np.zeros(shape=(numVars,),dtype=np.float64)
        Fcontribs = np.zeros(shape=(numVars,),dtype=np.float64)
        for i in range(numVars):
            indepVars[i]=np_solV[i];
    
        eps = 1e-3
        current = transConductance_*(indepVars[controlNodePos]-indepVars[controlNodeNeg])

        v_diff = indepVars[controlNodePos]-indepVars[controlNodeNeg]
        np_v_diff = np.reshape(np.array([v_diff,], dtype=np.float64), newshape=(1,1))
        
        computed_current = self.tf_model.predict(np_v_diff)
        assert abs(abs(current)-abs(computed_current))<eps, "Current (%.16f) and calculated current (%.16f) differ by more than machine precision" % (current,computed_current)
        Fcontribs[nodePos] = computed_current
        Fcontribs[nodeNeg] = -computed_current
        #Fcontribs[nodePos] = current
        #Fcontribs[nodeNeg] = -current
    
        for i in range(numVars):
            F[i] = Fcontribs[i]
    
        dFdX[nodePos][nodePos] = 0
        dFdX[nodePos][nodeNeg] = 0
        computed_transConductance_ = self.tf_model.gradient(np_v_diff)
        assert abs(transConductance_-computed_transConductance_)<eps, "transConductance_ (%.16f) and calculated transConductance_ (%.16f) differ by more than machine precision" % (transConductance_,computed_transConductance_)
        dFdX[nodePos][controlNodePos] = computed_transConductance_
        #dFdX[nodePos][controlNodePos] = transConductance_
        dFdX[nodePos][controlNodeNeg] = -computed_transConductance_
        #dFdX[nodePos][controlNodeNeg] = -transConductance_
        dFdX[nodeNeg][nodePos] = 0
        dFdX[nodeNeg][nodeNeg] = 0
        dFdX[nodeNeg][controlNodePos] = -computed_transConductance_
        #dFdX[nodeNeg][controlNodePos] = -transConductance_
        dFdX[nodeNeg][controlNodeNeg] = computed_transConductance_
        #dFdX[nodeNeg][controlNodeNeg] = transConductance_
        dFdX[controlNodePos][nodePos] = 0.0
        dFdX[controlNodePos][nodeNeg] = 0.0
        dFdX[controlNodePos][controlNodePos] = 0.0
        dFdX[controlNodePos][controlNodeNeg] = 0.0
        dFdX[controlNodeNeg][nodePos] = 0.0
        dFdX[controlNodeNeg][nodeNeg] = 0.0
        dFdX[controlNodeNeg][controlNodePos] = 0.0
        dFdX[controlNodeNeg][controlNodeNeg] = 0.0
        return 1
    
    def initialize(self, deviceOptions, solverState, b_params, d_params, i_params, s_params):
        # setup Tensorflow DNN
        self.tf_model = TFModel(s_params["TFModelFileName"])
    

