from pathlib import Path

import numpy as np

from fracgm.registration import LinearRegistrationSolver
from fracgm.rotation import LinearRotationSolver

ROOT = Path(__file__).parent.absolute()

CLOUD_SRC_PATH = ROOT / ".." / "data" / "cloud_src.txt"
CLOUD_DST_PATH = ROOT / ".." / "data" / "cloud_dst.txt"
GT_PATH = ROOT / ".." / "data" / "gt.txt"

FRACGM_C = 1.0
FRACGM_TOL = 1e-6
FRACGM_MAX_ITERATION = 100
FRACGM_NOISE_BOUND = 0.1


def get_rotation_test_data():
    src = np.loadtxt(CLOUD_SRC_PATH)
    dst = np.loadtxt(CLOUD_DST_PATH)
    gt = np.loadtxt(GT_PATH)

    return src, dst, gt


def get_registration_test_data():
    src = np.loadtxt(CLOUD_SRC_PATH)
    dst = np.loadtxt(CLOUD_DST_PATH)

    dst[:, 0] += 0.3
    dst[:, 1] += 0.2
    dst[:, 2] += 0.6

    gt = np.eye(4)
    gt[:3, :3] = np.loadtxt(GT_PATH)
    gt[0, 3] = 0.3
    gt[1, 3] = 0.2
    gt[2, 3] = 0.6

    return src, dst, gt


def main():
    print("[[ Example for FracGM-based rotation solver ]]", end="\n\n")
    src_rot, dst_rot, gt_rot = get_rotation_test_data()

    est_rot = LinearRotationSolver(
        FRACGM_MAX_ITERATION, FRACGM_TOL, FRACGM_NOISE_BOUND, FRACGM_C
    ).solve(src_rot, dst_rot)

    print(f"GT:\n {gt_rot}", end="\n\n")
    print(f"FracGM:\n {est_rot}", end="\n\n")

    print("[[ Example for FracGM-based registration solver ]]", end="\n\n")
    src_reg, dst_reg, gt_reg = get_registration_test_data()

    est_reg = LinearRegistrationSolver(
        FRACGM_MAX_ITERATION, FRACGM_TOL, FRACGM_NOISE_BOUND, FRACGM_C
    ).solve(src_reg, dst_reg)

    print(f"GT:\n {gt_reg}", end="\n\n")
    print(f"FracGM:\n {est_reg}", end="\n\n")

    print("[[ Done ]]")


if __name__ == "__main__":
    main()
