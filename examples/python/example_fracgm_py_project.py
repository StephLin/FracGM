# Copyright 2024 the FracGM authors. All rights reserved.
# Use of this source code is governed by a BSD-style
# license that can be found in the LICENSE file.

from pathlib import Path

import numpy as np

from fracgm.registration import LinearRegistrationSolver, max_clique_inlier_selection
from fracgm.rotation import LinearRotationSolver

ROOT = Path(__file__).parent.absolute()

CLOUD_SRC_PATH = ROOT / ".." / "data" / "cloud_src.txt"
CLOUD_DST_PATH = ROOT / ".." / "data" / "cloud_dst.txt"
GT_PATH = ROOT / ".." / "data" / "gt.txt"

FRACGM_C = 1.0
FRACGM_TOL = 1e-6
FRACGM_MAX_ITERATION = 100
FRACGM_NOISE_BOUND = 0.1
FRACGM_PMC_TIMEOUT = 3600.0

ENABLE_MAX_CLIQUE_INLIER_SELECTION = True


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


def perform_max_clique_inlier_selection(src, dst, noise_bound, pmc_timeout):
    indices = max_clique_inlier_selection(src, dst, noise_bound, pmc_timeout)
    return np.take(src, indices, axis=0), np.take(dst, indices, axis=0)


def main():
    print("[[ Example for FracGM-based rotation solver ]]", end="\n\n")
    src_rot, dst_rot, gt_rot = get_rotation_test_data()

    est_rot = LinearRotationSolver(
        FRACGM_MAX_ITERATION, FRACGM_TOL, FRACGM_NOISE_BOUND, FRACGM_C
    ).solve(src_rot, dst_rot)

    print(f"Ground Truth:\n {gt_rot}", end="\n\n")
    print(f"FracGM:\n {est_rot}", end="\n\n")

    print("[[ Example for FracGM-based registration solver ]]", end="\n\n")
    src_reg, dst_reg, gt_reg = get_registration_test_data()

    if ENABLE_MAX_CLIQUE_INLIER_SELECTION:
        src_reg, dst_reg = perform_max_clique_inlier_selection(
            src_reg, dst_reg, FRACGM_NOISE_BOUND, FRACGM_PMC_TIMEOUT
        )

    est_reg = LinearRegistrationSolver(
        FRACGM_MAX_ITERATION, FRACGM_TOL, FRACGM_NOISE_BOUND, FRACGM_C
    ).solve(src_reg, dst_reg)

    print(f"Ground Truth:\n {gt_reg}", end="\n\n")
    print(f"FracGM:\n {est_reg}", end="\n\n")

    print("[[ Done ]]")


if __name__ == "__main__":
    main()
