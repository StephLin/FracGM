// Copyright 2024 the FracGM authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! This module contains the implementation of the solver for the
//! Geman-McClure-based objective function.

use ndarray::{Array2, Zip};

/// A trait for a type that can be used to compute the quadratic form associated
/// with a matrix, and keep track of the most recently computed value.
///
/// The quadratic form is typically defined as $f(x) = x^\top A x$. Some
/// possible speedup is possible by using the fact that $A$ is symmetric. See
/// R2Sym for sample implementations.
pub trait R2 {
    /// Compute the quadratic form associated with self and x.
    ///
    /// # Arguments
    ///
    /// - `x` - The input vector to compute the quadratic form.
    ///
    /// # Returns
    ///
    /// The value of the quadratic form.
    fn call(&self, x: &Array2<f64>) -> f64;

    /// Update the cached value of the quadratic form associated with self.
    ///
    /// # Arguments
    ///
    /// - `x` - The input vector to compute the quadratic form.
    fn update_cache(&mut self, x: &Array2<f64>);

    /// Get a reference to the matrix associated with self (a.k.a. $A$).
    ///
    /// # Returns
    ///
    /// A reference to the matrix associated with self.
    fn mat(&self) -> &Array2<f64>;

    /// Get the most recently computed value of the quadratic form associated
    /// with self (a.k.a. $f(x)$).
    ///
    /// # Returns
    ///
    /// The most recently computed value of the quadratic form associated with
    /// self.
    fn cache(&self) -> f64;
}

/// A struct that implements the R2 trait using a symmetric matrix.
///
/// The R2 trait is used to compute the quadratic form associated with a matrix
/// and a vector. The quadratic form is typically defined as $f(x) = x^\top A
/// x$. R2Sym supposes that the matrix $A$ is symmetric.
///
/// # Warning
///
/// The matrix $A$ is assumed to be symmetric. The quadratic form may be
/// incorrect if the matrix $A$ is not symmetric.
pub struct R2Sym {
    /// The matrix associated with self (a.k.a. $A$).
    pub mat: Array2<f64>,

    /// The most recently computed value of the quadratic form associated with
    /// self (a.k.a. $f(x)$).
    cache: f64,
}

impl R2Sym {
    /// Creates a new instance of `R2Sym` from a matrix.
    ///
    /// # Arguments
    ///
    /// - `mat` - The matrix associated with self (a.k.a. $A$).
    ///
    /// # Warning
    ///
    /// The matrix $A$ is assumed to be symmetric.
    pub fn new(mat: Array2<f64>) -> R2Sym {
        R2Sym { mat, cache: 0.0 }
    }
}

impl R2 for R2Sym {
    fn call(&self, x: &Array2<f64>) -> f64 {
        let mut result_upper = 0.0;
        let mut result_diag = 0.0;

        x.iter().enumerate().for_each(|(i, &x_i)| {
            self.mat
                .index_axis(ndarray::Axis(0), i)
                .iter()
                .enumerate()
                .for_each(|(j, &a_ij)| {
                    if i == j {
                        result_diag += x_i * a_ij * x_i;
                    } else if i < j {
                        result_upper += x_i * a_ij * x[[j, 0]];
                    }
                });
        });

        result_upper * 2.0 + result_diag
    }

    fn update_cache(&mut self, x: &Array2<f64>) {
        self.cache = self.call(x);
    }

    fn mat(&self) -> &Array2<f64> {
        &self.mat
    }

    fn cache(&self) -> f64 {
        self.cache
    }
}

/// A structure to represent a fractional term $f(x)/h(x)$ in the
/// Geman-McClure-based objective function.
pub struct Fractional<R> {
    /// The quadratic form associated to the square of residual.
    r2: R,
    /// The scalar $c$ (usually set to 1).
    c: f64,
    /// The matrix used to compute the numerator.
    f_mat: Array2<f64>,
}

impl<R: R2> Fractional<R> {
    /// Creates a new instance of `Fractional` representing $f(x)/h(x)$ in the
    /// Geman-McClure-based objective function.
    ///
    /// # Arguments
    ///
    /// - `r2` - The quadratic form associated to the square of residual.
    /// - `c` - The scalar $c$ (usually set to 1).
    ///
    /// # Returns
    ///
    /// A new instance of `Fractional` with the given parameters.
    pub fn new(r2: R, c: f64) -> Fractional<R> {
        let f_mat = c * c * r2.mat();

        Fractional { r2, c, f_mat }
    }

