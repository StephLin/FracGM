# Copyright 2024 the FracGM authors. All rights reserved.
# Use of this source code is governed by a BSD-style
# license that can be found in the LICENSE file.

import typing as T
from enum import Enum

import numpy.typing as npt

from fracgm import Diagnostic

class LinearRegistrationSolver:
    def __init__(
        self,
        max_iter: int,
        tol: float,
        noise_bound: T.Optional[float],
        c: T.Optional[float],
    ):
        """Initialize the Geman-McClure linear solver for registration problem.

        Args:
            max_iter (int): Maximum number of iterations.
            tol (float): Tolerance for the stopping criterion.
            noise_bound (T.Optional[float]): Noise bound of the data (default: 0.1).
            c (T.Optional[float]): Gemam McClure parameter (default: 1.0).
        """
        ...

    def solve(self, A: npt.ArrayLike, B: npt.ArrayLike) -> npt.ArrayLike:
        """Solve the registration problem given two point clouds.

        Args:
            A (npt.ArrayLike): Pointcloud A.
            B (npt.ArrayLike): Pointcloud B.

        Returns:
            npt.ArrayLike: SE(3) matrix.
        """

    def solve_with_diagnostic(self, A: npt.ArrayLike, B: npt.ArrayLike) -> Diagnostic:
        """Solve the registration problem given two point clouds and return diagnostic.

        Args:
            A (npt.ArrayLike): Pointcloud A.
            B (npt.ArrayLike): Pointcloud B.

        Returns:
            Diagnostic: Solution with diagnostic.
        """
        ...

class TIMPolicy(Enum):
    COMPLETE = 0
    CHAIN = 1

class DecoupledRegistrationSolver:
    def __init__(
        self,
        max_iter: int,
        tol: float,
        noise_bound: T.Optional[float],
        c: T.Optional[float],
    ):
        """Initialize the Geman-McClure linear solver for registration problem.

        Args:
            max_iter (int): Maximum number of iterations.
            tol (float): Tolerance for the stopping criterion.
            noise_bound (T.Optional[float]): Noise bound of the data (default: 0.1).
            c (T.Optional[float]): Gemam McClure parameter (default: 1.0).
        """
        ...

    def set_tim_policy(self, tim_policy: TIMPolicy):
        """Set TIM (translation invariant measurement) computation policy.

        Args:
            tim_policy (TIMPolicy): TIM computation policy.
        """
        ...

    def solve(self, A: npt.ArrayLike, B: npt.ArrayLike) -> npt.ArrayLike:
        """Solve the registration problem given two point clouds.

        Args:
            A (npt.ArrayLike): Pointcloud A.
            B (npt.ArrayLike): Pointcloud B.

        Returns:
            npt.ArrayLike: SE(3) matrix.
        """

def max_clique_inlier_selection(
    A: npt.ArrayLike, B: npt.ArrayLike, noise_bound: float, pmc_timeout: float
) -> npt.ArrayLike:
    """
    Find inlier correspondences using the Maximum clique inlier selection (MCIS) algorithm.

    Args:
        A (npt.ArrayLike): Pointcloud A.
        B (npt.ArrayLike): Pointcloud B.
        noise_bound (float): Noise bound of the data.
        pmc_timeout (float): Timeout for the PMC solver in seconds.

    Returns:
        npt.ArrayLike: SE(3) matrix.
    """
    ...
