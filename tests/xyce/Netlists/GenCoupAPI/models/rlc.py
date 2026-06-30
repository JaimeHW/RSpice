import numpy as np
from BaseDevice import BaseDevice

class Device(BaseDevice):

    def processNumInternalVars(self, b_params, d_params, i_params, s_params):
        i_params["numInternalVars"] = 2
        return i_params["numInternalVars"]
    
    def processPythonParams(self, b_params, d_params, i_params, s_params):
        # minimum implementation is to return an empty dictionary
        p_params = {}
        p_params['node1']   = 0
        p_params['node2']   = 1
        p_params['nodeInt'] = 2
        p_params['branch']  = 3
        p_params['testtwo']  = False
        self.pythonParamsMerge(b_params, d_params, i_params, s_params, p_params)
        # p_params are inserted into b_params, d_params, i_params, and s_params,
        # if they key is not already defined (if it is defined, it is from the Netlist)
    
    def getArraySizes(self, b_params, d_params, i_params, s_params):
        sizes_dict = super().getArraySizes(b_params, d_params, i_params, s_params)
        sizes_dict['B']=[0,]
        return sizes_dict
    
    def getJacStampSize(self, b_params, d_params, i_params, s_params):
        # minimum implementation is to return a size 0 numpy array of ints
    
        node1   = i_params['node1']
        node2   = i_params['node2']
        nodeInt = i_params['nodeInt']
        branch  = i_params['branch']
    
        # There are four variables, the two external and two internal:
        row_sizes = np.zeros(shape=(4,),dtype='i4')
        row_sizes[node1]   = 1
        row_sizes[node2]   = 2
        row_sizes[nodeInt] = 3
        row_sizes[branch]  = 3
    
        return row_sizes
    
    
    def setJacStamp(self, jacStamp, b_params, d_params, i_params, s_params):
    
        node1   = i_params['node1']
        node2   = i_params['node2']
        nodeInt = i_params['nodeInt']
        branch  = i_params['branch']
    
        # The first variable is the positive external node.  The only
        # contribution to this is from the branch current, so we only
        # have one entry on this row
        jacStamp[node1][0] = branch;
    
        # The second variable is the negative external node.  The equation
        # for this variable depends on the node 2 value and also the internal
        # node value.  We set the sparsity pattern accordingly.
        jacStamp[node2][0]=node2;
        jacStamp[node2][1]=nodeInt;
    
        # The third variable is the internal node.  The equation for this
        # variable depends on the node 2 value and the internal node
        # value, as well as the branch current.  We set the sparsity
        # pattern accordingly.
        jacStamp[nodeInt][0]=node2;
        jacStamp[nodeInt][1]=nodeInt;
        jacStamp[nodeInt][2]=branch;
    
        # The fourth variable is the branch current.  The equation for this
        # variable depends on the node 1 value and the internal node
        # value, as well as the branch current.  We set the sparsity
        # pattern accordingly.
        jacStamp[branch][0]=node1;
        jacStamp[branch][1]=nodeInt;
        jacStamp[branch][2]=branch;
    
        return 1
    
    def computeXyceVectors(self, fSV, solV, stoV, staV, deviceOptions, solverState,
            origFlag, F, Q, B, dFdX, dQdX, dFdXdVp, dQdXdVp, 
            b_params, d_params, i_params, s_params):
    
        # get nextSolutionVariables which is index 0 of solV
        solV = solV[0]
        node1   = i_params['node1']
        node2   = i_params['node2']
        nodeInt = i_params['nodeInt']
        branch  = i_params['branch']
        R_  = d_params['R']
        L_  = d_params['L']
        C_  = d_params['C']
    
        # solV, F, Q, and B are memory views
        # cast them to numpy arrays without copying data
        np_solV = np.array(solV, dtype=np.float64, copy=False)
        np_F  = np.array( F, dtype=np.float64, copy=False)
        np_Q  = np.array( Q, dtype=np.float64, copy=False)
        np_B  = np.array( B, dtype=np.float64, copy=False)
    
        np_dFdX = [np.array(item, dtype=np.float64, copy=False) for item in dFdX]
        np_dQdX = [np.array(item, dtype=np.float64, copy=False) for item in dQdX]
    
        numVars=np_solV.shape[0]
    
        indepVars = np.zeros(shape=(numVars,),dtype=np.float64)
        Fcontribs = np.zeros(shape=(numVars,),dtype=np.float64)
        Qcontribs = np.zeros(shape=(numVars,),dtype=np.float64)
        for i in range(numVars):
            indepVars[i]=np_solV[i];
    
        # vars 0 and 1 are the external nodes
        # var 2 is the internal node
        # var 3 is the branch current
        # equation 0 and 1 are the equations for the external node
        # equation 2 is the equation for the internal node
        # equation 3 is the branch equation
    
        # Do the resistor and inductor as a single branch with just one
        # internal node (c.f rlc2.va in Xyce/utils/ADMS/examples/toys)
        # Branch equation is then
        #    (branch_current*R + L d(branch_current)/dt)-(v0-v2) = 0
        # Current between external node 0 and internal node 2 is just
        # the branch current
        Fcontribs[node1] = indepVars[branch]
        Fcontribs[nodeInt] = -indepVars[branch]
    
        Fcontribs[branch] = indepVars[branch]*R_ - (indepVars[node1]-indepVars[nodeInt])
        Qcontribs[branch] = L_*indepVars[branch]
    
    
        # Now the capacitor.  Current flowing from node 2 to node 1 is
        # just dQ/dt where Q=CV.
        capCharge = C_*(indepVars[nodeInt]-indepVars[node2])
        Qcontribs[nodeInt] = capCharge
        Qcontribs[node2] = -capCharge
    
        for i in range(numVars):
            F[i] = Fcontribs[i]
            Q[i] = Qcontribs[i]
    
        dFdX[node1][node1] = 0;
        dFdX[node1][node2] = 0;
        dFdX[node1][nodeInt] = 0;
        dFdX[node1][branch] = 1;
        dFdX[nodeInt][node1] = 0;
        dFdX[nodeInt][node2] = 0;
        dFdX[nodeInt][nodeInt] = 0;
        dFdX[nodeInt][branch] = -1;
        dFdX[branch][node1] = -1;
        dFdX[branch][node2] = 0;
        dFdX[branch][nodeInt] = 1;
        dFdX[branch][branch] = R_;
        dFdX[node2][node1] = 0;
        dFdX[node2][node2] = 0;
        dFdX[node2][nodeInt] = 0;
        dFdX[node2][branch] = 0;
    
        dQdX[node1][node1] = 0;
        dQdX[node1][node2] = 0;
        dQdX[node1][nodeInt] = 0;
        dQdX[node1][branch] = 0;
        dQdX[nodeInt][node1] = 0;
        dQdX[nodeInt][node2] = -C_;
        dQdX[nodeInt][nodeInt] = C_;
        dQdX[nodeInt][branch] = 0;
        dQdX[branch][node1] = 0;
        dQdX[branch][node2] = 0;
        dQdX[branch][nodeInt] = 0;
        dQdX[branch][branch] = L_;
        dQdX[node2][node1] = 0;
        dQdX[node2][node2] = C_;
        dQdX[node2][nodeInt] = -C_;
        dQdX[node2][branch] = 0;
    
        return 1
