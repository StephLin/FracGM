/**
 * Copyright 2020, Massachusetts Institute of Technology,
 * Cambridge, MA 02139
 * All Rights Reserved
 * Authors: Jingnan Shi, et al. (see THANKS for the full author list)
 *
 * MIT License
 */

/**
 * Modified by Yu-Kai Lin in 2024 for FracGM project.
 */

#include <vector>

#include "graph.h"
#include "omp.h"
#include "pmc/pmc.h"
#include "utils.h"

Eigen::Matrix<double, 3, Eigen::Dynamic> compute_tims(const Eigen::Matrix<double, 3, Eigen::Dynamic>& v,
                                                      Eigen::Matrix<int, 2, Eigen::Dynamic>* map) {
  auto N = v.cols();
  Eigen::Matrix<double, 3, Eigen::Dynamic> vtilde(3, N * (N - 1) / 2);
  map->resize(2, N * (N - 1) / 2);

#pragma omp parallel for default(none) shared(N, v, vtilde, map)
  for (size_t i = 0; i < N - 1; i++) {
    // Calculate some important indices
    // For each measurement, we compute the TIMs between itself and all the measurements after it.
    // For example:
    // i=0: add N-1 TIMs
    // i=1: add N-2 TIMs
    // etc..
    // i=k: add N-1-k TIMs
    // And by arithmatic series, we can get the starting index of each segment be:
    // k*N - k*(k+1)/2
    size_t segment_start_idx = i * N - i * (i + 1) / 2;
    size_t segment_cols = N - 1 - i;

    // calculate TIM
    Eigen::Matrix<double, 3, 1> m = v.col(i);
    Eigen::Matrix<double, 3, Eigen::Dynamic> temp = v - m * Eigen::MatrixXd::Ones(1, N);

    // concatenate to the end of the tilde vector
    vtilde.middleCols(segment_start_idx, segment_cols) = temp.rightCols(segment_cols);

    // populate the index map
    Eigen::Matrix<int, 2, Eigen::Dynamic> map_addition(2, N);
    for (size_t j = 0; j < N; ++j) {
      map_addition(0, j) = i;
      map_addition(1, j) = j;
    }
    map->middleCols(segment_start_idx, segment_cols) = map_addition.rightCols(segment_cols);
  }

  return vtilde;
}

void scale_inliers_selector(const Eigen::Matrix<double, 3, Eigen::Dynamic>& src,
                            const Eigen::Matrix<double, 3, Eigen::Dynamic>& dst, double noise_bound,
                            Eigen::Matrix<bool, 1, Eigen::Dynamic>* inliers) {
  Eigen::Matrix<double, 1, Eigen::Dynamic> v1_dist = src.array().square().colwise().sum().array().sqrt();
  Eigen::Matrix<double, 1, Eigen::Dynamic> v2_dist = dst.array().square().colwise().sum().array().sqrt();
  double beta = 2 * noise_bound;  // * sqrt(cbar2_)  // Skiped, which is typically 1.0

  // A pair-wise correspondence is an inlier if it passes the following test:
  // abs(|dst| - |src|) is within maximum allowed error
  *inliers = (v1_dist.array() - v2_dist.array()).array().abs() <= beta;
}

std::vector<int> inlier_selection_impl(const Eigen::Matrix<double, 3, Eigen::Dynamic>& src,
                                       const Eigen::Matrix<double, 3, Eigen::Dynamic>& dst, double noise_bound,
                                       double pmc_timeout, int pmc_n_threads) {
  Eigen::Matrix<int, 2, Eigen::Dynamic> src_tims_map_;
  Eigen::Matrix<int, 2, Eigen::Dynamic> dst_tims_map_;

  auto src_tims_ = compute_tims(src, &src_tims_map_);
  auto dst_tims_ = compute_tims(dst, &dst_tims_map_);

  Eigen::Matrix<bool, 1, Eigen::Dynamic> scale_inliers_mask_;

  // TEASER_DEBUG_INFO_MSG("Starting scale solver (only selecting inliers if scale estimation has been disabled).");
  scale_inliers_selector(src_tims_, dst_tims_, noise_bound, &scale_inliers_mask_);
  // TEASER_DEBUG_INFO_MSG("Scale estimation complete.");

  // Calculate Maximum Clique (PMC_EXACT)
  // Note: the max_clique_ vector holds the indices of original measurements that are within the
  // max clique of the built inlier graph.

  // Create inlier graph: A graph with (indices of) original measurements as vertices, and edges
  // only when the TIM between two measurements are inliers. Note: src_tims_map_ is the same as
  // dst_tim_map_
  teaser::Graph inlier_graph_;

  inlier_graph_.populateVertices(src.cols());
  for (size_t i = 0; i < scale_inliers_mask_.cols(); ++i) {
    if (scale_inliers_mask_(0, i)) {
      inlier_graph_.addEdge(src_tims_map_(0, i), src_tims_map_(1, i));
    }
  }

  teaser::MaxCliqueSolver::Params clique_params;

  clique_params.solver_mode = teaser::MaxCliqueSolver::CLIQUE_SOLVER_MODE::PMC_EXACT;
  clique_params.time_limit = pmc_timeout;
  clique_params.num_threads = pmc_n_threads;

  teaser::MaxCliqueSolver clique_solver(clique_params);
  auto max_clique_ = clique_solver.findMaxClique(inlier_graph_);
  std::sort(max_clique_.begin(), max_clique_.end());
  // TEASER_DEBUG_INFO_MSG("Max Clique of scale estimation inliers: ");

  return max_clique_;
}

extern "C" {
CBufferI32 inlier_selection(double* src_array, size_t src_array_len, double* dst_array, size_t dst_array_len,
                            double noise_bound, double pmc_timeout, int pmc_n_threads) {
  auto src = to_eigen_pc(src_array, src_array_len / 3);
  auto dst = to_eigen_pc(dst_array, dst_array_len / 3);

  auto inliers = inlier_selection_impl(src, dst, noise_bound, pmc_timeout, pmc_n_threads);

  return to_c_int_buffer(inliers);
}
}
