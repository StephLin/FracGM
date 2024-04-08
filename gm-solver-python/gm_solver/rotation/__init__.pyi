import typing as T

import numpy.typing as npt

class LinearSolver:
    def __init__(
        self,
        max_iter: int,
        tol: float,
        noise_bound: T.Optional[float],
        c: T.Optional[float],
    ):
        """Initialize the Geman-McClure linear solver for rotation problem.

        Args:
            max_iter (int): Maximum number of iterations.
            tol (float): Tolerance for the stopping criterion.
            noise_bound (T.Optional[float]): Noise bound of the data (default: 0.1).
            c (T.Optional[float]): Gemam McClure parameter (default: 1.0).
        """
        ...

    def solve(self, A: npt.ArrayLike, B: npt.ArrayLike) -> npt.ArrayLike:
        """Solve the rotation problem given two point clouds.

        Args:
            A (npt.ArrayLike): Pointcloud A.
            B (npt.ArrayLike): Pointcloud B.

        Returns:
            npt.ArrayLike: Rotation matrix.
        """
