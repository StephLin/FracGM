#include <Eigen/Dense>
#include <iostream>

#include "fracgm_rust_wrapper.h"

namespace fracgm {

using PointCloud = Eigen::Matrix<double, Eigen::Dynamic, 3, Eigen::RowMajor>;

class LinearRotationSolver {
 public:
  LinearRotationSolver(size_t max_iteration, double tol, double c, double noise_bound)
      : max_iteration_(max_iteration), tol_(tol), c_(c), noise_bound_(noise_bound) {}

  Eigen::Matrix<double, 3, 3, Eigen::RowMajor> solve(const PointCloud &pc1, const PointCloud &pc2) {
    rust::CBuffer buf = rust::fracgm_rotation_solver(pc1.data(), pc1.rows(), pc1.cols(), pc2.data(), pc2.rows(),
                                                     pc2.cols(), max_iteration_, tol_, noise_bound_, c_);
    Eigen::Matrix<double, 3, 3, Eigen::RowMajor> result;

    for (int i = 0; i < 3; i++) {
      for (int j = 0; j < 3; j++) {
        result(i, j) = buf.data[i * 3 + j];
      }
    }

    rust::free_buf(buf);
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
    rust::CBuffer buf = rust::fracgm_registration_solver(pc1.data(), pc1.rows(), pc1.cols(), pc2.data(), pc2.rows(),
                                                         pc2.cols(), max_iteration_, tol_, noise_bound_, c_);
    Eigen::Matrix<double, 4, 4, Eigen::RowMajor> result;

    for (int i = 0; i < 4; i++) {
      for (int j = 0; j < 4; j++) {
        result(i, j) = buf.data[i * 4 + j];
      }
    }

    rust::free_buf(buf);
    return result;
  }

 protected:
  size_t max_iteration_;
  double tol_;
  double c_;
  double noise_bound_;
};

}  // namespace fracgm