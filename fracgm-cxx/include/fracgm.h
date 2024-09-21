// Copyright 2024 the FracGM authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#include <Eigen/Dense>
#include <iostream>
#include <vector>

#include "fracgm_rust_wrapper.h"

namespace fracgm {

using PointCloud = Eigen::Matrix<double, Eigen::Dynamic, 3, Eigen::RowMajor>;

class LinearRotationSolver {
 public:
  LinearRotationSolver(size_t max_iteration, double tol, double c, double noise_bound)
      : max_iteration_(max_iteration), tol_(tol), c_(c), noise_bound_(noise_bound) {}

  Eigen::Matrix<double, 3, 3, Eigen::RowMajor> solve(const PointCloud &pc1, const PointCloud &pc2) {
    rust::CBufferF64 buf = rust::fracgm_rotation_solver(pc1.data(), pc1.rows(), pc1.cols(), pc2.data(), pc2.rows(),
                                                        pc2.cols(), max_iteration_, tol_, noise_bound_, c_);
    Eigen::Matrix<double, 3, 3, Eigen::RowMajor> result;

    for (int i = 0; i < 3; i++) {
      for (int j = 0; j < 3; j++) {
        result(i, j) = buf.data[i * 3 + j];
      }
    }

    rust::free_f64_buf(buf);
    return result;
  }

 protected:
  size_t max_iteration_;
  double tol_;
  double c_;
  double noise_bound_;
};

class LinearRegistrationSolver {
 public:
  LinearRegistrationSolver(size_t max_iteration, double tol, double c, double noise_bound)
      : max_iteration_(max_iteration), tol_(tol), c_(c), noise_bound_(noise_bound) {}

  Eigen::Matrix<double, 4, 4, Eigen::RowMajor> solve(const PointCloud &pc1, const PointCloud &pc2) {
    rust::CBufferF64 buf = rust::fracgm_registration_solver(pc1.data(), pc1.rows(), pc1.cols(), pc2.data(), pc2.rows(),
                                                            pc2.cols(), max_iteration_, tol_, noise_bound_, c_);
    Eigen::Matrix<double, 4, 4, Eigen::RowMajor> result;

    for (int i = 0; i < 4; i++) {
      for (int j = 0; j < 4; j++) {
        result(i, j) = buf.data[i * 4 + j];
      }
    }

    rust::free_f64_buf(buf);
    return result;
  }

 protected:
  size_t max_iteration_;
  double tol_;
  double c_;
  double noise_bound_;
};

std::vector<size_t> max_clique_inlier_selection(const PointCloud &pc1, const PointCloud &pc2, double noise_bound,
                                                double pmc_timeout) {
  rust::CBufferUSize buf = rust::fracgm_max_clique_inlier_selection(pc1.data(), pc1.rows(), pc1.cols(), pc2.data(),
                                                                    pc2.rows(), pc2.cols(), noise_bound, pmc_timeout);
  std::vector<size_t> indices;

  for (size_t i = 0; i < buf.len; i++) {
    indices.push_back(buf.data[i]);
  }

  rust::free_usize_buf(buf);
  return indices;
}

}  // namespace fracgm