    /// Updates the cache of the square of residual.
    ///
    /// # Arguments
    ///
    /// - `x` - The input vector to compute the quadratic form.
    pub fn update_cache(&mut self, x: &Array2<f64>) {
        self.r2.update_cache(x);
    }

    /// Computes the numerator $f(x)$.
    ///
    /// # Returns
    ///
    /// The value of the numerator $f(x)$.
    pub fn f(&self) -> f64 {
        self.c * self.c * self.r2.cache()
    }

    /// Computes the denominator $h(x)$.
    ///
    /// # Returns
    ///
    /// The value of the denominator $h(x)$.
    pub fn h(&self) -> f64 {
        self.r2.cache() + self.c * self.c
    }

    /// Get the matrix associated with the numerator.
    ///
    /// # Returns
    ///
    /// A reference to the matrix associated with the numerator.
    pub fn f_mat(&self) -> &Array2<f64> {
        &self.f_mat
    }

    /// Get the matrix associated with the denominator.
    ///
    /// # Returns
    ///
    /// A reference to the matrix associated with the denominator.
    pub fn h_mat(&self) -> &Array2<f64> {
        self.r2.mat()
    }
}

/// A trait for a type that can be used to implement the FracGM solver.
///
/// This trait defines the methods that the FracGM solver needs to implement.
pub trait FractionalProgrammingMaterials<R: R2> {
    /// Get the dimension of the point clouds.
    ///
    /// # Returns
    ///
    /// The dimension of the point clouds.
    fn dim(&self) -> usize;

    /// Get the maximum number of iterations allowed.
    ///
    /// # Returns
    ///
    /// The maximum number of iterations allowed.
    fn max_iteration(&self) -> usize;

    /// Get the tolerance value for the stopping criteria.
    ///
    /// # Returns
    ///
    /// The tolerance value for the stopping criteria.
    fn tol(&self) -> f64;

    /// Get the value of $c$ defined in Geman-McClure robust function.
    ///
    /// # Returns
    ///
    /// The value of $c$.
    fn c(&self) -> f64;

    /// Convert an input matrix to a flattened vector.
    ///
    /// # Arguments
    ///
    /// - `mat` - The input matrix.
    ///
    /// # Returns
    ///
    /// The vector representation of the matrix.
    fn mat_to_vec(&self, mat: &Array2<f64>) -> Array2<f64>;

    /// Convert a flattened vector to a matrix.
    ///
    /// # Arguments
    ///
    /// - `vec` - The input vector.
    ///
    /// # Returns
    ///
    /// The matrix representation of the vector.
    fn vec_to_mat(&self, vec: &Array2<f64>) -> Array2<f64>;

    /// Project the solution to the original variable space.
    ///
    /// # Arguments
    ///
    /// - `mat` - The input matrix.
    ///
    /// # Returns
    ///
    /// The projected matrix.
    fn project(&self, mat: &Array2<f64>) -> Array2<f64>;

