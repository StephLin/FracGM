import numpy as np
from scipy.optimize import minimize_scalar

class FracGM:
    """ 
    FracGM solver for the optimization problem:
        min x^2 / (x^2 + 1),
    where the optimal solution is x* = 0.
    """
    def __init__(self, max_iteration=1000, tolerance=1e-7, verbose=True):
        
        self.max_iter = max_iteration
        self.tol = tolerance 
        self.verbose = verbose

    @staticmethod
    def f(x):
        return x**2
    
    @staticmethod
    def h(x):
        return x**2 + 1

    def solve_beta_mu(self, x):
        
        beta = self.f(x)/self.h(x)
        mu = 1/self.h(x)
        
        return beta, mu

    def solve_x(self, beta, mu):
        
        def dual_problem(x):
            return mu*(self.f(x) - beta*self.h(x)) 
        
        # Scipy minimizer for scalar function of one variable
        res = minimize_scalar(dual_problem)
        
        return res.x

    def compute_psi_norm(self, beta, mu, x):
        
        psi_1 = -self.f(x) + beta*self.h(x)
        psi_2 = -1 + mu*self.h(x)
        
        return np.linalg.norm([psi_1, psi_2])

    def solve(self, initial):

        if self.verbose:
            print("initial guess:", initial)

        x = initial

        for i in range(self.max_iter):  
            beta, mu = self.solve_beta_mu(x)
            x = self.solve_x(beta, mu)

            norm = self.compute_psi_norm(beta, mu, x)
            if norm < self.tol:
                break

        if self.verbose:
            print("solution:", x)
            
        return x