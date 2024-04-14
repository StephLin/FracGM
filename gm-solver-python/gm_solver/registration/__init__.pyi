import typing as T

import numpy.typing as npt

from gm_solver import Diagnostic

class LinearSolver:
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