    /// Compute the Geman-McClure terms for the FracGM solver.
    ///
    /// # Arguments
    ///
    /// - `pc1` - The source point cloud.
    /// - `pc2` - The target point cloud.
    ///
    /// # Returns
    ///
    /// The computed Geman-McClure terms.
    fn compute_terms(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Vec<Fractional<R>>;

    /// Compute the initial guess of the solution. In general a regular least
    /// squares solution is acceptable.
    ///
    /// # Arguments
    ///
    /// - `pc1` - The source point cloud.
    /// - `pc2` - The target point cloud.
    ///
    /// # Returns
    ///
    /// The computed initial guess.
    fn compute_initial_guess(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64>;

    /// Solve the underlying convex optimization problem defined in the FracGM
    /// solver.
    ///
    /// # Arguments
    ///
    /// - `mat` - The input matrix.
    ///
    /// # Returns
    ///
    /// The solution of the convex optimization problem.
    fn solve_x(&self, mat: &Array2<f64>) -> Array2<f64>;

    /// Solve the underlying linear system for auxilary variables $\beta$ and
    /// $\mu$.
    ///
    /// # Arguments
    ///
    /// - `terms` - The computed terms.
    ///
    /// # Returns
    ///
    /// The solution of the linear system.
    fn solve_beta_mu(&self, terms: &Vec<Fractional<R>>) -> (Vec<f64>, Vec<f64>) {
        let beta = terms.iter().map(|term| term.f() / term.h()).collect();
        let mu = terms.iter().map(|term| 1.0 / term.h()).collect();

        (beta, mu)
    }

    /// Compute the norm of the vector $\psi$ defined in the FracGM solver. It
    /// is used to identify if the solution has converged.
    ///
    /// # Arguments
    ///
    /// - `beta` - The auxiliary variable $\beta$.
    /// - `mu` - The auxiliary variable $\mu$.
    /// - `terms` - The computed Geman-McClure terms.
    ///
    /// # Returns
    ///
    /// The norm of the vector $psi$.
    fn compute_psi_norm(&self, beta: &Vec<f64>, mu: &Vec<f64>, terms: &Vec<Fractional<R>>) -> f64 {
        assert!(beta.len() == mu.len());
        assert!(beta.len() == terms.len());

        beta.iter()
            .zip(terms.iter().zip(mu.iter()))
            .map(|(beta_, (term, mu_))| {
                let f = term.f();
                let h = term.h();

                let a = -1.0 * f + beta_ * h;
                let b = -1.0 + mu_ * h;
                a * a + b * b
            })
            .sum::<f64>()
            .sqrt()
    }

    /// Update the cache of the Geman-McClure terms.
    ///
    /// # Arguments
    ///
    /// - `terms` - The stored Geman-McClure terms.
    /// - `alpha` - The current solution of the optimization problem.
    fn update_terms_cache(&self, terms: &mut Vec<Fractional<R>>, alpha: &Array2<f64>) {
        terms.iter_mut().for_each(|term| term.update_cache(alpha));
    }
}

/// A trait that implements the FracGM solver for Geman-McClure-based
/// optimization problems.
pub trait GemanMcclureSolver<R: R2>: FractionalProgrammingMaterials<R> {
    /// Solve the Geman-McClure-based objective function with the FracGM solver.
    ///
    /// # Arguments
    ///
    /// - `pc1` - The source point cloud.
    /// - `pc2` - The target point cloud.
    ///
    /// # Returns
    ///
    /// The solution of the optimization problem.
    fn solve(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64> {
        assert!(
            pc1.shape() == pc2.shape(),
            "Input point clouds must have the same shape"
        );
        assert!(
            pc1.shape()[1] == 3,
            "Input point clouds must have 3 columns"
        );

        let mut terms = self.compute_terms(pc1, pc2);

        let init_mat = self.compute_initial_guess(pc1, pc2);
        let mut vec = self.mat_to_vec(&init_mat);
        self.update_terms_cache(&mut terms, &vec);

        let (mut beta, mut mu) = self.solve_beta_mu(&terms);

        for _ in 0..self.max_iteration() {
            let mut mat_a = Array2::<f64>::zeros((self.dim(), self.dim()));
            for i in 0..pc1.dim().0 {
                let mu_ = &mu[i];
                let beta_ = &beta[i];
                Zip::from(&mut mat_a)
                    .and(terms[i].f_mat())
                    .and(terms[i].h_mat())
                    .for_each(|a, f, h| {
                        *a += mu_ * f - mu_ * beta_ * h;
                    });
            }

            vec = self.solve_x(&mat_a);
            self.update_terms_cache(&mut terms, &vec);

            let psi_norm = self.compute_psi_norm(&beta, &mu, &terms);
            if psi_norm < self.tol() {
                break;
            }

            (beta, mu) = self.solve_beta_mu(&terms);

        }

        self.project(&self.vec_to_mat(&vec))
    }
}

/// Component of the diagnostic information.
#[derive(Clone)]
pub struct IterationComponent {
    /// The current solution in vector form.
    pub alpha_vec: Array2<f64>,
    /// The current solution in matrix form.
    pub alpha_mat: Array2<f64>,
    /// The current solution projected to the original variable space.
    pub alpha_proj: Array2<f64>,
    /// The current value of the auxiliary variable $\beta$.
    pub beta: Vec<f64>,
    /// The current value of the auxiliary variable $\mu$.
    pub mu: Vec<f64>,
    /// The norm of the vector $\psi$ which is used to check if the solution has
    /// converged.
    pub psi_norm: f64,
}

/// The diagnostic information returned by the FracGM solver.
#[derive(Clone)]
pub struct Diagnostic {
    /// The diagnostic information of each iteration.
    pub iterations: Vec<IterationComponent>,
    /// The final solution of the optimization problem.
    pub solution: Array2<f64>,
    /// The number of iterations performed by the solver.
    pub n_iters: usize,
}

/// A trait that implements the FracGM solver for Geman-McClure-based
/// optimization problems with diagnostics.
pub trait GemanMcclureSolverDiagnostic<R: R2>: FractionalProgrammingMaterials<R> {
    /// Updates the diagnostic information of each iteration.
    ///
    /// # Arguments
    ///
    /// - `alpha` - The current solution in vector form.
    /// - `beta` - The current value of the auxiliary variable $\beta$.
    /// - `mu` - The current value of the auxiliary variable $\mu$.
    /// - `terms` - The computed Geman-McClure terms.
    /// - `diagnostics` - The vector to store the diagnostic information of each
    ///   iteration.
    fn update_diagnostics(
        &self,
        alpha: &Array2<f64>,
        beta: &Vec<f64>,
        mu: &Vec<f64>,
        terms: &Vec<Fractional<R>>,
        diagnostics: &mut Vec<IterationComponent>,
    ) {
        let psi_norm = self.compute_psi_norm(&beta, &mu, &terms);

        let component = IterationComponent {
            alpha_vec: alpha.clone(),
            alpha_mat: self.vec_to_mat(&alpha),
            alpha_proj: self.project(&self.vec_to_mat(&alpha)),
            beta: beta.clone(),
            mu: mu.clone(),
            psi_norm,
        };

        diagnostics.push(component);
    }

    /// Solve and diagnose the Geman-McClure-based objective function with the
    /// FracGM solver.
    ///
    /// # Arguments
    ///
    /// - `pc1` - The source point cloud.
    /// - `pc2` - The target point cloud.
    ///
    /// # Returns
    ///
    /// The solution of the optimization problem and the diagnostic information.
    fn solve(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Diagnostic {
        assert!(
            pc1.shape() == pc2.shape(),
            "Input point clouds must have the same shape"
        );
        assert!(
            pc1.shape()[1] == 3,
            "Input point clouds must have 3 columns"
        );

        let mut iterations: Vec<IterationComponent> = Vec::new();

        let mut terms = self.compute_terms(pc1, pc2);

        let init_mat = self.compute_initial_guess(pc1, pc2);
        let mut vec = self.mat_to_vec(&init_mat);
        self.update_terms_cache(&mut terms, &vec);

        let (mut beta, mut mu) = self.solve_beta_mu(&terms);

        self.update_diagnostics(&vec, &beta, &mu, &terms, &mut iterations);

        let mut n_iters: usize = 0;
        for _ in 0..self.max_iteration() {
            n_iters += 1;

            let mut mat_a = Array2::<f64>::zeros((self.dim(), self.dim()));
            for i in 0..pc1.dim().0 {
                let mu_ = &mu[i];
                let beta_ = &beta[i];
                Zip::from(&mut mat_a)
                    .and(terms[i].f_mat())
                    .and(terms[i].h_mat())
                    .for_each(|a, f, h| {
                        *a += mu_ * f - mu_ * beta_ * h;
                    });
            }

            vec = self.solve_x(&mat_a);
            self.update_terms_cache(&mut terms, &vec);

            let psi_norm = self.compute_psi_norm(&beta, &mu, &terms);
            self.update_diagnostics(&vec, &beta, &mu, &terms, &mut iterations);
            if psi_norm < self.tol() {
                break;
            }

            (beta, mu) = self.solve_beta_mu(&terms);
            
        }

        Diagnostic {
            iterations,
            solution: self.project(&self.vec_to_mat(&vec)),
            n_iters,
        }
    }
}
